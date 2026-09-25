use std::collections::HashMap;

use anyhow::{Result, anyhow};
use chrono::Utc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{
    forms::{MemberView, RoomView},
    messages::ServerMessage,
};

use super::{member::Member, validation};

// Chat room
#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    description: String,
    is_listed: bool,
    password: Option<String>,
    members: HashMap<Uuid, Member>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
}

impl Room {
    pub fn new(
        name: String,
        description: String,
        is_listed: bool,
        password: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            is_listed,
            password,
            members: Default::default(),
            broadcast_tx: broadcast::channel::<ServerMessage>(16).0,
        }
    }

    pub fn as_view(&self, uuid: &Uuid) -> Option<RoomView> {
        self.is_listed.then(|| RoomView {
            uuid: uuid.to_owned(),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            has_password: self.password.is_some(),
            online: self.members.iter().filter(|(_, v)| v.is_online).count(),
            total: self.members.len(),
        })
    }

    pub fn broadcast_tx(&self) -> &broadcast::Sender<ServerMessage> {
        &self.broadcast_tx
    }

    pub async fn add_member(
        &mut self,
        member_name: String,
        room_password: Option<String>,
    ) -> Result<Uuid> {
        // Validate parameters
        validation::validate_name(&member_name)?;

        // Verify password
        if self.password != room_password {
            return Err(anyhow!("Wrong password"));
        }

        // Create member
        let uuid = Uuid::new_v4();
        let member = Member {
            name: member_name,
            is_online: false,
            last_seen: Utc::now(),
        };

        // Add new member
        self.members.insert(uuid, member);

        Ok(uuid)
    }

    pub async fn view_member(&self, member_uuid: &Uuid) -> Option<MemberView> {
        self.members.get(&member_uuid).map(|v| MemberView {
            uuid: member_uuid.to_owned(),
            name: v.name.to_owned(),
            is_online: v.is_online,
            last_seen: v.last_seen,
        })
    }

    pub async fn list_members(&self) -> Vec<MemberView> {
        self.members
            .iter()
            .map(|(k, v)| MemberView {
                uuid: k.to_owned(),
                name: v.name.to_owned(),
                is_online: v.is_online,
                last_seen: v.last_seen,
            })
            .collect()
    }

    pub async fn all_offline(&self) -> bool {
        !self.members.iter().any(|(_, v)| v.is_online)
    }

    pub async fn update_member(&mut self, member_uuid: &Uuid, is_online: bool) -> Result<()> {
        if let Some(member) = self.members.get_mut(&member_uuid) {
            member.is_online = is_online;
            member.last_seen = Utc::now();
            Ok(())
        } else {
            Err(anyhow!("Member not found"))
        }
    }
}
