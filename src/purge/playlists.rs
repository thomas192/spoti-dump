use anyhow::Result;
use serde_json::Value;

use crate::utils;

pub async fn purge_playlists(access_token: &str) -> Result<()> {
    println!("Purging playlists...");

    let playlists: Vec<Value> = utils::get_all_items(access_token, "https://api.spotify.com/v1/me/playlists").await?;

    for playlist in playlists {
        let playlist_id = playlist["id"].as_str().unwrap();
        let url = format!("https://api.spotify.com/v1/playlists/{}/followers", playlist_id);
        utils::delete_spotify(access_token, &url).await?;
        println!("Unfollowed playlist: {}", playlist["name"].as_str().unwrap());
    }

    println!("Playlists purged successfully.");
    Ok(())
}
