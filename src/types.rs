use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: Option<String>,
    #[serde(default, deserialize_with = "default_on_null")]
    pub name: String,
    #[serde(default, deserialize_with = "default_on_null")]
    pub artists: Vec<Artist>,
    #[serde(default, deserialize_with = "default_on_null")]
    pub album: Album,
}

#[derive(Debug, Deserialize, Default)]
pub struct Artist {
    #[serde(default, deserialize_with = "default_on_null")]
    pub name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Album {
    #[serde(default, deserialize_with = "default_on_null")]
    pub name: String,
}

fn default_on_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
