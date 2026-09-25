use chrono::{DateTime, Utc};

// Chat member
#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
}
