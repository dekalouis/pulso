use axum::{routing::post, Router};
use axum::middleware as axum_middleware;
use sqlx::PgPool;

use crate::handlers::events::{create_event, events};
use crate::middleware::auth::require_api_key;

pub fn routes(pool: PgPool) -> Router<PgPool> {
    Router::new()
        .route("/events", post(create_event).get(events))
        .route_layer(axum_middleware::from_fn_with_state(pool, require_api_key))
}
