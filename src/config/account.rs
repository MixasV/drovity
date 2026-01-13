use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub token: TokenData,
    pub disabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_timestamp: i64,
    pub token_type: String,
}

impl TokenData {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        let expiry_timestamp = Utc::now().timestamp() + expires_in;
        Self {
            access_token,
            refresh_token,
            expiry_timestamp,
            token_type: "Bearer".to_string(),
        }
    }
}

pub fn get_accounts_dir() -> Result<PathBuf> {
    let config_dir = super::get_config_dir()?;
    let accounts_dir = config_dir.join("accounts");
    
    if !accounts_dir.exists() {
        std::fs::create_dir_all(&accounts_dir)?;
    }
    
    Ok(accounts_dir)
}

pub fn list_accounts() -> Result<Vec<Account>> {
    let accounts_dir = get_accounts_dir()?;
    let mut accounts = Vec::new();
    
    for entry in std::fs::read_dir(accounts_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = std::fs::read_to_string(&path)?;
            let account: Account = serde_json::from_str(&content)?;
            accounts.push(account);
        }
    }
    
    // Sort by created_at
    accounts.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    
    Ok(accounts)
}

pub fn save_account(account: &Account) -> Result<()> {
    let accounts_dir = get_accounts_dir()?;
    let account_path = accounts_dir.join(format!("{}.json", account.id));
    let content = serde_json::to_string_pretty(account)?;
    std::fs::write(&account_path, content)?;
    Ok(())
}

pub fn create_account(email: String, display_name: Option<String>, token: TokenData) -> Result<Account> {
    let now = Utc::now().timestamp();
    let account = Account {
        id: Uuid::new_v4().to_string(),
        email,
        display_name,
        token,
        disabled: false,
        created_at: now,
        updated_at: now,
    };
    
    save_account(&account)?;
    Ok(account)
}

pub fn delete_account(account_id: &str) -> Result<()> {
    let accounts_dir = get_accounts_dir()?;
    let account_path = accounts_dir.join(format!("{}.json", account_id));
    
    if account_path.exists() {
        std::fs::remove_file(&account_path)?;
    }
    
    Ok(())
}
