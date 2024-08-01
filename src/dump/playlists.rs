use anyhow::{Context, Result};
use csv::Writer;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::types::Track;

#[derive(Debug, Deserialize)]
struct PlaylistsResponse {
    items: Vec<Playlist>,
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Playlist {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct PlaylistTracksResponse {
    items: Vec<Track>,
    next: Option<String>,
}

pub async fn dump_playlists(access_token: &String) -> Result<()> {
    let dump_dir = Path::new("dump");
    if !dump_dir.exists() {
        fs::create_dir(dump_dir).context("Failed to create dump directory")?;
    }

    let client = reqwest::Client::new();
    let mut url = "https://api.spotify.com/v1/me/playlists?limit=50".to_string();

    loop {
        let playlists = fetch_playlists(&client, &access_token, &url).await?;

        for playlist in &playlists.items {
            dump_playlist(&client, &access_token, &playlist.id, &playlist.name, &dump_dir).await?;
        }

        match playlists.next {
            Some(next_url) => url = next_url,
            None => break,
        }
    }

    println!("All playlists have been exported.");
    Ok(())
}

async fn fetch_playlists(
    client: &reqwest::Client,
    access_token: &str,
    url: &str,
) -> Result<PlaylistsResponse> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let response = client.get(url).headers(headers).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("API request failed: {:?}", response));
    }

    let playlists_response: PlaylistsResponse = response.json().await?;
    Ok(playlists_response)
}

async fn dump_playlist(
    client: &reqwest::Client,
    access_token: &str,
    playlist_id: &str,
    playlist_name: &str,
    dump_dir: &Path,
) -> Result<()> {
    let sanitized_name = sanitize_filename(playlist_name);
    let output_file = dump_dir.join(format!("{}.csv", sanitized_name));
    let mut writer = Writer::from_path(&output_file)
        .with_context(|| format!("Failed to create CSV file: {:?}", output_file))?;

    writer.write_record(&["Added At", "Track Name", "Artists", "Album"])?;

    let mut url = format!(
        "https://api.spotify.com/v1/playlists/{}/tracks?limit=50",
        playlist_id
    );

    loop {
        let tracks = fetch_playlist_tracks(client, access_token, &url).await?;

        for track in &tracks.items {
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

        match tracks.next {
            Some(next_url) => url = next_url,
            None => break,
        }
    }

    writer.flush()?;
    println!(
        "Playlist '{}' has been exported to {}",
        playlist_name, output_file.to_str().unwrap()
    );

    Ok(())
}

async fn fetch_playlist_tracks(
    client: &reqwest::Client,
    access_token: &str,
    url: &str,
) -> Result<PlaylistTracksResponse> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let response = client.get(url).headers(headers).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("API request failed: {:?}", response));
    }

    let tracks_response: PlaylistTracksResponse = response.json().await?;
    Ok(tracks_response)
}

fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c)
            } else if c == ' ' {
                Some('_')
            } else {
                None
            }
        })
        .collect();

    if sanitized.is_empty() {
        generate_random_name()
    } else {
        sanitized
    }
}

fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let random_name: String = (0..10)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    random_name
}
