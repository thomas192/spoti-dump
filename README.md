# spoti-dump

A command-line tool to export, import, and purge your Spotify saved tracks and playlists.

While Spotify is a reliable service, having a local backup of your music library is always a good practice. Don't lose your music collection, carefully curated over the years.

`spoti-dump` makes it easy to back up your carefully curated music library, migrate it to a new account, or start fresh.

## Features

- **Export**: Save your "Liked Songs" and all your playlists to local `.csv` files.
- **Import**: Add songs and playlists from your `.csv` backups to any Spotify account.
- **Purge**: Completely wipe all saved tracks and playlists from an account.

## Getting Started

### 1. Spotify App Setup

You need to create a free Spotify Developer App to use this tool.

<details>
<summary>Click for step-by-step instructions</summary>

1.  **Go to the Spotify Developer Dashboard and log in.**
2.  **Click `Create app`.**
    -   Give it any `App name` and `App description`.
    -   Check the `Website` and `Redirect URI` boxes. You can put any valid URL for now (e.g., `http://localhost`).
3.  **Go to your new app's `Settings`.**
4.  **Add the Redirect URI:**
    -   Find the `Redirect URIs` section.
    -   Add exactly this URI: `http://localhost:8888/callback`
    -   Click `Save`.
5.  **Copy your Credentials:**
    -   Find and copy your `Client ID` and `Client Secret`. You'll need them in the next step.
6.  **(For Import Only) Add Users:**
    -   In the Developer Dashboard, go to the `Users and Access` tab.
    -   Add the email address of the Spotify account you want to **import music to**.

</details>

### 2. Configuration

`spoti-dump` reads your Spotify credentials from a `.env` file.

1.  Rename the `.env.example` file in the project directory to `.env`.
2.  Open `.env` and paste your `Client ID` and `Client Secret`.

    ```env
    SPOTIFY_CLIENT_ID=your_client_id_here
    SPOTIFY_CLIENT_SECRET=your_client_secret_here
    ```

### 3. Installation

#### From Release (Recommended for Windows)

1.  Download the latest `.exe` file from the Releases page.
2.  Place the `.exe` and your `.env` file in the same folder.
3.  Open PowerShell or Command Prompt in that folder and run commands.

#### From Source (Linux / macOS / Windows)

1.  Install the Rust toolchain.
2.  Clone the repository:
    ```sh
    git clone https://github.com/thomas192/spoti-dump.git
    cd spoti-dump
    ```
3.  Run the application with `cargo`:
    ```sh
    # Example for exporting
    cargo run --release -- export
    ```

## Usage

All commands are run from your terminal. The first time you run a command, your browser will open to ask for Spotify authorization. Just log in and click "Agree".

### `export`

Backs up your "Liked Songs" and all playlists.

```sh
spoti-dump export
```

This creates a `dump` folder containing `saved_tracks.csv` and a `playlists` subfolder with a CSV for each of your playlists.

### `import`

Imports tracks and playlists from a `dump` folder into your Spotify account.

```sh
spoti-dump import
```

- **Important:** Before running, make sure the `dump` folder (from a previous export) is in the same directory as the executable.
- **To migrate to a new account:** Log out of Spotify in your browser, then log in to the **target account** *before* running the `import` command.

### `purge`

Removes all "Liked Songs" and unfollows all playlists from your account.

```sh
spoti-dump purge
```

> **Warning:** This action is irreversible. Use with caution!
