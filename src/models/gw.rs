use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct TrackData {
    #[serde(rename = "DATA")]
    pub track: Track,
    pub lyrics: Option<Lyrics>,
    pub isrc: Isrc,
    pub related_albums: RelatedAlbums,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Track {
    pub sng_id: String,
    pub product_track_id: Option<String>,
    pub upload_id: Option<i64>,
    pub sng_title: String,
    pub art_id: String,
    pub provider_id: String,
    pub art_name: String,
    pub digital_release_date: Option<String>,
    pub artist_is_dummy: Option<bool>,
    pub artists: Vec<Artist>,
    pub alb_id: String,
    pub alb_title: String,
    #[serde(rename = "TYPE")]
    pub type_field: i64,
    pub video: Option<bool>,
    pub duration: String,
    pub alb_picture: String,
    pub art_picture: Option<String>,
    pub rank_sng: String,
    #[serde(default)]
    pub filesize_aac_64: String,
    #[serde(default)]
    pub filesize_mp3_64: String,
    #[serde(default)]
    pub filesize_mp3_128: String,
    #[serde(default)]
    pub filesize_mp3_256: String,
    #[serde(default)]
    pub filesize_mp3_320: String,
    #[serde(default)]
    pub filesize_mp4_ra1: String,
    #[serde(default)]
    pub filesize_mp4_ra2: String,
    #[serde(default)]
    pub filesize_mp4_ra3: String,
    #[serde(default)]
    pub filesize_mhm1_ra1: String,
    #[serde(default)]
    pub filesize_mhm1_ra2: String,
    #[serde(default)]
    pub filesize_mhm1_ra3: String,
    #[serde(default)]
    pub filesize_flac: String,
    pub filesize: String,
    pub gain: Option<String>,
    pub media_version: String,
    pub disk_number: String,
    pub track_number: String,
    pub track_token: String,
    pub track_token_expire: i64,
    pub version: Option<String>,
    pub media: Vec<Media>,
    pub explicit_lyrics: String,
    pub rights: Rights,
    pub isrc: String,
    pub hierarchical_title: Option<String>,
    pub sng_contributors: Option<SngContributors>,
    pub lyrics_id: i64,
    pub explicit_track_content: ExplicitContent,
    pub genre_id: Option<String>,
    pub copyright: Option<String>,
    pub physical_release_date: String,
    pub s_mod: Option<i64>,
    pub s_premium: Option<i64>,
    pub date_start_premium: Option<String>,
    pub date_start: Option<String>,
    pub smartradio: Option<i64>,
    pub status: i64,
    pub user_id: i64,
    pub url_rewriting: Option<String>,
    pub sng_status: Option<String>,
    #[serde(default)]
    pub available_countries: AvailableCountries,
    pub update_date: Option<String>,
    pub __type__: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Song {
    pub alb_id: String,
    pub alb_picture: String,
    pub alb_title: String,
    pub artists: Vec<Artist>,
    pub art_id: String,
    pub art_name: String,
    pub artist_is_dummy: bool,
    pub digital_release_date: String,
    pub disk_number: String,
    pub duration: String,
    pub explicit_lyrics: String,
    pub explicit_track_content: ExplicitContent,
    #[serde(skip_serializing)]
    pub fallback: Option<Value>,
    pub genre_id: String,
    pub hierarchical_title: String,
    pub isrc: String,
    pub lyrics_id: i64,
    pub physical_release_date: String,
    pub provider_id: String,
    pub rank: String,
    pub smartradio: i32,
    pub sng_contributors: SngContributors,
    pub sng_id: String,
    pub sng_title: String,
    pub status: i32,
    pub track_number: String,
    pub user_id: i64,
    pub version: Option<String>,
    pub filesize_aac_64: String,
    pub filesize_mp3_64: String,
    pub filesize_mp3_128: String,
    pub filesize_mp3_256: String,
    pub filesize_mp3_320: String,
    pub filesize_mp4_ra1: String,
    pub filesize_mp4_ra2: String,
    pub filesize_mp4_ra3: String,
    pub filesize_flac: String,
    pub filesize: String,
    pub gain: Option<String>,
    pub media_version: String,
    pub track_token: String,
    pub track_token_expire: i64,
    pub media: Vec<Media>,
    pub rights: Rights,
    pub __type__: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SongListData {
    #[serde(rename = "data")]
    pub songs: Option<Vec<Track>>,
    pub count: i64,
    pub total: i64,
    pub filtered_count: i64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Rights {
    pub stream_ads_available: Option<bool>,
    pub stream_ads: Option<String>,
    pub stream_sub_available: Option<bool>,
    pub stream_sub: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
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

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Media {
    #[serde(rename = "TYPE")]
    pub media_type: String,
    pub href: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Lyrics {
    pub lyrics_id: String,
    pub lyrics_sync_json: Option<Vec<LyricsSyncItem>>,
    pub lyrics_text: String,
    pub lyrics_copyrights: String,
    pub lyrics_writers: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LyricsSyncItem {
    pub lrc_timestamp: Option<String>,
    pub milliseconds: Option<String>,
    pub duration: Option<String>,
    pub line: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Isrc {
    pub data: Vec<IsrcData>,
    pub count: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct IsrcData {
    pub art_name: String,
    pub art_id: String,
    pub alb_picture: String,
    pub alb_id: String,
    pub alb_title: String,
    pub duration: String,
    pub digital_release_date: String,
    pub rights: Rights,
    pub lyrics_id: i64,
    pub __type__: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RelatedAlbums {
    pub data: Vec<RelatedAlbumsData>,
    pub count: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct RelatedAlbumsData {
    pub art_name: String,
    pub art_id: String,
    pub alb_picture: String,
    pub alb_id: String,
    pub alb_title: String,
    pub duration: String,
    pub digital_release_date: String,
    pub rights: Rights,
    pub lyrics_id: i64,
    pub __type__: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum SngContributors {
    SngContributors(HashMap<String, Vec<String>>),
    Empty(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct AvailableCountries {
    pub stream_ads: Vec<String>,
    pub stream_sub_only: Vec<String>,
}

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

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct Album {
    pub alb_id: String,
    pub alb_picture: String,
    pub explicit_album_content: ExplicitContent,
    pub alb_title: String,
    pub art_id: String,
    pub art_name: String,
    pub copyright: String,
    pub digital_release_date: String,
    pub genre_id: String,
    pub label_name: String,
    pub nb_fan: i32,
    pub number_disk: String,
    pub number_track: String,
    pub physical_release_date: String,
    pub original_release_date: Option<String>,
    pub rank: String,
    pub rank_art: String,
    pub status: String,
    pub __type__: String,
}
