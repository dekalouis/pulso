use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sha2::{Digest, Sha256};
// use sqlx::PgPool;

use crate::state::AppState;

#[derive(Clone)]
pub struct Tenant {
    pub tenant_id: String,
    pub tenant_name: String,
}

pub async fn require_api_key(
    // State(pool): State<PgPool>,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT tenant_id, tenant_name FROM api_keys WHERE key_hash = $1",
    )
    .bind(&key_hash)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (tenant_id, tenant_name) = row.ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(Tenant { tenant_id, tenant_name });

    Ok(next.run(req).await)
}
