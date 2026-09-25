use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ConnectInfo, Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::{TypedHeader, headers::UserAgent};
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::sync::{
    RwLock,
    broadcast::{self, error::RecvError},
    mpsc,
};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    app_state::{AppState, Room},
    messages::{ClientMessage, ClientMessageError, ServerMessage},
};

pub async fn request(
    wsu: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    cookies: Cookies,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(room_uuid): Path<Uuid>,
) -> Result<Response, Response> {
    // Client identity
    let ad = addr.to_string();
    let ua = match user_agent {
        Some(TypedHeader(ua)) => ua.to_string(),
        None => String::from("None"),
    };

    log::trace!("Room socket request {room_uuid} {ad} {ua}");

    // Token
    let error = (
        StatusCode::UNAUTHORIZED,
        "Must authenticate before connection",
    )
        .into_response();

    let (room_uuid, member_uuid) = cookies
        // Get the cookie
        .get(&room_uuid.to_string())
        // Verify it and return (room_uuid, member_uuid)
        .and_then(|cookie| state.verify_token(cookie.value()))
        // room_uuid must match
        .filter(|token| token.0 == room_uuid)
        .ok_or(error)?;

    log::debug!("Cookies of addr {ad} room {room_uuid} member {member_uuid}");

    Ok(wsu.on_upgrade(async move |ws| handle_socket(ws, state, room_uuid, member_uuid).await))
}

async fn handle_socket(ws: WebSocket, state: AppState, room_uuid: Uuid, member_uuid: Uuid) {
    // Room
    let room = state.rooms().get(&room_uuid).await.unwrap();
    // .ok_or((StatusCode::BAD_REQUEST, "Room does not exist").into_response())?;

    // Client socket
    let (client_tx, client_rx) = ws.split();

    // Broadcast channel
    let broadcast_tx = room.read().await.broadcast_tx().clone();
    let broadcast_rx = broadcast_tx.subscribe();

    // Reply channel
    let (reply_tx, reply_rx) = mpsc::channel::<ServerMessage>(1);

    tokio::select! {
        _ = tokio::spawn(receive_loop(
            client_rx,
            broadcast_tx,
            reply_tx,
            room,
            member_uuid,
        )) => {},
        _ = tokio::spawn(response_loop(
            client_tx,
            broadcast_rx,
            reply_rx,
            member_uuid,
        )) => {},
    }

    // Try remove room
    if let Err(err) = state.rooms().cleanup(&room_uuid).await {
        log::info!("Cleanup room: {err}");
    } else {
        log::info!("Cleanup room: OK");
    }
}

async fn receive_loop(
    mut client_rx: SplitStream<WebSocket>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
    reply_tx: mpsc::Sender<ServerMessage>,
    room: Arc<RwLock<Room>>,
    member_uuid: Uuid,
) {
    // Set online state
    room.write()
        .await
        .update_member(&member_uuid, true)
        .await
        .unwrap();
    let member = room.read().await.view_member(&member_uuid).await.unwrap();

    // Reply identity
    let value = ServerMessage::SyncIdentity {
        member: member.clone(),
    };
    reply_tx.send(value).await.unwrap();

    // Send member list
    let members = room.read().await.list_members().await;
    let message = ServerMessage::SyncMembers { members };
    reply_tx.send(message).await.unwrap();

    // Broadcast member online
    let value = ServerMessage::MemberState { member };
    broadcast_tx.send(value).unwrap();

    loop {
        let socket_message = match client_rx.next().await {
            Some(Ok(socket_message)) => socket_message,
            Some(Err(_)) => break,
            None => break,
        };

        let client_message: ClientMessage = match socket_message.try_into() {
            Ok(client_message) => client_message,
            Err(ClientMessageError::WrongMessage(reason)) => {
                let message = ServerMessage::BadMessage { reason };
                reply_tx.send(message).await.unwrap();
                continue;
            }
            Err(ClientMessageError::IgnoreMessage) => continue,
        };

        match client_message {
            ClientMessage::SyncMembers => {
                let members = room.read().await.list_members().await;
                let message = ServerMessage::SyncMembers { members };
                reply_tx.send(message).await.unwrap();
            }

            ClientMessage::PublicMessage { message } => {
                let message = ServerMessage::PublicMessage {
                    sender: member_uuid,
                    message: message,
                };
                broadcast_tx.send(message).unwrap();
            }

            ClientMessage::PrivateMessage { receiver, message } => {
                let message = ServerMessage::PrivateMessage {
                    sender: member_uuid,
                    receiver,
                    message,
                };
                broadcast_tx.send(message).unwrap();
            }
        };
    }

    // Set offline state
    room.write()
        .await
        .update_member(&member_uuid, false)
        .await
        .unwrap();
    let member = room.read().await.view_member(&member_uuid).await.unwrap();

    // Broadcast member offline
    let value = ServerMessage::MemberState { member };
    broadcast_tx.send(value).unwrap();
}

async fn response_loop(
    mut client_tx: SplitSink<WebSocket, Message>,
    mut broadcast_rx: broadcast::Receiver<ServerMessage>,
    mut reply_rx: mpsc::Receiver<ServerMessage>,
    member_uuid: Uuid,
) {
    loop {
        let chat_message = tokio::select! {
            broadcast = broadcast_rx.recv() => match broadcast {
                Ok(message) => message,
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => break,
            },

            reply = reply_rx.recv() => match reply {
                Some(message) => message,
                None => break,
            },
        };

        // Private message (check sender and receiver)
        if let ServerMessage::PrivateMessage {
            sender, receiver, ..
        } = &chat_message
            && sender != &member_uuid
            && receiver != &member_uuid
        {
            continue;
        }

        // Public message
        if client_tx.send(chat_message.into()).await.is_err() {
            break;
        }
    }
}
