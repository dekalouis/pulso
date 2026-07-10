// the actual logic (like controllers)

use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::models::event::Event;

// pub async fn create_event(Json(payload): Json<Event>) -> &'static str {
//     println!("Received event: {:?}", payload);
//     "Event received"
// }

pub async fn create_event(
    State(pool): State<PgPool>,
    Json(payload): Json<Event>,
) -> &'static str {
    sqlx::query("INSERT INTO events (tenant_id, event_type) VALUES ($1, $2)")
        .bind(&payload.tenant_id)
        .bind(&payload.event_type)
        .execute(&pool)
        .await
        .expect("Failed to insert event");

    "Event stored"
}

pub async fn health_check() -> &'static str {
    "OK"
}
