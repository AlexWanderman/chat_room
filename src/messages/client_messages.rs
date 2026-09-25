use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::validation::validate_message;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    SyncMembers,
    PublicMessage { message: String },
    PrivateMessage { receiver: Uuid, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessageError {
    WrongMessage(String),
    IgnoreMessage,
}

impl TryFrom<Message> for ClientMessage {
    type Error = ClientMessageError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let string_json = match value {
            Message::Text(string_json) => string_json.to_string(),
            _ => return Err(Self::Error::IgnoreMessage),
        };

        let client_message = match serde_json::from_str::<ClientMessage>(&string_json) {
            Ok(client_message) => client_message,
            Err(err) => return Err(Self::Error::WrongMessage(err.to_string())),
        };

        match client_message {
            ClientMessage::PublicMessage { ref message }
            | ClientMessage::PrivateMessage { ref message, .. } => {
                if let Err(err) = validate_message(message) {
                    return Err(Self::Error::WrongMessage(err.to_string()));
                }
            }
            _ => (),
        }

        Ok(client_message)
    }
}
