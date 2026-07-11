use axum::{routing::post, Router};
// use sqlx::PgPool;

use crate::handlers::tenants::create_tenant;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/tenants", post(create_tenant))
}
