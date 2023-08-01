use super::shared::{Artist, Contributor};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub upc: String,
    pub link: String,
    pub share: String,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub genre_id: i64,
    pub genres: GenresData,
    pub label: String,
    pub nb_tracks: i64,
    pub duration: i64,
    pub fans: i64,
    pub release_date: String,
    pub record_type: String,
    pub available: bool,
    pub tracklist: String,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: i64,
    pub explicit_content_cover: i64,
    pub contributors: Vec<Contributor>,
    pub artist: Artist,
    #[serde(rename = "type")]
    pub type_field: String,
    pub tracks: TracksData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenresData {
    pub data: Vec<Genre>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
    pub picture: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TracksData {
    pub data: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: i64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: String,
    pub link: String,
    pub duration: i64,
    pub rank: i64,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: i64,
    pub explicit_content_cover: i64,
    pub preview: String,
    pub md5_image: String,
    pub artist: Artist,
    pub album: Album,
    #[serde(rename = "type")]
    pub type_field: String,
}
