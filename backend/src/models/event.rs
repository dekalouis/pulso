// structs = models (event shape)

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventInput {
    pub event_type: String,
}
