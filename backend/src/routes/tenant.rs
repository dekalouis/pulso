use axum::{middleware as axum_middleware, routing::{get, post}, Router};
// use sqlx::PgPool;

use crate::handlers::tenants::{create_tenant, get_current_tenant};
use crate::middleware::auth::require_api_key;
use crate::state::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    let public = Router::new().route("/tenants", post(create_tenant));

    let authed = Router::new()
        .route("/tenant", get(get_current_tenant))
        .route_layer(axum_middleware::from_fn_with_state(state, require_api_key));

    public.merge(authed)
}
