use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Track {
    pub added_at: Option<String>,
    pub track: TrackDetails,
}

#[derive(Debug, Deserialize)]
pub struct TrackDetails {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub name: String,
}
