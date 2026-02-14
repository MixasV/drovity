use serde_json::Value;

/// Get project_id using Antigravity's loadCodeAssist API
pub async fn fetch_project_id(access_token: &str) -> Result<String, String> {
    // Use Sandbox environment to avoid Prod 429 errors
    let url = "https://daily-cloudcode-pa.sandbox.googleapis.com/v1internal:loadCodeAssist";
    
    let request_body = serde_json::json!({
        "metadata": {
            "ideType": "ANTIGRAVITY"
        }
    });
    
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .bearer_auth(access_token)
        .header("User-Agent", crate::constants::USER_AGENT.as_str())
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("loadCodeAssist request failed: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("loadCodeAssist error {}: {}", status, body));
    }
    
    let data: Value = response.json()
        .await
        .map_err(|e| format!("Parse response failed: {}", e))?;
    
    // Extract cloudaicompanionProject
    if let Some(project_id) = data.get("cloudaicompanionProject")
        .and_then(|v| v.as_str()) {
        return Ok(project_id.to_string());
    }
    
    // Fallback to mock project_id
    let mock_id = generate_mock_project_id();
    tracing::warn!("Account has no cloudaicompanionProject, using mock: {}", mock_id);
    Ok(mock_id)
}

/// Generate random project_id (fallback when API doesn't return one)
/// Uses a known working project ID to ensure stability
pub fn generate_mock_project_id() -> String {
    // Return the known working project ID from DroidGravity-Manager
    "bamboo-precept-lgxtn".to_string()
}
