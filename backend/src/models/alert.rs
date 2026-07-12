
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AlertRule {
    pub id: uuid::Uuid, 
    pub tenant_id: String, 
    pub event_type: String,
    pub rule_condition: String,
    pub threshold: i32,
    pub time_window: String, 
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AlertEvent {
    pub id: uuid::Uuid,
    pub rule_id: uuid::Uuid,
    pub tenant_id: String,
    pub event_type: String,
    pub rule_condition: String,
    pub threshold: i32,
    pub value_at_trigger: i32,
    pub triggered_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlertRuleInput {
    pub event_type: String,
    pub rule_condition: String,
    pub threshold: i32,
    pub time_window: String,
}
