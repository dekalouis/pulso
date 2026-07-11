// structs = models (event shape)

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventInput {
    pub event_type: String,
}


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Event {
    pub id: uuid::Uuid,
    pub tenant_id: String,
    pub event_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


