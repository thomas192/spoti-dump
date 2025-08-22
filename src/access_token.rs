use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::{env, time::Duration};
use tiny_http::{Response, Server};
use url::Url;

use crate::Commands;

const REDIRECT_URI: &str = "http://localhost:8888/callback";

// Scopes for Spotify API
const SCOPE_EXPORT: &str = "user-library-read playlist-read-private";
const SCOPE_IMPORT: &str = "user-library-modify playlist-modify-public playlist-modify-private";
const SCOPE_PURGE: &str = "user-library-read user-library-modify playlist-read-private playlist-modify-public playlist-modify-private";

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
}

pub async fn get_access_token(command: Commands) -> Result<String> {
    dotenv::dotenv().context("Failed to load .env file")?;

    if let Ok(refresh_token) = env::var("SPOTIFY_REFRESH_TOKEN") {
        return get_access_token_from_refresh_token(&refresh_token).await;
    }

    let client_id = env::var("SPOTIFY_CLIENT_ID").context("SPOTIFY_CLIENT_ID not set")?;
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").context("SPOTIFY_CLIENT_SECRET not set")?;

    // Step 1: Get the authorization code
    let (code, _) = get_authorization_code(command, &client_id)?;

    // Step 2: Exchange the code for an access token
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let response: AccessTokenResponse = client
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send()
        .await
        .context("Failed to send request")?
        .json()
        .await
        .context("Failed to parse response")?;

    if let Some(refresh_token) = response.refresh_token {
        println!("Your refresh token is: {}", refresh_token);
        println!("Please set it as SPOTIFY_REFRESH_TOKEN in your .env file.");
    }

    Ok(response.access_token)
}

pub async fn get_access_token_from_refresh_token(refresh_token: &str) -> Result<String> {
    dotenv::dotenv().context("Failed to load .env file")?;

    let client_id = env::var("SPOTIFY_CLIENT_ID").context("SPOTIFY_CLIENT_ID not set")?;
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").context("SPOTIFY_CLIENT_SECRET not set")?;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let response: AccessTokenResponse = client
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send()
        .await
        .context("Failed to send request")?
        .json()
        .await
        .context("Failed to parse response")?;

    Ok(response.access_token)
}

fn get_authorization_code(command: Commands, client_id: &str) -> Result<(String, String)> {
    // Generate a random state string
    let state: String = {
        let random_bytes: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
        general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes)
    };

    let scope = match command {
        Commands::Export { .. } => SCOPE_EXPORT,
        Commands::Import { .. } => SCOPE_IMPORT,
        Commands::Purge { .. } => SCOPE_PURGE,
    };

    let auth_url = Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("client_id", client_id),
            ("response_type", "code"),
            ("redirect_uri", REDIRECT_URI),
            ("state", &state),
            ("scope", scope),
        ],
    )?;

    // Open the authorization URL in the user's default browser
    open::that(auth_url.as_str())?;

    // Start a local server to handle the callback
    let server = Server::http("127.0.0.1:8888").unwrap();
    println!("Waiting for Spotify authorization... (will time out in 2 minutes)");

    // Wait for the callback with a timeout
    if let Ok(Some(request)) = server.recv_timeout(Duration::from_secs(120)) {
            let url = Url::parse(&format!("http://localhost{}", request.url()))?;
            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.into_owned())
                .context("No code found in callback URL")?;

            let received_state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, value)| value.into_owned())
                .context("No state found in callback URL")?;

            if received_state != state {
                return Err(anyhow::anyhow!("State mismatch: CSRF check failed."));
            }

            // Send a response to the browser
            let response =
                Response::from_string("Authorization successful! You can close this window now.");
            request.respond(response)?;

            return Ok((code, state.to_string()));
    }
    Err(anyhow::anyhow!("Authorization timed out. Please try again."))
}
