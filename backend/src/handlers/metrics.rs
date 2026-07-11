use axum::{extract::State, Extension, Json};
use axum::http::StatusCode;
use std::collections::HashMap;
use serde::Serialize;

use crate::middleware::auth::Tenant;
use crate::services::metrics::get_counts;
use crate::state::AppState;

#[derive(Serialize)]
pub struct MetricsResponse {
    pub tenant_id: String,
    pub windows: HashMap<String, HashMap<String, i64>>,
}

pub async fn get_metrics(
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
) -> Result<Json<MetricsResponse>, StatusCode> {
    let mut redis = state.redis.clone();

    let counts = get_counts(&mut redis, &tenant.tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut one_min: HashMap<String, i64> = HashMap::new();
    for (event_type, window) in counts {
        one_min.insert(event_type, window.one_min);
    }

    let mut windows = HashMap::new();
    windows.insert("1m".to_string(), one_min);

    Ok(Json(MetricsResponse {
        tenant_id: tenant.tenant_id,
        windows,
    }))
}



