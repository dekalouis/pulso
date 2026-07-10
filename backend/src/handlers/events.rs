// the actual logic (like controllers)

use axum::Json;
use crate::models::event::Event;

pub async fn create_event(Json(payload): Json<Event>) -> &'static str {
    println!("Received event: {:?}", payload);
    "Event received"
}

pub async fn health_check() -> &'static str {
    "OK"
}
