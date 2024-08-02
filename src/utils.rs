use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;

pub async fn get_user_id(access_token: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let response = client
        .get("https://api.spotify.com/v1/me")
        .headers(headers)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to get user profile"));
    }

    let user_profile: Value = response.json().await?;
    user_profile["id"]
        .as_str()
        .context("User ID not found in profile")
        .map(String::from)
}
