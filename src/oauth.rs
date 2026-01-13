use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

// Google OAuth constants
const CLIENT_ID: &str = "1071006060591-tmhssin2h21lcre235vtolojh4g403ep.apps.googleusercontent.com";
const CLIENT_SECRET: &str = "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";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const REDIRECT_URI: &str = "http://localhost:8087/oauth/callback";

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

/// Generate OAuth authorization URL and get authorization code manually
pub async fn authorize_with_manual_callback() -> Result<crate::config::account::Account> {
    // Step 1: Generate authorization URL
    let auth_url = generate_auth_url()?;
    
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                 🔑 Google Authorization                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    println!("📱 {} Open this URL:\n", console::style("Step 1:").yellow().bold());
    println!("   {}\n", console::style(&auth_url).cyan().underlined());
    
    println!("🔐 {} Authorize with your Google account\n", console::style("Step 2:").yellow().bold());
    
    println!("📋 {} After authorization, Google will try to redirect to:\n", console::style("Step 3:").yellow().bold());
    println!("   {}", console::style("http://localhost:8087/oauth/callback?code=...").dim());
    println!();
    println!("   The page won't load (that's OK!), but the URL will contain a CODE.");
    println!("   Look for {}\n", console::style("?code=").green().bold());
    
    println!("✂️  {} Copy EVERYTHING after '?code=' from the URL\n", console::style("Step 4:").yellow().bold());
    
    // Step 2: Get authorization code from user
    let code = dialoguer::Input::<String>::new()
        .with_prompt("Paste the authorization code here")
        .interact_text()?;
    
    let code = code.trim().to_string();
    
    if code.is_empty() {
        anyhow::bail!("Authorization code cannot be empty");
    }
    
    println!();
    println!("{}", console::style("⏳ Exchanging code for tokens...").yellow());
    
    // Step 3: Exchange code for tokens
    exchange_code_for_tokens(&code).await
}

/// Generate OAuth authorization URL
fn generate_auth_url() -> Result<String> {
    let scopes = vec![
        "https://www.googleapis.com/auth/cloud-platform",
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/userinfo.profile",
        "https://www.googleapis.com/auth/cclog",
        "https://www.googleapis.com/auth/experimentsandconfigs"
    ].join(" ");

    let params = vec![
        ("client_id", CLIENT_ID),
        ("redirect_uri", REDIRECT_URI),
        ("response_type", "code"),
        ("scope", &scopes),
        ("access_type", "offline"),
        ("prompt", "consent"),
    ];
    
    let url = url::Url::parse_with_params(AUTH_URL, &params)
        .context("Failed to generate OAuth URL")?;
    
    Ok(url.to_string())
}

/// Exchange authorization code for tokens
async fn exchange_code_for_tokens(code: &str) -> Result<crate::config::account::Account> {
    let client = reqwest::Client::new();
    
    let params = [
        ("client_id", CLIENT_ID),
        ("client_secret", CLIENT_SECRET),
        ("code", code),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post(TOKEN_URL)
        .form(&params)
        .send()
        .await
        .context("Failed to exchange code for tokens")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("Token exchange failed: {}", error_text);
    }

    let token_res: TokenResponse = response
        .json()
        .await
        .context("Failed to parse token response")?;
    
    let refresh_token = token_res.refresh_token
        .ok_or_else(|| anyhow::anyhow!("No refresh token received"))?;

    println!("{}", console::style("✅ Authorization successful!\n").green().bold());

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

    Ok(account)
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
