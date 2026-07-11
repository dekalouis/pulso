use axum::{routing::{get, post}, Router};
use axum::middleware as axum_middleware;
// use sqlx::PgPool;
use crate::state::AppState;
use crate::handlers::events::{create_event, events};
use crate::middleware::auth::require_api_key;
use crate::handlers::metrics::get_metrics;

// pub fn routes(pool: PgPool) -> Router<PgPool> {
pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/events", post(create_event).get(events))
        .route("/metrics", get(get_metrics))
        .route_layer(axum_middleware::from_fn_with_state(state, require_api_key))
}
