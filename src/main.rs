use anyhow::Result;

mod access_token;
mod dump;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = access_token::get_access_token().await?;
    println!("Access token retrieved: {access_token}");

    dump::saved_tracks::dump_saved_tracks(&access_token).await?;
    dump::playlists::dump_playlists(&access_token).await?;

    Ok(())
}
