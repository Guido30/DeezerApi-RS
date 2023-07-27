use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// ------------------------------------------------
// Payloads retrieved using the song.getData method
// ------------------------------------------------

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
    pub explicit_track_content: ExplicitTrackContent,
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
    pub version: String,
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
#[serde(untagged)]
pub enum Locales {
    HashMapLocale(HashMap<String, HashMap<String, String>>),
    VecLocale(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum SngContributors {
    HashMapSngContributors(HashMap<String, Vec<String>>),
    VecSngContributors(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Media {
    #[serde(rename = "TYPE")]
    pub media_type: String,
    pub href: String,
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
pub struct ExplicitTrackContent {
    pub explicit_lyrics_status: i32,
    pub explicit_cover_status: i32,
}

// -----------------------------------------------------
// Payloads retrieved using the deezer.pageTrack method
// -----------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct TrackResults {
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
    pub hierarchical_title: String,
    pub sng_contributors: SngContributors,
    pub lyrics_id: i64,
    pub explicit_track_content: ExplicitTrackContent,
    pub copyright: String,
    pub physical_release_date: String,
    pub s_mod: i64,
    pub s_premium: i64,
    pub date_start_premium: String,
    pub date_start: String,
    pub status: i64,
    pub user_id: i64,
    pub url_rewriting: String,
    pub sng_status: String,
    pub available_countries: AvailableCountries,
    pub update_date: String,
    pub __type__: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct AvailableCountries {
    pub stream_ads: Vec<String>,
    pub stream_sub_only: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Lyrics {
    pub lyrics_id: String,
    pub lyrics_sync_json: Option<Vec<LyricsSyncJson>>,
    pub lyrics_text: String,
    pub lyrics_copyrights: String,
    pub lyrics_writers: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LyricsSyncJson {
    pub lrc_timestamp: Option<String>,
    pub milliseconds: Option<String>,
    pub duration: Option<String>,
    pub line: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Isrc {
    pub data: Vec<IsrcData>,
    pub count: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct RelatedAlbums {
    pub data: Vec<RelatedAlbumsData>,
    pub count: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize)]
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

// -----------------------------------------------------
// Payloads retrieved using the song.getListData method
// -----------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct SongListData {
    #[serde(rename = "data")]
    pub songs: Option<Vec<SongListItem>>,
    pub count: i64,
    pub total: i64,
    pub filtered_count: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SongListItem {
    pub sng_id: String,
    pub alb_id: String,
    pub alb_picture: String,
    pub alb_title: String,
    pub artists: Vec<Artist>,
    pub art_id: String,
    pub art_name: String,
    pub digital_release_date: String,
    pub disk_number: String,
    pub duration: String,
    pub explicit_lyrics: String,
    pub explicit_track_content: ExplicitTrackContent,
    pub genre_id: String,
    pub isrc: String,
    pub lyrics_id: i64,
    pub physical_release_date: String,
    pub provider_id: String,
    pub rank_sng: String,
    pub smartradio: i64,
    pub sng_title: String,
    pub status: i64,
    pub track_number: String,
    #[serde(rename = "TYPE")]
    pub type_field: i64,
    pub upload_id: Option<i64>,
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
