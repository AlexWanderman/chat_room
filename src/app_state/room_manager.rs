use std::{collections::HashMap, ops::Deref, sync::Arc};

use anyhow::{Result, anyhow};
use futures::{StreamExt, stream};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::forms::RoomView;

use super::{room::Room, validation};

// Shared map
type SharedMap<K, V> = Arc<RwLock<HashMap<K, Arc<RwLock<V>>>>>;

// Room manager
#[derive(Debug, Clone)]
pub struct RoomManager(SharedMap<Uuid, Room>);

// Access the map
impl Deref for RoomManager {
    type Target = SharedMap<Uuid, Room>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Keeps shared map of rooms and implements operations on rooms
impl RoomManager {
    pub fn new() -> Self {
        Self(Default::default())
    }

    // Create new room
    pub async fn create(
        &self,
        name: String,
        description: String,
        is_listed: bool,
        password: Option<String>,
    ) -> Result<Uuid> {
        // Validate parameters
        validation::validate_name(&name)?;
        validation::validate_description(&description)?;

        if let Some(password) = &password {
            validation::validate_password(password)?;
        }

        // Create room
        let uuid = Uuid::new_v4();
        let room = Room::new(name, description, is_listed, password);

        // Write room
        self.write().await.insert(uuid, Arc::new(RwLock::new(room)));

        Ok(uuid)
    }

    // Get room by uuid
    pub async fn get(&self, room_uuid: &Uuid) -> Option<Arc<RwLock<Room>>> {
        self.read().await.get(room_uuid).cloned()
    }

    // View list of rooms
    pub async fn list(&self) -> Vec<RoomView> {
        let rooms = self.read().await;

        stream::iter(rooms.iter())
            .filter_map(|(uuid, room_arc)| async move {
                let room = room_arc.read().await;
                room.as_view(uuid)
            })
            .collect()
            .await
    }

    // Remove the room if all members are offline
    pub async fn cleanup(&self, room_uuid: &Uuid) -> Result<()> {
        self.read()
            .await
            .get(&room_uuid)
            .ok_or(anyhow!("Room does not exist"))?
            .read()
            .await
            .all_offline()
            .await
            .ok_or(anyhow!("Room must be empty"))?;

        self.write().await.remove(room_uuid);

        Ok(())
    }
}
