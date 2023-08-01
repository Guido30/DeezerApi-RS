use super::shared::Contributor;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Album {
    pub id: u64,
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
    pub nb_tracks: u64,
    pub duration: u64,
    pub fans: u64,
    pub release_date: String,
    pub record_type: String,
    pub available: bool,
    pub tracklist: String,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: u8,
    pub explicit_content_cover: u8,
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
    pub id: u64,
    pub name: String,
    pub picture: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TracksData {
    pub data: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: u64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: String,
    pub link: String,
    pub duration: u64,
    pub rank: u64,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: u8,
    pub explicit_content_cover: u8,
    pub preview: String,
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
pub struct AlbumTrack {
    pub id: u64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: String,
    pub isrc: String,
    pub link: String,
    pub duration: u64,
    pub track_position: u64,
    pub disk_number: u64,
    pub rank: u64,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: u8,
    pub explicit_content_cover: u8,
    pub preview: String,
    pub md5_image: String,
    pub artist: AlbumTrackArtist,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumTrackArtist {
    pub id: u64,
    pub name: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
