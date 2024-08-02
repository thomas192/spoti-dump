use anyhow::Result;
use clap::{Parser, Subcommand};

mod access_token;
mod export;
mod import;
mod types;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Export,
    Import,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Export) => {
            let access_token = access_token::get_access_token(Commands::Export).await?;

            println!("Exporting tracks and playlists...");
            export::saved_tracks::export_saved_tracks(&access_token).await?;
            export::playlists::export_playlists(&access_token).await?;
            println!("Export completed successfully.");
        }
        Some(Commands::Import) => {
            let access_token = access_token::get_access_token(Commands::Import).await?;

            let user_id = utils::get_user_id(&access_token).await?;
            println!("Retrieved user ID: {}", user_id);

            println!("Importing tracks and playlists...");
            import::saved_tracks::import_saved_tracks(&access_token).await?;
            import::playlists::import_playlists(&access_token, &user_id).await?;
            println!("Import completed successfully.");
        }
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}
