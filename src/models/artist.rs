use super::shared::Contributor;
use serde::Deserialize;

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
    pub id: i64,
    pub title: String,
    pub link: String,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub genre_id: i64,
    pub fans: i64,
    pub release_date: String,
    pub record_type: String,
    pub tracklist: String,
    pub explicit_lyrics: bool,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: u64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: Option<String>,
    pub link: String,
    pub duration: u64,
    pub rank: u64,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: u8,
    pub explicit_content_cover: u8,
    pub preview: String,
    pub contributors: Vec<Contributor>,
    pub md5_image: String,
    pub artist: TrackArtist,
    pub album: TrackAlbum,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackAlbum {
    pub id: u64,
    pub title: String,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackArtist {
    pub id: u64,
    pub name: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelatedArtist {
    pub id: u64,
    pub name: String,
    pub link: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub nb_album: u64,
    pub nb_fan: u64,
    pub radio: bool,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
