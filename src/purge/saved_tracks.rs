use anyhow::Result;
use serde_json::Value;

use crate::utils;

pub async fn purge_saved_tracks(access_token: &str, force: bool) -> Result<Vec<String>> {
    println!("Purging saved tracks...");

    let tracks: Vec<Value> = utils::get_all_items(access_token, "https://api.spotify.com/v1/me/tracks").await?;
    let track_ids: Vec<String> = tracks
        .into_iter()
        .map(|t| t["track"]["id"].as_str().unwrap().to_string())
        .collect();

    if force {
        for chunk in track_ids.chunks(50) {
            let url = format!("https://api.spotify.com/v1/me/tracks?ids={}", chunk.join(","));
            utils::delete_spotify(access_token, &url).await?;
            println!("Purged a chunk of saved tracks.");
        }
        println!("Saved tracks purged successfully.");
        Ok(Vec::new())
    } else {
        println!("Found {} saved tracks to purge.", track_ids.len());
        println!("Saved tracks purge dry run complete.");
        Ok(track_ids)
    }
}
