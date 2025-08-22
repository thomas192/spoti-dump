use anyhow::{Context, Result};
use csv::Writer;
use std::fs;
use std::path::Path;

use crate::types::Track;
use crate::utils;

#[derive(Debug, serde::Deserialize)]
struct SavedTrack {
    added_at: Option<String>,
    track: Option<Track>,
}

pub async fn export_saved_tracks(access_token: &String, force: bool) -> Result<()> {
    let tracks: Vec<SavedTrack> = utils::get_all_items(access_token, "https://api.spotify.com/v1/me/tracks").await?;

    if !force {
        println!("Dry run: would have exported {} saved tracks.", tracks.len());
        return Ok(());
    }

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

    writer.write_record(&["Added At", "Track Name", "Artists", "Album", "Id"])?;

    let mut skipped_tracks_count = 0;

    for track in tracks {
        if let Some(track_data) = track.track {
            let artists = track_data
                .artists
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            writer.write_record(&[
                &track.added_at.unwrap_or("Unknown".to_string()),
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
        "Saved tracks have been exported to {}",
        output_file.to_str().unwrap()
    );

    if skipped_tracks_count > 0 {
        println!("Skipped {} saved tracks.", skipped_tracks_count);
    }

    Ok(())
}
