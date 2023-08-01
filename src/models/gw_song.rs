use super::gw_shared::{Artist, ExplicitContent, Media, Rights, SngContributors};
use serde::Deserialize;
use serde_json::Value;

// ------------------------------------------------
// Payload retrieved using the song.getData method
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
    pub explicit_track_content: ExplicitContent,
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
