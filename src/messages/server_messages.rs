use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::forms::MemberView;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    SyncIdentity {
        member: MemberView,
    },
    SyncMembers {
        members: Vec<MemberView>,
    },
    MemberState {
        member: MemberView,
    },
    PublicMessage {
        sender: Uuid,
        message: String,
    },
    PrivateMessage {
        sender: Uuid,
        receiver: Uuid,
        message: String,
    },
    BadMessage {
        reason: String,
    },
}

impl Into<Message> for ServerMessage {
    fn into(self) -> Message {
        let message_json = serde_json::to_string(&self).unwrap();

        Message::Text(message_json.into())
    }
}
