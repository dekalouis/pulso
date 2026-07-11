// the actual logic (like controllers)

use axum::{extract::State, Extension, Json};
use axum::http::StatusCode;
// use sqlx::PgPool;
use crate::middleware::auth::Tenant;
use crate::models::event::{EventInput, Event};
use crate::state::AppState;
use crate::services::metrics;

// pub async fn create_event(Json(payload): Json<Event>) -> &'static str {
//     println!("Received event: {:?}", payload);
//     "Event received"
// }

pub async fn create_event(
    // State(pool): State<PgPool>,
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
    Json(payload): Json<EventInput>,
// ) -> &'static str {
) -> Result<&'static str, StatusCode> {
    sqlx::query("INSERT INTO events (tenant_id, event_type) VALUES ($1, $2)")
        .bind(&tenant.tenant_id)
        .bind(&payload.event_type)
        .execute(&state.pool)
        .await
        //.expect("Failed to insert event")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut redis = state.redis.clone();
    let tid = tenant.tenant_id.clone();
    let etype = payload.event_type.clone();
    tokio::spawn(async move {
        if let Err(e) = metrics::record_event(&mut redis, &tid, &etype).await {
            eprintln!("Failed to record metric: {}", e);
        }
    });
    // "Event stored"
    Ok("Event stored")
}


pub async fn events(
    // State(pool): State<PgPool>,
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
) -> Result<Json<Vec<Event>>, StatusCode> {
    let events = sqlx::query_as::<_, Event>(
        "SELECT id, tenant_id, event_type, created_at FROM events WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 100"
        )
        .bind(&tenant.tenant_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}

pub async fn health_check() -> &'static str {
    "OK"
}
