use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub port: u16,
    pub api_key: String,
    pub auto_start: bool,
    pub allow_lan_access: bool,
}
