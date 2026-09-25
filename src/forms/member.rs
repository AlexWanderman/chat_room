use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberView {
    pub uuid: Uuid,
    pub name: String,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
}
