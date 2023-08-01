use super::shared::Contributor;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: u64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: Option<String>,
    pub isrc: String,
    pub link: String,
    pub share: String,
    pub duration: u64,
    pub track_position: u64,
    pub disk_number: u64,
    pub rank: u64,
    pub release_date: String,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: u8,
    pub explicit_content_cover: u8,
    pub preview: String,
    pub bpm: f32,
    pub gain: f32,
    pub available_countries: Vec<String>,
    pub contributors: Vec<Contributor>,
    pub md5_image: String,
    pub artist: Artist,
    pub album: Album,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub link: String,
    pub share: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub radio: bool,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub link: String,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub release_date: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
