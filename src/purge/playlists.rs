use anyhow::Result;
use serde_json::Value;

use crate::utils;

pub async fn purge_playlists(access_token: &str, force: bool) -> Result<Vec<String>> {
    println!("Purging playlists...");

    let playlists: Vec<Value> = utils::get_all_items(access_token, "https://api.spotify.com/v1/me/playlists").await?;
    let playlist_names: Vec<String> = playlists
        .iter()
        .map(|p| p["name"].as_str().unwrap().to_string())
        .collect();

    if force {
        for playlist in &playlists {
            let playlist_id = playlist["id"].as_str().unwrap();
            let url = format!("https://api.spotify.com/v1/playlists/{}/followers", playlist_id);
            utils::delete_spotify(access_token, &url).await?;
            println!("Unfollowed playlist: {}", playlist["name"].as_str().unwrap());
        }
        println!("Playlists purged successfully.");
        Ok(Vec::new())
    } else {
        println!("Found {} playlists to unfollow.", playlists.len());
        for name in &playlist_names {
            println!("- {}", name);
        }
        println!("Playlists purge dry run complete.");
        Ok(playlist_names)
    }
}
