use super::gw_shared::ExplicitContent;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
