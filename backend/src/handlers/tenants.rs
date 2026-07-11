use axum::{extract::State, http::StatusCode, Json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::tenant::{ApiKeyResponse, CreateTenantInput};

pub async fn create_tenant(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTenantInput>,
) -> Result<Json<ApiKeyResponse>, StatusCode> {
    let tenant_id = format!("tenant-{}", Uuid::new_v4());
    let raw_key = Uuid::new_v4().to_string();
    let tenant_name = payload.tenant_name;

    let mut hasher = Sha256::new();
    hasher.update(raw_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    sqlx::query("INSERT INTO api_keys (key_hash, tenant_id, tenant_name) VALUES ($1, $2, $3)")
        .bind(&key_hash)
        .bind(&tenant_id)
        .bind(&tenant_name)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiKeyResponse { tenant_id, api_key: raw_key, tenant_name }))
}
