// structs = models (event shape)

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub tenant_id: String,
    pub event_type: String,
}
