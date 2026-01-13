use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub port: u16,
    pub api_key: String,
    pub allow_lan_access: bool,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            port: 8045,
            api_key: String::new(),
            allow_lan_access: true,
        }
    }
}

impl ProxyConfig {
    pub fn get_bind_address(&self) -> &str {
        if self.allow_lan_access {
            "0.0.0.0"
        } else {
            "127.0.0.1"
        }
    }
}
