pub mod account;
pub mod proxy;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub proxy: ProxyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub port: u16,
    pub api_key: String,
    pub auto_start: bool,
    pub allow_lan_access: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxy: ProxyConfig {
                port: 8045,
                api_key: generate_api_key(),
                auto_start: true,
                allow_lan_access: true,
            },
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_dir = home.join(".drovity");
    
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }
    
    Ok(config_dir)
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_dir()?.join("config.json");
    
    if !config_path.exists() {
        // Create default config
        let config = Config::default();
        save_config(&config)?;
        return Ok(config);
    }
    
    let content = std::fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_dir()?.join("config.json");
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(&config_path, content)?;
    Ok(())
}

fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    
    let key: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    format!("sk-{}", key)
}

pub fn regenerate_api_key() -> Result<String> {
    let mut config = load_config()?;
    config.proxy.api_key = generate_api_key();
    save_config(&config)?;
    Ok(config.proxy.api_key)
}
