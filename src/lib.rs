use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderValue, ACCEPT_LANGUAGE, USER_AGENT};
use reqwest::{header, Error as RequestError, Url};
use serde_json::{json, Error as JsonError, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;
use url::ParseError;

mod models;
mod tests;

const GW_API_URL: &str = "http://www.deezer.com/ajax/gw-light.php";
const API_URL: &str = "https://api.deezer.com/";
const USER_AGENT_HEADER: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36";

fn parse_response_to_value(response: Response) -> Result<Value, DeezerError> {
    let body = match response.text() {
        Ok(v) => v,
        Err(err) => return Err(DeezerError::RequestError(err)),
    };
    let value: Value = match serde_json::from_str(body.as_str()) {
        Ok(v) => v,
        Err(err) => return Err(DeezerError::JsonError(err)),
    };
    match value.get("error") {
        Some(error_value) => {
            if error_value.is_object() {
                return Err(DeezerError::ApiError(error_value.to_string()));
            }
        }
        None => (),
    };
    Ok(value)
}

#[derive(Debug)]
pub struct Deezer {
    client: Client,
    token: RefCell<String>,
}

#[derive(Debug)]
pub enum DeezerError {
    JsonError(JsonError),
    RequestError(RequestError),
    ApiError(String),
    ParseError(ParseError),
}

impl Deezer {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_HEADER));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));

        let client = Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .timeout(Duration::from_secs(15))
            .build()
            .expect("Could not establish client!");

        Deezer {
            client,
            token: RefCell::from(String::from("null")),
        }
    }

    fn refresh_token(&self) -> String {
        let empty_token = String::from("null");
        let response = self.gw_method_call("deezer.getUserData");
        match response {
            Ok(resp) => match resp.text() {
                Ok(body) => match serde_json::from_str(body.as_str()) as Result<Value, JsonError> {
                    Ok(value) => match value["results"]["checkForm"].as_str() {
                        Some(token) => {
                            let token = token.to_owned();
                            self.token.replace(token.clone());
                            token
                        }
                        None => empty_token,
                    },
                    Err(_) => empty_token,
                },
                Err(_) => empty_token,
            },
            Err(_) => empty_token,
        }
    }

    fn method_call(&self, path: &str) -> Result<Response, DeezerError> {
        let url = match Url::parse(API_URL) {
            Ok(base_url) => match base_url.join(path) {
                Ok(url) => url,
                Err(error) => return Err(DeezerError::ParseError(error)),
            },
            Err(error) => return Err(DeezerError::ParseError(error)),
        };
        let response = self.client.get(url).send();
        match response {
            Ok(r) => Ok(r),
            Err(error) => Err(DeezerError::RequestError(error)),
        }
    }

    fn gw_method_call(&self, method: &str) -> Result<Response, RequestError> {
        if self.token.borrow().to_string() == "null" && !(method == "deezer.getUserData") {
            self.refresh_token();
        }

        let api_token = match method {
            "deezer.getUserData" => "null".to_owned(),
            _ => self.token.borrow().to_string(),
        };

        let mut params = HashMap::new();
        params.insert("api_token", api_token);
        params.insert("api_version", "1.0".to_owned());
        params.insert("method", method.to_owned());
        params.insert("input", params.len().to_string());

        let response = self.client.post(GW_API_URL).form(&params).send()?;
        Ok(response)
    }

    fn gw_method_call_params(&self, method: &str, params: HashMap<&str, String>) -> Result<Response, RequestError> {
        if self.token.borrow().to_string() == "null" && !(method == "deezer.getUserData") {
            self.refresh_token();
        }

        let api_token = match method {
            "deezer.getUserData" => "null".to_owned(),
            _ => self.token.borrow().to_string(),
        };

        let mut params = params.clone();
        params.insert("api_token", api_token);
        params.insert("api_version", "1.0".to_owned());
        params.insert("method", method.to_owned());
        params.insert("input", params.len().to_string());

        let response = self.client.post(GW_API_URL).form(&params).send()?;
        Ok(response)
    }

    fn gw_method_call_body(&self, method: &str, body: &Value) -> Result<Response, RequestError> {
        if self.token.borrow().to_string() == "null" && !(method == "deezer.getUserData") {
            self.refresh_token();
        }

        let api_token = match method {
            "deezer.getUserData" => "null".to_owned(),
            _ => self.token.borrow().to_string(),
        };

        let mut params = HashMap::new();
        params.insert("api_token", api_token.clone());
        params.insert("api_version", "1.0".to_owned());
        params.insert("method", method.to_owned());
        params.insert("input", params.len().to_string());

        let url = Url::parse_with_params(GW_API_URL, params).unwrap();

        let response = self.client.post(url).json(body).send()?;
        Ok(response)
    }

    pub fn gw_track(&self, song_id: u64) -> Result<models::gw_track::TrackResults, DeezerError> {
        let params: HashMap<&str, String> = [("sng_id", song_id.to_string())].into();
        let response: Response = match self.gw_method_call_params("deezer.pageTrack", params) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(track) => Ok(track),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn gw_song(&self, song_id: u64) -> Result<models::gw_song::Song, DeezerError> {
        let params: HashMap<&str, String> = [("sng_id", song_id.to_string())].into();
        let response: Response = match self.gw_method_call_params("song.getData", params) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(song) => Ok(song),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn gw_songs(&self, song_ids: &Vec<u64>) -> Result<models::gw_song::SongListData, DeezerError> {
        let body: Value = json!({"sng_ids": song_ids});
        let response: Response = match self.gw_method_call_body("song.getListData", &body) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(songs) => Ok(songs),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn gw_songs_by_album(&self, album_id: u64) -> Result<models::gw_song::SongListData, DeezerError> {
        let params: HashMap<&str, String> = [("alb_id", album_id.to_string())].into();
        let response: Response = match self.gw_method_call_params("song.getListByAlbum", params) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(songs) => Ok(songs),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn gw_album(&self, album_id: u64) -> Result<models::gw_album::Album, DeezerError> {
        let params: HashMap<&str, String> = [("alb_id", album_id.to_string())].into();
        let response: Response = match self.gw_method_call_params("album.getData", params) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(album) => Ok(album),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn gw_lyrics(&self, song_id: u64) -> Result<models::gw_track::Lyrics, DeezerError> {
        let params: HashMap<&str, String> = [("sng_id", song_id.to_string())].into();
        let response: Response = match self.gw_method_call_params("song.getLyrics", params) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(lyrics) => Ok(lyrics),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    /*
    // TODO payload is often huge, models need to be fixed to match returned json, and takes a lot of time
    pub fn gw_artist(&self, artist_id: u64, lang: &str) -> Result<models::gw_artist::ArtistResults, DeezerError> {
        let params: HashMap<&str, String> = [("art_id", artist_id.to_string()), ("lang", lang.to_string())].into();
        let response = match self.gw_method_call_params("deezer.pageArtist", params) {
            Ok(r) => r,
            Err(err) => return Err(DeezerError::RequestError(err)),
        };
        let value: Value = parse_response(response)?;
        let track = match serde_json::from_value(value["results"].clone()) {
            Ok(v) => v,
            Err(err) => return Err(DeezerError::JsonError(err)),
        };
        Ok(track)
    }
    */

    pub fn track(&self, song_id: u64) -> Result<models::track::Track, DeezerError> {
        let path: String = format!("track/{}", song_id);
        let response: Response = self.method_call(path.as_str())?;
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value) {
            Ok(v) => Ok(v),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn track_by_isrc(&self, isrc: &str) -> Result<models::track::Track, DeezerError> {
        let path: String = format!("track/isrc:{}", isrc);
        let response: Response = self.method_call(path.as_str())?;
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value) {
            Ok(v) => Ok(v),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn album(&self, album_id: u64) -> Result<models::album::Album, DeezerError> {
        let path: String = format!("album/{}", album_id);
        let response: Response = self.method_call(path.as_str())?;
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value) {
            Ok(v) => Ok(v),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }
}
