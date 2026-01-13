use anyhow::{Result, Context};
use std::path::PathBuf;

/// Auto-configure Factory Droid settings.json
pub async fn auto_configure(settings_path: &PathBuf) -> Result<()> {
    // Read existing settings
    let content = std::fs::read_to_string(settings_path)
        .context("Failed to read Factory settings")?;
    
    let mut settings: serde_json::Value = serde_json::from_str(&content)
        .context("Failed to parse Factory settings")?;
    
    // Get current API key
    let config = crate::config::load_config()?;
    let api_key = &config.proxy.api_key;
    let base_url = format!("http://127.0.0.1:{}", config.proxy.port);
    
    // Generate models config
    let models = generate_models_array(api_key, &base_url)?;
    
    // Merge with existing customModels
    if let Some(existing_models) = settings.get_mut("customModels") {
        if let Some(existing_array) = existing_models.as_array_mut() {
            // Merge: add our models, rename conflicts
            for model in models.as_array().unwrap() {
                let model_id = model["id"].as_str().unwrap();
                
                // Check if model with same id exists
                let exists = existing_array.iter().any(|m| {
                    m["id"].as_str() == Some(model_id)
                });
                
                if exists {
                    // Rename our model
                    let mut model_copy = model.clone();
                    let original_name = model_copy["displayName"].as_str().unwrap();
                    model_copy["displayName"] = serde_json::json!(format!("{} [drovity]", original_name));
                    model_copy["id"] = serde_json::json!(format!("{}-drovity", model_id));
                    existing_array.push(model_copy);
                } else {
                    existing_array.push(model.clone());
                }
            }
        }
    } else {
        // No existing customModels, create new
        settings["customModels"] = models;
    }
    
    // Write back
    let updated_content = serde_json::to_string_pretty(&settings)?;
    std::fs::write(settings_path, updated_content)
        .context("Failed to write Factory settings")?;
    
    Ok(())
}

/// Generate JSON config for manual setup
pub fn generate_config_json() -> Result<String> {
    let config = crate::config::load_config()?;
    let api_key = &config.proxy.api_key;
    let base_url = format!("http://127.0.0.1:{}", config.proxy.port);
    
    let models = generate_models_array(api_key, &base_url)?;
    let json = serde_json::to_string_pretty(&models)?;
    
    Ok(json)
}

fn generate_models_array(api_key: &str, base_url: &str) -> Result<serde_json::Value> {
    Ok(serde_json::json!([
        {
            "model": "gemini-3-flash",
            "id": "gemini-3-flash-drovity",
            "index": 100,
            "baseUrl": format!("{}/", base_url),
            "apiKey": api_key,
            "displayName": "Gemini 3 Flash [drovity]",
            "maxOutputTokens": 24576,
            "noImageSupport": false,
            "provider": "anthropic"
        },
        {
            "model": "gemini-3-pro-high",
            "id": "gemini-3-pro-high-drovity",
            "index": 101,
            "baseUrl": format!("{}/", base_url),
            "apiKey": api_key,
            "displayName": "Gemini 3 Pro High [drovity]",
            "maxOutputTokens": 32768,
            "noImageSupport": false,
            "provider": "anthropic"
        },
        {
            "model": "gemini-2.5-flash",
            "id": "gemini-2-5-flash-drovity",
            "index": 102,
            "baseUrl": format!("{}/", base_url),
            "apiKey": api_key,
            "displayName": "Gemini 2.5 Flash [drovity]",
            "maxOutputTokens": 24576,
            "noImageSupport": false,
            "provider": "anthropic"
        },
        {
            "model": "gemini-2.5-pro",
            "id": "gemini-2-5-pro-drovity",
            "index": 103,
            "baseUrl": format!("{}/", base_url),
            "apiKey": api_key,
            "displayName": "Gemini 2.5 Pro [drovity]",
            "maxOutputTokens": 32768,
            "noImageSupport": false,
            "provider": "anthropic"
        },
        {
            "model": "claude-sonnet-4-5",
            "id": "claude-sonnet-4-5-drovity",
            "index": 104,
            "baseUrl": base_url,
            "apiKey": api_key,
            "displayName": "Claude 4.5 Sonnet [drovity]",
            "maxOutputTokens": 8192,
            "noImageSupport": false,
            "provider": "anthropic"
        }
    ]))
}
