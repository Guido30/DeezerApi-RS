use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MainTrack {
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
pub struct MainAlbum {
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
pub struct TracksData {
    pub data: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Track {
    pub id: u64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: Option<String>,
    pub isrc: Option<String>,
    pub link: Option<String>,
    pub duration: u64,
    pub track_position: Option<u64>,
    pub disk_number: Option<u64>,
    pub rank: u64,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: u8,
    pub explicit_content_cover: u8,
    pub preview: String,
    #[serde(default)]
    pub contributors: Vec<Contributor>,
    pub md5_image: String,
    pub artist: Artist,
    #[serde(default)]
    pub album: Album,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub link: Option<String>,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub genre_id: Option<i64>,
    pub nb_tracks: Option<u64>,
    pub fans: Option<i64>,
    pub release_date: Option<String>,
    pub record_type: Option<String>,
    pub tracklist: String,
    pub explicit_lyrics: Option<bool>,
    #[serde(default)]
    pub artist: Artist,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Contributor {
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
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub link: Option<String>,
    pub share: Option<String>,
    pub picture: Option<String>,
    pub picture_small: Option<String>,
    pub picture_medium: Option<String>,
    pub picture_big: Option<String>,
    pub picture_xl: Option<String>,
    pub nb_album: Option<u64>,
    pub nb_fan: Option<u64>,
    pub radio: Option<bool>,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Playlist {
    pub id: u64,
    pub title: String,
    pub public: bool,
    pub nb_tracks: Option<u64>,
    pub link: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub checksum: String,
    pub tracklist: String,
    pub creation_date: String,
    pub md5_image: String,
    pub picture_type: String,
    pub user: PlaylistUser,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlaylistUser {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Editorial {
    pub id: u64,
    pub name: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
