use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::handlers::tenants::create_tenant;

pub fn routes() -> Router<PgPool> {
    Router::new().route("/tenants", post(create_tenant))
}
