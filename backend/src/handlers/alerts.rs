use axum::{extract::{Path, State}, Extension, Json};
use axum::http::StatusCode;

use crate::state::AppState;
use crate::middleware::auth::Tenant;
use crate::models::alert::{AlertRule, AlertEvent, CreateAlertRuleInput};

// All handlers:
// - `State(state): State<AppState>`
// - `Extension(tenant): Extension<Tenant>`
// - Return `Result<Json<...>, StatusCode>`
// - Map DB errors to `StatusCode::INTERNAL_SERVER_ERROR`

// **`create_alert_rule`** — validates input (condition must be "above"/"below", 
// window must be "1m"/"5m"/"1hr"), inserts into `alert_rules`, returns the created rule.
pub async fn create_alert_rule(
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
    Json(payload): Json<CreateAlertRuleInput>,
) -> Result<Json<AlertRule>, StatusCode> {
    if payload.rule_condition != "above" && payload.rule_condition != "below" {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !["1m", "5m", "1hr"].contains(&payload.time_window.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let rule = sqlx::query_as::<_, AlertRule>(
        "INSERT INTO alert_rules (tenant_id, event_type, rule_condition, threshold, time_window)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *",
    )
    .bind(&tenant.tenant_id)
    .bind(&payload.event_type)
    .bind(&payload.rule_condition)
    .bind(payload.threshold)
    .bind(&payload.time_window)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rule))
}

// **`list_alert_rules`** — queries `alert_rules WHERE tenant_id = $1 AND 
// is_active = TRUE ORDER BY created_at DESC`.
pub async fn list_alert_rules(
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
) -> Result<Json<Vec<AlertRule>>, StatusCode> {
    let rules = sqlx::query_as::<_, AlertRule>(
        "SELECT * FROM alert_rules WHERE tenant_id = $1 AND is_active = TRUE ORDER BY created_at DESC",    
    )
    .bind(&tenant.tenant_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rules))
}

// **`delete_alert_rule`** — takes rule `id` as a path param, 
// verifies the rule belongs to the tenant (security — never delete by id alone), 
// then deletes.
pub async fn delete_alert_rule(
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "UPDATE alert_rules SET is_active = FALSE WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(&tenant.tenant_id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

// **`list_alert_events`** — queries `alert_events WHERE tenant_id = $1 ORDER BY 
// triggered_at DESC LIMIT 100`.
pub async fn list_alert_events( 
    State(state): State<AppState>,
    Extension(tenant): Extension<Tenant>,
) -> Result<Json<Vec<AlertEvent>>, StatusCode> {
    let events = sqlx::query_as::<_, AlertEvent>(
        "SELECT * FROM alert_events WHERE tenant_id = $1 ORDER BY triggered_at DESC LIMIT 100",
        )
        .bind(&tenant.tenant_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(events))
}
