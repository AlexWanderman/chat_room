use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: Uuid,
    pub sub: Uuid,
    pub exp: usize,
}
