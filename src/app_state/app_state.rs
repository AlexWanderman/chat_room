use axum::response::Html;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use tera::{Context, Tera};
use tower_cookies::{
    Cookie,
    cookie::{SameSite, time},
};
use uuid::Uuid;

use super::{claims::Claims, room_manager::RoomManager};

#[derive(Debug, Clone)]
pub struct AppState {
    templates: Tera,
    rooms: RoomManager,
    secret: [u8; 128],
}

impl AppState {
    pub fn new() -> Self {
        let mut templates = Tera::new();
        templates
            .add_template_files([
                (format!("templates/index.html"), Some("index")),
                (format!("templates/about.html"), Some("about")),
                (format!("templates/room.html"), Some("room")),
                (format!("templates/no_room.html"), Some("no_room")),
            ])
            .unwrap();

        Self {
            templates,
            rooms: RoomManager::new(),
            secret: rand::random(),
        }
    }

    pub fn render_page(&self, template_name: &str, context: &Context) -> Html<String> {
        Html(self.templates.render(template_name, &context).unwrap())
    }

    pub fn rooms(&self) -> RoomManager {
        self.rooms.clone()
    }

    pub fn create_token(&self, room_uuid: &Uuid, member_uuid: &Uuid) -> String {
        let exp = chrono::Utc::now()
            .checked_add_signed(Duration::hours(1)) // Token lifetime
            .unwrap()
            .timestamp();

        let claims = Claims {
            iss: room_uuid.to_owned(),
            sub: member_uuid.to_owned(),
            exp: exp as usize,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .unwrap()
    }

    pub fn verify_token(&self, token: &str) -> Option<(Uuid, Uuid)> {
        jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .ok()
        .map(|data| (data.claims.iss, data.claims.sub))
    }

    pub fn produce_cookie<'a>(name: String, token: String) -> Cookie<'a> {
        let mut exp = time::OffsetDateTime::now_utc();
        exp += time::Duration::hours(1); // Cookie lifetime

        let mut cookie = Cookie::new(name, token);
        cookie.set_expires(exp);
        cookie.set_http_only(true);
        // cookie.set_secure(true);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);

        cookie
    }
}
