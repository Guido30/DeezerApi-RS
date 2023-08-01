use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Artist {
    pub art_id: String,
    pub role_id: String,
    pub artists_songs_order: String,
    pub art_name: String,
    pub artist_is_dummy: bool,
    pub art_picture: String,
    pub rank: String,
    pub locales: Locales,
    pub __type__: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Rights {
    pub stream_ads_available: Option<bool>,
    pub stream_ads: Option<String>,
    pub stream_sub_available: Option<bool>,
    pub stream_sub: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct ExplicitContent {
    pub explicit_lyrics_status: i32,
    pub explicit_cover_status: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Locales {
    Locale(HashMap<String, HashMap<String, String>>),
    Empty(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Media {
    #[serde(rename = "TYPE")]
    pub media_type: String,
    pub href: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum SngContributors {
    SngContributors(HashMap<String, Vec<String>>),
    Empty(Vec<String>),
}
