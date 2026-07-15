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

    let mut windows: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for (event_type, window) in counts {
        let mut w = HashMap::new();
        w.insert("five_min".to_string(), window.five_min);
        w.insert("fifteen_min".to_string(), window.fifteen_min);
        w.insert("one_hour".to_string(), window.one_hour);
        w.insert("one_day".to_string(), window.one_day);
        windows.insert(event_type, w);
    }

    Ok(Json(MetricsResponse {
        tenant_id: tenant.tenant_id,
        windows,
    }))
}



