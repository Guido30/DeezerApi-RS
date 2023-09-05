# Documentation

## gw-light API Methods

### `gw_track(song_id: u64) -> Result<models::gw::TrackData, DeezerError>`

This method retrieves detailed track data for a specific song identified by its song_id.

### `gw_song(song_id: u64) -> Result<models::gw::Song, DeezerError>`

This method retrieves basic information about a specific song identified by its song_id.

### `gw_songs(song_ids: &Vec<u64>) -> Result<models::gw::SongListData, DeezerError>`

This method retrieves a list of songs based on a provided list of song IDs.

### `gw_songs_from_album(album_id: u64) -> Result<models::gw::SongListData, DeezerError>`

This method retrieves a list of songs associated with a specific album identified by its album_id.

### `gw_album(album_id: u64) -> Result<models::gw::Album, DeezerError>`

This method retrieves information about a specific album identified by its album_id.

### `gw_lyrics(song_id: u64) -> Result<models::gw::Lyrics, DeezerError>`

This method retrieves the lyrics associated with a specific song identified by its song_id.

## Public API Methods

### `track(song_id: u64) -> Result<models::api::MainTrack, DeezerError>`

This method retrieves detailed information about a specific track identified by its song_id.

### `track_from_isrc(isrc: &str) -> Result<models::api::MainTrack, DeezerError>`

This method retrieves detailed information about a track identified by its International Standard Recording Code (ISRC).

### `album(album_id: u64) -> Result<models::api::MainAlbum, DeezerError>`

This method retrieves detailed information about a specific album identified by its album_id.

### `album_from_upc(upc: u64) -> Result<models::api::MainAlbum, DeezerError>`

This method retrieves detailed information about an album identified by its Universal Product Code (UPC).

### `album_tracks(album_id: u64) -> Result<Vec<models::api::Track>, DeezerError>`

This method retrieves a list of tracks associated with a specific album identified by its album_id.

### `artist(artist_id: u64) -> Result<models::api::Artist, DeezerError>`

This method retrieves detailed information about a specific artist identified by their artist_id.

### `artist_albums(artist_id: u64) -> Result<Vec<models::api::Album>, DeezerError>`

This method retrieves a list of albums associated with a specific artist identified by their artist_id.

### `artist_top_tracks(artist_id: u64) -> Result<Vec<models::api::Track>, DeezerError>`

This method retrieves a list of top tracks associated with a specific artist identified by their artist_id.

### `artist_related_artists(artist_id: u64) -> Result<Vec<models::api::RelatedArtist>, DeezerError>`

This method retrieves a list of related artists associated with a specific artist identified by their artist_id.

### `artist_radio(artist_id: u64) -> Result<Vec<models::api::Track>, DeezerError>`

This method retrieves a list of tracks for an artist's radio station identified by their artist_id.

### `artist_playlists(artist_id: u64) -> Result<Vec<models::api::Playlist>, DeezerError>`

This method retrieves a list of playlists associated with a specific artist identified by their artist_id.

### `editorial() -> Result<Vec<models::api::Editorial>, DeezerError>`

This method retrieves a list of editorial content (genres).

### `editorial_from_genre(genre_id: u64) -> Result<models::api::Editorial, DeezerError>`

This method retrieves editorial content related to a specific genre identified by its genre_id.

### `genres() -> Result<Vec<models::api::Editorial>, DeezerError>`

This method retrieves a list of available genres.

### `genre(genre_id: u64) -> Result<models::api::Editorial, DeezerError>`

This method retrieves information about a specific genre identified by its genre_id.

### `genre_artists(genre_id: u64) -> Result<Vec<models::api::Artist>, DeezerError>`

This method retrieves a list of artists associated with a specific genre identified by its genre_id.

### `genre_radios(genre_id: u64) -> Result<Vec<models::api::Radio>, DeezerError>`

This method retrieves a list of radios associated with a specific genre identified by its genre_id.

### `infos() -> Result<models::api::Info, DeezerError>`

This method retrieves general information based on the request location.

### `radios() -> Result<Vec<models::api::Radio>, DeezerError>`

This method retrieves a list of current available radios on Deezer.

### `radio(radio_id: u64) -> Result<models::api::Radio, DeezerError>`

This method retrieves information about a specific radio station identified by its radio_id.

### `radio_tracks(radio_id: u64) -> Result<Vec<models::api::Track>, DeezerError>`

This method retrieves a list of tracks associated with a specific radio station identified by its radio_id.

### `radio_genres() -> Result<Vec<models::api::GenreRadios>, DeezerError>`

This method retrieves a list of genres and their associated radios.

### `radio_top() -> Result<Vec<models::api::Radio>, DeezerError>`

This method retrieves a list of top radio stations.

### `radio_lists() -> Result<Vec<models::api::Radio>, DeezerError>`

This method retrieves a list of current radios.

### `search(query: &str, strict: bool) -> Result<Vec<models::api::Track>, DeezerError>`

This method performs a search using the given query string and returns a list of tracks.

### `search_album(query: &str, strict: bool) -> Result<Vec<models::api::Album>, DeezerError>`

This method performs a search using the given query string and retrieves a list of albums.

### `search_artist(query: &str, strict: bool) -> Result<Vec<models::api::Artist>, DeezerError>`

This method performs a search using the given query string and retrieves a list of artists.

### `search_playlist(query: &str, strict: bool) -> Result<Vec<models::api::Playlist>, DeezerError>`

This method performs a search using the given query string and retrieves a list of playlists.

### `search_user(query: &str, strict: bool) -> Result<Vec<models::api::User>, DeezerError>`

This method performs a search using the given query string and retrieves a list of users.

### `search_track(track: &str, artist: &str, album: &str, strict: bool) -> Result<models::api::Track, DeezerError>`

This method tries to search a track with the given parameters until one is returned or errors if all tries fail.  
The requests are sent in order with the following parameters:

- track + artist + album.
- track + artist
- track
