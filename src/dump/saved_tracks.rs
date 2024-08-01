use anyhow::{Context, Result};
use csv::Writer;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::types::Track;

#[derive(Debug, Deserialize)]
struct SpotifyResponse {
    items: Vec<Track>,
    next: Option<String>,
}

pub async fn dump_saved_tracks(access_token: &String) -> Result<()> {
    let dump_dir = Path::new("dump");
    if !dump_dir.exists() {
        fs::create_dir(dump_dir).context("Failed to create dump directory")?;
    }

    let output_file = dump_dir.join("saved_tracks.csv");
    let mut writer = Writer::from_path(&output_file).with_context(|| {
        format!(
            "Failed to create CSV file: {}",
            output_file.to_str().unwrap()
        )
    })?;

    writer.write_record(&["Added At", "Track Name", "Artists", "Album"])?;

    let client = reqwest::Client::new();
    let mut url = "https://api.spotify.com/v1/me/tracks?limit=50&offset=0".to_string();

    loop {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))?,
        );

        let response = client.get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("API request failed: {:?}", response));
        }

        let spotify_response: SpotifyResponse = response.json().await?;

        for track in spotify_response.items {
            let artists = track
                .track
                .artists
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            writer.write_record(&[
                &track.added_at,
                &track.track.name,
                &artists,
                &track.track.album.name,
            ])?;
        }

        match spotify_response.next {
            Some(next_url) => url = next_url,
            None => break,
        }
    }

    writer.flush()?;
    println!(
        "Saved tracks have been exported to {}",
        output_file.to_str().unwrap()
    );

    Ok(())
}
