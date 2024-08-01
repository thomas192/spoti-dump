# spoti-dump

Export your Spotify saved tracks and playlists to csv.

## Why

While Spotify is a reliable service, having a local backup of your music library is always a good practice. Never lose your music collection, carefully curated over the years. 

## Usage

### Windows

1. Download the latest .exe from [Releases](https://github.com/thomas192/spoti-dump/releases).
2. Open PowerShell in the download folder.
3. Run: `./spoti-dump-x86_64-pc-windows-msvc.exe`
4. Follow authentication prompts.
5. Find exports in the `dump` folder.

### Linux

1. Install Rust: [rustup.rs](https://rustup.rs/)
2. Clone and run:
   ```
   git clone https://github.com/thomas192/spoti-dump.git
   cd spoti-dump
   cargo run --release
   ```
3. Follow authentication prompts.
4. Find exports in the `dump` folder.
