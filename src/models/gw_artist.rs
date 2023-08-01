use super::gw_shared::{Artist, ExplicitContent, Locales, Media, Rights, SngContributors};
use super::gw_track::Track;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// --------------------
// RESULTS
// --------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ArtistResults {
    #[serde(rename = "DATA")]
    pub artist: ArtistData,
    #[serde(skip_serializing)]
    pub top: Top,
    #[serde(skip_serializing)]
    pub highlight: Highlight,
    #[serde(skip_serializing)]
    pub bio: Bio,
    #[serde(skip_serializing)]
    pub selected_playlist: SelectedPlaylist,
    #[serde(skip_serializing)]
    pub related_playlist: RelatedPlaylist,
    #[serde(skip_serializing)]
    pub related_artists: RelatedArtists,
    #[serde(skip_serializing)]
    pub video: Video,
    #[serde(skip_serializing)]
    pub albums: Albums,
}

// --------------------
// ARTIST_DATA
// --------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ArtistData {
    pub art_id: String,
    pub art_name: String,
    pub url_rewriting: String,
    pub art_picture: String,
    pub nb_fan: i64,
    pub url: String,
    pub twitter: String,
    pub facebook: String,
    pub status: Status,
    pub user_id: String,
    pub data: ArtistDataData,
    pub __type__: String,
    pub smartradio: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ArtistDataData {
    pub locales: Locales,
    pub copyright_picture: Option<String>,
    pub art_banner: Option<String>,
    pub art_banner_align: Option<String>,
    pub art_banner_bgcolor: Option<String>,
    pub art_banner_link: Option<String>,
    pub disable_catalog: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub status: String,
    pub date: String,
}

// --------------------
// TOP
// --------------------

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Top {
    Top(TopData),
    Empty(Vec<Value>),
}

#[derive(Debug, Deserialize, Clone)]
pub struct TopData {
    #[serde(rename = "data")]
    pub songs: Vec<Song>,
    pub count: i64,
    pub total: i64,
    pub version: String,
    pub filtered_count: i64,
    #[serde(skip_serializing)]
    pub filtered_items: Vec<String>,
    pub next: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Song {
    pub sng_id: String,
    pub product_track_id: String,
    pub upload_id: i64,
    pub sng_title: String,
    pub art_id: String,
    pub provider_id: String,
    pub art_name: String,
    pub artist_is_dummy: bool,
    pub artists: Vec<Artist>,
    pub alb_id: String,
    pub alb_title: String,
    #[serde(rename = "TYPE")]
    pub type_field: i64,
    pub video: bool,
    pub duration: String,
    pub alb_picture: String,
    pub art_picture: String,
    pub rank_sng: String,
    pub filesize_aac_64: String,
    pub filesize_mp3_64: String,
    pub filesize_mp3_128: String,
    pub filesize_mp3_256: String,
    pub filesize_mp3_320: String,
    pub filesize_mp4_ra1: String,
    pub filesize_mp4_ra2: String,
    pub filesize_mp4_ra3: String,
    pub filesize_mhm1_ra1: String,
    pub filesize_mhm1_ra2: String,
    pub filesize_mhm1_ra3: String,
    pub filesize_flac: String,
    pub filesize: String,
    pub gain: String,
    pub media_version: String,
    pub disk_number: String,
    pub track_number: String,
    pub track_token: String,
    pub track_token_expire: i64,
    pub version: String,
    pub media: Vec<Media>,
    pub explicit_lyrics: String,
    pub rights: Rights,
    pub isrc: String,
    pub hierarchical_title: String,
    pub sng_contributors: SngContributors,
    pub lyrics_id: i64,
    pub explicit_track_content: ExplicitContent,
    pub __type__: String,
    pub __payload__: Option<Payload>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Payload {
    pub country_rank: Option<i64>,
    pub global_rank: Option<i64>,
    pub score: Option<i64>,
}

// --------------------
// HIGHLIGHT
// --------------------

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Highlight {
    Highlight(HighlightData),
    Empty(Value),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct HighlightData {
    #[serde(rename = "TYPE")]
    pub type_field: String,
    pub title: String,
    pub item: Value,
}

// --------------------
// BIO
// --------------------

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bio {
    BioData(BioData),
    BioBool(bool),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct BioData {
    pub bio: String,
    pub resume: String,
    pub source: String,
}

// --------------------
// SELECTED_PLAYLIST
// --------------------

#[derive(Debug, Deserialize, Clone)]
pub struct SelectedPlaylist {
    #[serde(rename = "data")]
    pub playlists: Vec<Playlist>,
    pub count: i64,
    pub total: i64,
    pub version: Option<i64>,
    pub filtered_count: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Playlist {
    pub playlist_id: String,
    pub title: String,
    pub playlist_picture: String,
    pub picture_type: String,
    pub parent_user_id: String,
    pub parent_username: String,
    pub nb_fan: i64,
    pub nb_song: i64,
    pub status: i64,
    pub __type__: String,
    pub __payload__: Option<Payload>,
}

// --------------------
// RELATED_PLAYLIST
// --------------------

#[derive(Debug, Deserialize, Clone)]
pub struct RelatedPlaylist {
    #[serde(rename = "data")]
    pub playlists: Vec<Playlist>,
    pub count: i64,
    pub total: i64,
    pub list_payload: Value,
    pub version: String,
    pub filtered_count: i64,
    pub filtered_items: Vec<String>,
    pub next: i64,
}

// --------------------
// RELATED_ARTISTS
// --------------------

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct RelatedArtists {
    #[serde(rename = "data")]
    pub artists: Vec<RelatedArtistData>,
    pub count: i64,
    pub total: i64,
    pub list_payload: Value,
    pub version: String,
    pub filtered_count: i64,
    pub element_id: i64,
    pub start: i64,
    pub nb: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct RelatedArtistData {
    pub art_id: String,
    pub art_name: String,
    pub art_picture: String,
    pub nb_fan: i64,
    pub url_rewriting: String,
    pub data: RelatedArtistDataData,
    pub __type__: String,
    pub __payload__: Option<Payload>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct RelatedArtistDataData {
    #[serde(rename = "locales")]
    pub locales: Locales,
    pub copyright_picture: Option<String>,
    pub copyright_banner: Option<String>,
    pub disable_catalog: Option<bool>,
}

// --------------------
// VIDEO
// --------------------

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Video {
    VideoError(HashMap<String, HashMap<String, String>>),
    VideoData(Value),
}

// --------------------
// ALBUMS
// --------------------

#[derive(Debug, Deserialize, Clone)]

pub struct Albums {
    #[serde(rename = "data")]
    pub albums: Vec<Album>,
    pub count: i64,
    pub total: i64,
    pub cache_version: i64,
    pub filtered_count: i64,
    pub art_id: i64,
    pub start: i64,
    pub nb: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Album {
    pub product_album_id: String,
    pub alb_id: String,
    pub alb_title: String,
    pub alb_picture: String,
    pub highlight: String,
    pub digital_release_date: String,
    pub physical_release_date: String,
    pub original_release_date: String,
    pub producer_line: String,
    #[serde(rename = "TYPE")]
    pub type_field: String,
    pub subtypes: AlbumSubtypes,
    pub role_id: i64,
    pub url_rewriting: String,
    pub artists: Vec<AlbumArtist>,
    pub art_name: String,
    pub rank: String,
    pub copyright: String,
    pub explicit_album_content: ExplicitContent,
    pub artists_albums_is_official: bool,
    pub __type__: String,
    pub songs: Track,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSubtypes {
    pub is_studio: bool,
    pub is_live: bool,
    pub is_compilation: bool,
    pub is_karaoke: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct AlbumArtist {
    pub art_id: String,
    pub role_id: String,
    pub artists_albums_order: String,
    pub art_name: String,
    pub art_picture: String,
    pub rank: String,
    pub locales: Locales,
    pub artist_is_dummy: bool,
    pub __type__: String,
}
