use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
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
