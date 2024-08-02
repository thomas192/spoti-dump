# spoti-dump

Export your Spotify saved tracks and playlists to csv.

## Why

While Spotify is a reliable service, having a local backup of your music library is always a good practice. Never lose your music collection, carefully curated over the years. 

## Usage

### Prerequisites

1. **Spotify Developer Account**
   - Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
   - Create a new app

2. **App Configuration**
   - In your app's settings, locate and copy the Client ID and Client Secret

3. **Environment Setup**
   - Copy the `.env.example` file and rename it to `.env`
   - Open the `.env` file and paste your Client ID and Client Secret
   - Ensure the `.env` file is in the same directory as the executable

4. **User Management (for Import Feature)**
   - In the Developer Dashboard, navigate to User Management
   - Add the email associated with the Spotify account you want to import tracks and playlists to

### Installation

#### Windows

1. Download the latest `.exe` file from the [Releases](https://github.com/thomas192/spoti-dump/releases) page
2. Open PowerShell in the download folder
3. Run the executable:
   ```
   ./spoti-dump-x86_64-pc-windows-msvc.exe export
   ```

#### Linux

1. Install Rust:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone the repository and run:
   ```
   git clone https://github.com/thomas192/spoti-dump.git
   cd spoti-dump
   cargo run --release -- export
   ```
<<<<<<< HEAD

### Commands

spoti-dump supports two main commands: `export` and `import`.

#### Export Command

The `export` command allows you to backup your Spotify saved tracks and playlists.

Usage:
```
spoti-dump export
```

This will create CSV files of your saved tracks and playlists in a folder named `dump` in the same directory as the executable.

#### Import Command

The `import` command allows you to import saved tracks and playlists into your Spotify account.

Usage:
```
spoti-dump import
```

**Important Notes for Import:**
- The CSV files to be imported must be in the same format as those created by the `export` command.
- Place the CSV files you want to import in a folder named `dump` in the same directory as the executable.
- Ensure that the Spotify account email you added in the User Management step of the Prerequisites section matches the account you're importing to.
=======
3. Follow authentication prompts.
4. Find exports in the `dump` folder.
>>>>>>> e1e6ebaf1ad8723dadfd9a9d9f9fe2f5973ee14b
