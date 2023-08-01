use super::shared::{Album, Artist, Contributor};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: i64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: String,
    pub isrc: String,
    pub link: String,
    pub share: String,
    pub duration: i64,
    pub track_position: i64,
    pub disk_number: i64,
    pub rank: i64,
    pub release_date: String,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: i64,
    pub explicit_content_cover: i64,
    pub preview: String,
    pub bpm: f32,
    pub gain: f64,
    pub available_countries: Vec<Value>,
    pub contributors: Vec<Contributor>,
    pub md5_image: String,
    pub artist: Artist,
    pub album: Album,
    #[serde(rename = "type")]
    pub type_field: String,
}
