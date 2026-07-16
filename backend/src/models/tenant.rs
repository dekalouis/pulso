use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTenantInput {
    pub tenant_name: String,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub tenant_id: String,
    pub api_key: String,
    pub tenant_name: String,
}

#[derive(Debug, Serialize)]
pub struct TenantInfo {
    pub tenant_id: String,
    pub tenant_name: String,
}
