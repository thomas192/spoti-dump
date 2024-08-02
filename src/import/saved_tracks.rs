use anyhow::{Context, Result};
use csv::Reader;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::path::Path;

pub async fn import_saved_tracks(access_token: &str) -> Result<()> {
    let dump_dir = Path::new("dump");
    let input_file = dump_dir.join("saved_tracks.csv");

    let mut reader = Reader::from_path(&input_file)
        .with_context(|| format!("Failed to open CSV file: {}", input_file.to_str().unwrap()))?;

    let client = reqwest::Client::new();
    let mut track_ids = Vec::new();

    for result in reader.records() {
        let record = result?;
        if let Some(track_id) = record.get(4) {
            track_ids.push(track_id.to_string());

            if track_ids.len() == 50 {
                save_tracks(&client, access_token, &track_ids).await?;
                track_ids.clear();
            }
        }
    }

    if !track_ids.is_empty() {
        save_tracks(&client, access_token, &track_ids).await?;
    }

    println!("All saved tracks have been imported.");
    Ok(())
}

async fn save_tracks(
    client: &reqwest::Client,
    access_token: &str,
    track_ids: &[String],
) -> Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let ids = track_ids.join(",");
    let url = format!("https://api.spotify.com/v1/me/tracks?ids={}", ids);

    let response = client
        .put(&url)
        .headers(headers)
        .json(&json!({"ids": track_ids}))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to save tracks: {:?}", response));
    }

    println!("Saved {} tracks", track_ids.len());
    Ok(())
}
