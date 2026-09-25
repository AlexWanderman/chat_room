use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomView {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub has_password: bool,
    pub online: usize,
    pub total: usize,
}
