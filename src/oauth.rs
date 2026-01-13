use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

// Google OAuth constants
const CLIENT_ID: &str = "1071006060591-tmhssin2h21lcre235vtolojh4g403ep.apps.googleusercontent.com";
const CLIENT_SECRET: &str = "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";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const DEVICE_CODE_URL: &str = "https://oauth2.googleapis.com/device/code";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

impl UserInfo {
    pub fn get_display_name(&self) -> Option<String> {
        if let Some(name) = &self.name {
            if !name.trim().is_empty() {
                return Some(name.clone());
            }
        }
        
        match (&self.given_name, &self.family_name) {
            (Some(given), Some(family)) => Some(format!("{} {}", given, family)),
            (Some(given), None) => Some(given.clone()),
            (None, Some(family)) => Some(family.clone()),
            (None, None) => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_url: String,
    expires_in: i64,
    interval: i64,
}

/// Start Device Authorization Flow and get tokens
pub async fn authorize_device() -> Result<crate::config::account::Account> {
    let client = reqwest::Client::new();
    
    // Step 1: Request device code
    let scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform",
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/userinfo.profile",
        "https://www.googleapis.com/auth/cclog",
        "https://www.googleapis.com/auth/experimentsandconfigs"
    ].join(" ");

    let params = [
        ("client_id", CLIENT_ID),
        ("scope", &scopes),
    ];

    println!("🔐 Requesting authorization code...");
    
    let response = client
        .post(DEVICE_CODE_URL)
        .form(&params)
        .send()
        .await
        .context("Failed to request device code")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("Device code request failed: {}", error_text);
    }

    let device_response: DeviceCodeResponse = response
        .json()
        .await
        .context("Failed to parse device code response")?;

    // Step 2: Display instructions to user
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                 🔑 Google Authorization                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    println!("📱 Open this URL on ANY device (phone/computer):\n");
    println!("   \x1b[1;36m{}\x1b[0m\n", device_response.verification_url);
    println!("🔢 Enter this code when prompted:\n");
    println!("   \x1b[1;32m{}\x1b[0m\n", device_response.user_code);
    println!("⏱️  Code expires in {} seconds", device_response.expires_in);
    println!("\n⏳ Waiting for authorization...\n");

    // Step 3: Poll for token
    let poll_params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("device_code", device_response.device_code.as_str()),
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
    ];

    let poll_interval = tokio::time::Duration::from_secs(device_response.interval.max(5) as u64);
    let timeout = tokio::time::Duration::from_secs(device_response.expires_in as u64);
    let start_time = tokio::time::Instant::now();

    loop {
        if start_time.elapsed() > timeout {
            anyhow::bail!("Authorization timeout - code expired");
        }

        tokio::time::sleep(poll_interval).await;

        let response = client
            .post(TOKEN_URL)
            .form(&poll_params)
            .send()
            .await
            .context("Failed to poll for token")?;

        if response.status().is_success() {
            let token_res: TokenResponse = response
                .json()
                .await
                .context("Failed to parse token response")?;
            
            let refresh_token = token_res.refresh_token
                .ok_or_else(|| anyhow::anyhow!("No refresh token received"))?;

            println!("✅ Authorization successful!\n");

            // Get user info
            let user_info = get_user_info(&token_res.access_token).await?;
            
            println!("👤 Authorized as: {}", user_info.email);
            if let Some(name) = user_info.get_display_name() {
                println!("📝 Name: {}", name);
            }

            // Create token data
            let token = crate::config::account::TokenData::new(
                token_res.access_token,
                refresh_token,
                token_res.expires_in,
            );

            // Create and save account
            let account = crate::config::account::create_account(
                user_info.email.clone(),
                user_info.get_display_name(),
                token,
            )?;

            return Ok(account);
        }

        // Handle errors
        let error_text = response.text().await.unwrap_or_default();
        
        if error_text.contains("authorization_pending") {
            // Still waiting, continue polling
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).ok();
            continue;
        } else if error_text.contains("slow_down") {
            // Google asking us to slow down
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            continue;
        } else if error_text.contains("access_denied") {
            anyhow::bail!("Authorization denied by user");
        } else if error_text.contains("expired_token") {
            anyhow::bail!("Authorization code expired");
        } else {
            anyhow::bail!("Token request failed: {}", error_text);
        }
    }
}

/// Get user info from access token
async fn get_user_info(access_token: &str) -> Result<UserInfo> {
    let client = reqwest::Client::new();
    
    let response = client
        .get(USERINFO_URL)
        .bearer_auth(access_token)
        .send()
        .await
        .context("Failed to get user info")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("Get user info failed: {}", error_text);
    }

    let user_info: UserInfo = response
        .json()
        .await
        .context("Failed to parse user info")?;

    Ok(user_info)
}

/// Refresh access token using refresh token
pub async fn refresh_access_token(refresh_token: &str) -> Result<TokenResponse> {
    let client = reqwest::Client::new();
    
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("refresh_token", refresh_token),
        ("grant_type", "refresh_token"),
    ];

    let response = client
        .post(TOKEN_URL)
        .form(&params)
        .send()
        .await
        .context("Failed to refresh token")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("Token refresh failed: {}", error_text);
    }

    let token_data: TokenResponse = response
        .json()
        .await
        .context("Failed to parse refresh response")?;

    Ok(token_data)
}
