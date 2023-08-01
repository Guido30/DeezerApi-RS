use serde::Deserialize;

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
    pub release_date: String,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub link: Option<String>,
    pub share: Option<String>,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub radio: Option<bool>,
    pub tracklist: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Contributor {
    pub id: i64,
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
