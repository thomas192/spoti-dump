use anyhow::{Context, Result};
use csv::Writer;
use rand::Rng;
use std::fs;
use std::path::Path;

use crate::types::Track;
use crate::utils;

#[derive(Debug, serde::Deserialize)]
struct Playlist {
    id: String,
    name: String,
}

#[derive(Debug, serde::Deserialize)]
struct PlaylistItem {
    added_at: Option<String>,
    track: Option<Track>,
}

pub async fn export_playlists(access_token: &String) -> Result<()> {
    let dump_dir = Path::new("dump");
    if !dump_dir.exists() {
        fs::create_dir(dump_dir).context("Failed to create dump directory")?;
    }

    let playlists: Vec<Playlist> = utils::get_all_items(access_token, "https://api.spotify.com/v1/me/playlists").await?;
    let mut total_skipped_tracks = 0;

    for playlist in playlists {
        total_skipped_tracks += export_playlist(access_token, &playlist.id, &playlist.name, &dump_dir).await?;
    }

    println!("All playlists have been exported.");
    if total_skipped_tracks > 0 {
        println!("Skipped {} tracks in playlists.", total_skipped_tracks);
    }
    Ok(())
}

async fn export_playlist(
    access_token: &str,
    playlist_id: &str,
    playlist_name: &str,
    dump_dir: &Path,
) -> Result<u32> {
    let sanitized_name = sanitize_filename(playlist_name);
    let output_file = dump_dir.join(format!("{}.csv", sanitized_name));
    let mut writer = Writer::from_path(&output_file)
        .with_context(|| format!("Failed to create CSV file: {:?}", output_file))?;

    writer.write_record(&["Added At", "Track Name", "Artists", "Album", "Id"])?;

    let url = format!(
        "https://api.spotify.com/v1/playlists/{}/tracks",
        playlist_id
    );
    let tracks: Vec<PlaylistItem> = utils::get_all_items(access_token, &url).await?;
    let mut skipped_tracks_count = 0;

    for item in tracks {
        if let Some(track_data) = item.track {
            let artists = track_data
                .artists
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            writer.write_record(&[
                &item.added_at.unwrap_or("Unknown".to_string()),
                &track_data.name,
                &artists,
                &track_data.album.name,
                &track_data.id,
            ])?;
        } else {
            skipped_tracks_count += 1;
        }
    }

    writer.flush()?;
    println!(
        "Playlist '{}' has been exported to {}",
        playlist_name,
        output_file.to_str().unwrap()
    );

    Ok(skipped_tracks_count)
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
