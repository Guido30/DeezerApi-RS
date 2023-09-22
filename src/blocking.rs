use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderValue, ACCEPT_LANGUAGE, USER_AGENT};
use reqwest::{header, Error as RequestError, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Error as JsonError, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use url::ParseError;

use crate::models;
use crate::{API_URL, GW_API_URL, USER_AGENT_HEADER};

#[derive(Debug, Clone)]
pub struct Deezer {
    client: Client,
    token: Arc<Mutex<String>>,
}

#[derive(Debug)]
pub enum DeezerError {
    JsonError(JsonError),
    RequestError(RequestError),
    ApiError(String),
    ParseError(ParseError),
}

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

        Self {
            client,
            token: Arc::new(Mutex::new(String::from("null"))),
        }
    }

    pub fn refresh_token(&self) -> String {
        let empty_token = String::from("null");
        let response = self.gw_method_call("deezer.getUserData");
        match response {
            Ok(resp) => match resp.text() {
                Ok(body) => match serde_json::from_str(body.as_str())
                    as Result<Value, JsonError>
                {
                    Ok(value) => match value["results"]["checkForm"].as_str() {
                        Some(new_token) => {
                            let new_token = new_token.to_owned();
                            let mut token = self.token.lock().unwrap();
                            *token = new_token.clone();
                            new_token
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
        let mut url = match Url::parse(API_URL) {
            Ok(base_url) => match base_url.join(path) {
                Ok(url) => url,
                Err(error) => return Err(DeezerError::ParseError(error)),
            },
            Err(error) => return Err(DeezerError::ParseError(error)),
        };
        let mut params: HashMap<String, String> = url
            .query_pairs()
            .map(|x| (x.0.into_owned(), x.1.into_owned()))
            .collect();
        params
            .entry("limit".to_string())
            .or_insert("100".to_string());
        url.query_pairs_mut()
            .clear()
            .extend_pairs(params.into_iter());
        let response = self.client.get(url).send();
        match response {
            Ok(r) => Ok(r),
            Err(error) => Err(DeezerError::RequestError(error)),
        }
    }

    fn gw_method_call(&self, method: &str) -> Result<Response, RequestError> {
        let mut token = self.token.lock().unwrap().to_owned();
        if token == "null" && !(method == "deezer.getUserData") {
            self.refresh_token();
            token = self.token.lock().unwrap().to_owned();
        }

        let api_token = match method {
            "deezer.getUserData" => "null".to_owned(),
            _ => token,
        };

        let mut params = HashMap::new();
        params.insert("api_token", api_token);
        params.insert("api_version", "1.0".to_owned());
        params.insert("method", method.to_owned());
        params.insert("input", params.len().to_string());

        let response = self.client.post(GW_API_URL).form(&params).send()?;
        Ok(response)
    }

    fn gw_method_call_params(
        &self,
        method: &str,
        params: HashMap<&str, String>,
    ) -> Result<Response, RequestError> {
        let mut token = self.token.lock().unwrap().to_owned();
        if token == "null" && !(method == "deezer.getUserData") {
            self.refresh_token();
            token = self.token.lock().unwrap().to_owned();
        }

        let api_token = match method {
            "deezer.getUserData" => "null".to_owned(),
            _ => token,
        };

        let mut params = params.clone();
        params.insert("api_token", api_token);
        params.insert("api_version", "1.0".to_owned());
        params.insert("method", method.to_owned());
        params.insert("input", params.len().to_string());

        let response = self.client.post(GW_API_URL).form(&params).send()?;
        Ok(response)
    }

    fn gw_method_call_body(
        &self,
        method: &str,
        body: &Value,
    ) -> Result<Response, RequestError> {
        let mut token = self.token.lock().unwrap().to_owned();
        if token == "null" && !(method == "deezer.getUserData") {
            self.refresh_token();
            token = self.token.lock().unwrap().to_owned();
        }

        let api_token = match method {
            "deezer.getUserData" => "null".to_owned(),
            _ => token,
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

    fn call_deserialize_gw_request_with_params<T: DeserializeOwned>(
        &self,
        method: &str,
        params: HashMap<&str, String>,
    ) -> Result<T, DeezerError> {
        let response: Response =
            match self.gw_method_call_params(method, params) {
                Ok(r) => r,
                Err(err) => return Err(DeezerError::RequestError(err)),
            };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(v) => Ok(v),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    fn call_deserialize_api_request<T: DeserializeOwned>(
        &self,
        url_path: &str,
    ) -> Result<T, DeezerError> {
        let response: Response = self.method_call(url_path)?;
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value) {
            Ok(v) => Ok(v),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    fn call_deserialize_api_request_as_vec<T: DeserializeOwned>(
        &self,
        url_path: &str,
    ) -> Result<Vec<T>, DeezerError> {
        let mut objects: Vec<T> = Vec::new();
        let mut url_path = url_path.to_string();
        loop {
            let response: Response = self.method_call(url_path.as_str())?;
            let value: Value = parse_response_to_value(response)?;
            let result: Vec<T> =
                match serde_json::from_value(value["data"].clone()) {
                    Ok(v) => v,
                    Err(err) => return Err(DeezerError::JsonError(err)),
                };
            objects.extend(result);
            if let Some(next) = value.get("next") {
                url_path = match next.as_str() {
                    Some(next_url) => next_url.to_string(),
                    None => break,
                };
            } else {
                break;
            }
        }
        Ok(objects)
    }

    pub fn gw_track(
        &self,
        song_id: u64,
    ) -> Result<models::gw::TrackData, DeezerError> {
        let params: HashMap<&str, String> =
            [("sng_id", song_id.to_string())].into();
        self.call_deserialize_gw_request_with_params("deezer.pageTrack", params)
    }

    pub fn gw_song(
        &self,
        song_id: u64,
    ) -> Result<models::gw::Song, DeezerError> {
        let params: HashMap<&str, String> =
            [("sng_id", song_id.to_string())].into();
        self.call_deserialize_gw_request_with_params("song.getData", params)
    }

    pub fn gw_songs(
        &self,
        song_ids: &Vec<u64>,
    ) -> Result<models::gw::SongListData, DeezerError> {
        let body: Value = json!({"sng_ids": song_ids});
        let response: Response =
            match self.gw_method_call_body("song.getListData", &body) {
                Ok(r) => r,
                Err(err) => return Err(DeezerError::RequestError(err)),
            };
        let value: Value = parse_response_to_value(response)?;
        match serde_json::from_value(value["results"].clone()) {
            Ok(songs) => Ok(songs),
            Err(err) => return Err(DeezerError::JsonError(err)),
        }
    }

    pub fn gw_songs_from_album(
        &self,
        album_id: u64,
    ) -> Result<models::gw::SongListData, DeezerError> {
        let params: HashMap<&str, String> =
            [("alb_id", album_id.to_string())].into();
        self.call_deserialize_gw_request_with_params(
            "song.getListByAlbum",
            params,
        )
    }

    pub fn gw_album(
        &self,
        album_id: u64,
    ) -> Result<models::gw::Album, DeezerError> {
        let params: HashMap<&str, String> =
            [("alb_id", album_id.to_string())].into();
        self.call_deserialize_gw_request_with_params("album.getData", params)
    }

    pub fn gw_lyrics(
        &self,
        song_id: u64,
    ) -> Result<models::gw::Lyrics, DeezerError> {
        let params: HashMap<&str, String> =
            [("sng_id", song_id.to_string())].into();
        self.call_deserialize_gw_request_with_params("song.getLyrics", params)
    }

    // TODO payload is huge, models need to be implemented to match returned json, and takes a lot of time
    // pub fn gw_artist(&self, artist_id: u64, lang: &str) -> Result<models::gw::ArtistResults, DeezerError> {
    //     let params: HashMap<&str, String> = [("art_id", artist_id.to_string()), ("lang", lang.to_string())].into();
    //     let response = match self.gw_method_call_params("deezer.pageArtist", params) {
    //         Ok(r) => r,
    //         Err(err) => return Err(DeezerError::RequestError(err)),
    //     };
    //     let value: Value = parse_response(response)?;
    //     let track = match serde_json::from_value(value["results"].clone()) {
    //         Ok(v) => v,
    //         Err(err) => return Err(DeezerError::JsonError(err)),
    //     };
    //     Ok(track)
    // }

    pub fn track(
        &self,
        song_id: u64,
    ) -> Result<models::api::MainTrack, DeezerError> {
        self.call_deserialize_api_request(format!("track/{}", song_id).as_str())
    }

    pub fn track_from_isrc(
        &self,
        isrc: &str,
    ) -> Result<models::api::MainTrack, DeezerError> {
        self.call_deserialize_api_request(
            format!("track/isrc:{}", isrc).as_str(),
        )
    }

    pub fn album(
        &self,
        album_id: u64,
    ) -> Result<models::api::MainAlbum, DeezerError> {
        self.call_deserialize_api_request(
            format!("album/{}", album_id).as_str(),
        )
    }

    pub fn album_from_upc(
        &self,
        upc: u64,
    ) -> Result<models::api::MainAlbum, DeezerError> {
        self.call_deserialize_api_request(format!("album/upc:{}", upc).as_str())
    }

    pub fn album_tracks(
        &self,
        album_id: u64,
    ) -> Result<Vec<models::api::Track>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("album/{}/tracks", album_id).as_str(),
        )
    }

    pub fn artist(
        &self,
        artist_id: u64,
    ) -> Result<models::api::Artist, DeezerError> {
        self.call_deserialize_api_request(
            format!("artist/{}", artist_id).as_str(),
        )
    }

    pub fn artist_albums(
        &self,
        artist_id: u64,
    ) -> Result<Vec<models::api::Album>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("artist/{}/albums", artist_id).as_str(),
        )
    }

    pub fn artist_top_tracks(
        &self,
        artist_id: u64,
    ) -> Result<Vec<models::api::Track>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("artist/{}/top", artist_id).as_str(),
        )
    }

    pub fn artist_related_artists(
        &self,
        artist_id: u64,
    ) -> Result<Vec<models::api::RelatedArtist>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("artist/{}/related", artist_id).as_str(),
        )
    }

    pub fn artist_radio(
        &self,
        artist_id: u64,
    ) -> Result<Vec<models::api::Track>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("artist/{}/radio", artist_id).as_str(),
        )
    }

    pub fn artist_playlists(
        &self,
        artist_id: u64,
    ) -> Result<Vec<models::api::Playlist>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("artist/{}/playlists", artist_id).as_str(),
        )
    }

    pub fn editorial(
        &self,
    ) -> Result<Vec<models::api::Editorial>, DeezerError> {
        self.call_deserialize_api_request_as_vec("editorial")
    }

    pub fn editorial_from_genre(
        &self,
        genre_id: u64,
    ) -> Result<models::api::Editorial, DeezerError> {
        self.call_deserialize_api_request(
            format!("editorial/{}", genre_id).as_str(),
        )
    }

    pub fn genres(&self) -> Result<Vec<models::api::Editorial>, DeezerError> {
        self.call_deserialize_api_request_as_vec("genre")
    }

    pub fn genre(
        &self,
        genre_id: u64,
    ) -> Result<models::api::Editorial, DeezerError> {
        self.call_deserialize_api_request(
            format!("genre/{}", genre_id).as_str(),
        )
    }

    pub fn genre_artists(
        &self,
        genre_id: u64,
    ) -> Result<Vec<models::api::Artist>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("genre/{}/artists", genre_id).as_str(),
        )
    }

    pub fn genre_radios(
        &self,
        genre_id: u64,
    ) -> Result<Vec<models::api::Radio>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("genre/{}/radios", genre_id).as_str(),
        )
    }

    pub fn infos(&self) -> Result<models::api::Info, DeezerError> {
        self.call_deserialize_api_request("infos")
    }

    pub fn radios(&self) -> Result<Vec<models::api::Radio>, DeezerError> {
        self.call_deserialize_api_request_as_vec("radio")
    }

    pub fn radio(
        &self,
        radio_id: u64,
    ) -> Result<models::api::Radio, DeezerError> {
        self.call_deserialize_api_request(
            format!("radio/{}", radio_id).as_str(),
        )
    }

    pub fn radio_tracks(
        &self,
        radio_id: u64,
    ) -> Result<Vec<models::api::Track>, DeezerError> {
        self.call_deserialize_api_request_as_vec(
            format!("radio/{}/tracks", radio_id).as_str(),
        )
    }

    pub fn radio_genres(
        &self,
    ) -> Result<Vec<models::api::GenreRadios>, DeezerError> {
        self.call_deserialize_api_request_as_vec("radio/genres")
    }

    pub fn radio_top(&self) -> Result<Vec<models::api::Radio>, DeezerError> {
        self.call_deserialize_api_request_as_vec("radio/top")
    }

    pub fn radio_lists(&self) -> Result<Vec<models::api::Radio>, DeezerError> {
        self.call_deserialize_api_request_as_vec("radio/lists")
    }

    pub fn search(
        &self,
        query: &str,
        strict: bool,
    ) -> Result<Vec<models::api::Track>, DeezerError> {
        let strict = match strict {
            true => "on",
            false => "off",
        };
        self.call_deserialize_api_request_as_vec(
            format!("search?q={query}&strict={strict}").as_str(),
        )
    }

    pub fn search_album(
        &self,
        query: &str,
        strict: bool,
    ) -> Result<Vec<models::api::Album>, DeezerError> {
        let strict = match strict {
            true => "on",
            false => "off",
        };
        self.call_deserialize_api_request_as_vec(
            format!("search/album?q={query}&strict={strict}").as_str(),
        )
    }

    pub fn search_artist(
        &self,
        query: &str,
        strict: bool,
    ) -> Result<Vec<models::api::Artist>, DeezerError> {
        let strict = match strict {
            true => "on",
            false => "off",
        };
        self.call_deserialize_api_request_as_vec(
            format!("search/artist?q={query}&strict={strict}").as_str(),
        )
    }

    pub fn search_playlist(
        &self,
        query: &str,
        strict: bool,
    ) -> Result<Vec<models::api::Playlist>, DeezerError> {
        let strict = match strict {
            true => "on",
            false => "off",
        };
        self.call_deserialize_api_request_as_vec(
            format!("search/playlist?q={query}&strict={strict}").as_str(),
        )
    }

    pub fn search_user(
        &self,
        query: &str,
        strict: bool,
    ) -> Result<Vec<models::api::User>, DeezerError> {
        let strict = match strict {
            true => "on",
            false => "off",
        };
        self.call_deserialize_api_request_as_vec(
            format!("search/user?q={query}&strict={strict}").as_str(),
        )
    }

    pub fn search_track(
        &self,
        track: &str,
        artist: &str,
        album: &str,
        strict: bool,
    ) -> Result<models::api::Track, DeezerError> {
        let strict = match strict {
            true => "on",
            false => "off",
        };
        let searches: Vec<models::api::Track> = self.call_deserialize_api_request_as_vec(
            format!(r#"search/track?q=track:"{track}" artist:"{artist}" album:"{album}"&strict={strict}"#).as_str(),
        )?;
        if let Some(_) = searches.get(0) {
            return Ok(searches[0].to_owned());
        }
        let searches: Vec<models::api::Track> =
            self.call_deserialize_api_request_as_vec(format!(r#"search/track?q=track:"{track}" artist:"{artist}"&strict={strict}"#).as_str())?;
        if let Some(_) = searches.get(0) {
            return Ok(searches[0].to_owned());
        }
        let searches: Vec<models::api::Track> = self
            .call_deserialize_api_request_as_vec(
                format!(r#"search/track?q=track:"{track}"&strict={strict}"#)
                    .as_str(),
            )?;
        if let Some(_) = searches.get(0) {
            return Ok(searches[0].to_owned());
        }
        Err(DeezerError::ApiError("No Track Found".to_string()))
    }
}
