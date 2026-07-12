use axum::{routing::{delete, get, post}, Router};
use axum::middleware as axum_middleware;

use crate::state::AppState;
use crate::handlers::alerts::{create_alert_rule, delete_alert_rule, list_alert_rules, list_alert_events};
use crate::middleware::auth::require_api_key;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/alert-rules", post(create_alert_rule).get(list_alert_rules))
        .route("/alert-rules/{id}", delete(delete_alert_rule))
        .route("/alerts", get(list_alert_events))
        .route_layer(axum_middleware::from_fn_with_state(state, require_api_key))
}
