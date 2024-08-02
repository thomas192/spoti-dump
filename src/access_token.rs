use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::env;
use tiny_http::{Response, Server};
use url::Url;

use crate::Commands;

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u32,
    refresh_token: String,
}

pub async fn get_access_token(command: Commands) -> Result<String> {
    dotenv::dotenv().context("Failed to load .env file")?;

    let client_id = env::var("SPOTIFY_CLIENT_ID").context("SPOTIFY_CLIENT_ID not set")?;
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").context("SPOTIFY_CLIENT_SECRET not set")?;
    let redirect_uri = "http://localhost:8888/callback";

    // Step 1: Get the authorization code
    let (code, _) = get_authorization_code(command, &client_id, redirect_uri)?;

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
        ("redirect_uri", redirect_uri),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let response: TokenResponse = client
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

fn get_authorization_code(
    command: Commands,
    client_id: &str,
    redirect_uri: &str,
) -> Result<(String, String)> {
    // Generate a random state string
    let state: String = {
        let random_bytes: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
        general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes)
    };

    let scope = match command {
        Commands::Export => "user-library-read playlist-read-private",
        Commands::Import => "user-library-modify playlist-modify-public playlist-modify-private",
    };

    let auth_url = Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("client_id", client_id),
            ("response_type", "code"),
            ("redirect_uri", redirect_uri),
            ("state", &state),
            ("scope", scope),
        ],
    )?;

    // Open the authorization URL in the user's default browser
    open::that(auth_url.as_str())?;

    // Start a local server to handle the callback
    let server = Server::http("127.0.0.1:8888").unwrap();
    println!("Waiting for Spotify authorization...");

    for request in server.incoming_requests() {
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
            return Err(anyhow::anyhow!("State mismatch"));
        }

        // Send a response to the browser
        let response =
            Response::from_string("Authorization successful! You can close this window now.");
        request.respond(response)?;

        return Ok((code, state.to_string()));
    }

    Err(anyhow::anyhow!("Failed to get authorization code"))
}
