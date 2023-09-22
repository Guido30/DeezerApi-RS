use crate::blocking::{Deezer, DeezerError};
use std::fmt::Debug;

pub fn print_errors_for_items<I: Debug, T: Debug>(
    items: &[I; 3],
    results: &Vec<Result<T, DeezerError>>,
) {
    for (index, result) in results.iter().enumerate() {
        if let Err(error) = result {
            println!("Error for {:?}: {:?}", items[index], error);
        }
    }
}

#[test]
fn refresh_token() {
    let deezer = Deezer::new();
    let token = deezer.refresh_token();
    assert_ne!(token, String::from("null"));
}

#[test]
fn test_gw_track() {
    let track_ids = [1141668, 64555665, 88765665];
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.gw_track(track_ids[0]),
        deezer.gw_track(track_ids[1]),
        deezer.gw_track(track_ids[2]),
    ];
    print_errors_for_items(&track_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn test_gw_song() {
    let song_ids = [354344, 3545446, 4363245];
    let deezer = Deezer::new();
    let songs = vec![
        deezer.gw_song(song_ids[0]),
        deezer.gw_song(song_ids[1]),
        deezer.gw_song(song_ids[2]),
    ];
    print_errors_for_items(&song_ids, &songs);
    assert_eq!(songs.iter().all(Result::is_ok), true);
}

#[test]
fn test_gw_songs() {
    let song_ids = vec![354344, 3545446, 4363245];
    let deezer = Deezer::new();
    let songs = deezer.gw_songs(&song_ids);
    if let Err(ref error) = songs {
        println!("Error {:?} ", error);
    }
    assert_eq!(songs.is_ok(), true);
}

#[test]
fn test_gw_songs_from_album() {
    let album_ids = [122366, 4535, 6566665];
    let deezer = Deezer::new();
    let album_songs = vec![
        deezer.gw_songs_from_album(album_ids[0]),
        deezer.gw_songs_from_album(album_ids[1]),
        deezer.gw_songs_from_album(album_ids[2]),
    ];
    print_errors_for_items(&album_ids, &album_songs);
    assert_eq!(album_songs.iter().all(Result::is_ok), true);
}

#[test]
fn test_gw_album() {
    let album_ids = [122366, 4535, 6566665];
    let deezer = Deezer::new();
    let albums = vec![
        deezer.gw_album(album_ids[0]),
        deezer.gw_album(album_ids[1]),
        deezer.gw_album(album_ids[2]),
    ];
    print_errors_for_items(&album_ids, &albums);
    assert_eq!(albums.iter().all(Result::is_ok), true);
}

#[test]
fn test_gw_lyrics() {
    let song_ids = [1141668, 3443212, 87766453];
    let deezer = Deezer::new();
    let lyrics = vec![
        deezer.gw_lyrics(song_ids[0]),
        deezer.gw_lyrics(song_ids[1]),
        deezer.gw_lyrics(song_ids[2]),
    ];
    print_errors_for_items(&song_ids, &lyrics);
    assert_eq!(lyrics.iter().all(Result::is_ok), true);
}

#[test]
fn test_track() {
    let track_ids = [1141668, 64555665, 88765665];
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.track(track_ids[0]),
        deezer.track(track_ids[1]),
        deezer.track(track_ids[2]),
    ];
    print_errors_for_items(&track_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn test_track_from_isrc() {
    let isrcs = ["TCAFP2196109", "GBUM71507634", "GBAYE0200771"];
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.track_from_isrc(isrcs[0]),
        deezer.track_from_isrc(isrcs[1]),
        deezer.track_from_isrc(isrcs[2]),
    ];
    print_errors_for_items(&isrcs, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn test_album() {
    let album_ids = [122366, 6545543, 6566665];
    let deezer = Deezer::new();
    let albums = vec![
        deezer.album(album_ids[0]),
        deezer.album(album_ids[1]),
        deezer.album(album_ids[2]),
    ];
    print_errors_for_items(&album_ids, &albums);
    assert_eq!(albums.iter().all(Result::is_ok), true);
}

#[test]
fn test_album_from_upc() {
    let upcs = [3700368446423, 634479384776, 707541948593];
    let deezer = Deezer::new();
    let albums = vec![
        deezer.album_from_upc(upcs[0]),
        deezer.album_from_upc(upcs[1]),
        deezer.album_from_upc(upcs[2]),
    ];
    print_errors_for_items(&upcs, &albums);
    assert_eq!(albums.iter().all(Result::is_ok), true);
}

#[test]
fn test_editorial() {
    let deezer = Deezer::new();
    let editorial = deezer.editorial();
    if let Err(ref error) = editorial {
        println!("Error {:?} ", error);
    }
    assert_eq!(editorial.is_ok(), true);
}

#[test]
fn test_editorial_from_genre() {
    let deezer = Deezer::new();
    let editorial = deezer.editorial_from_genre(113);
    if let Err(ref error) = editorial {
        println!("Error {:?} ", error);
    }
    assert_eq!(editorial.is_ok(), true);
}

#[test]
fn test_genres() {
    let deezer = Deezer::new();
    let genres = deezer.genres();
    if let Err(ref error) = genres {
        println!("Error {:?} ", error);
    }
    assert_eq!(genres.is_ok(), true);
}

#[test]
fn test_genre() {
    let genre_ids = [0, 113, 152];
    let deezer = Deezer::new();
    let genres = vec![
        deezer.genre(genre_ids[0]),
        deezer.genre(genre_ids[1]),
        deezer.genre(genre_ids[2]),
    ];
    print_errors_for_items(&genre_ids, &genres);
    assert_eq!(genres.iter().all(Result::is_ok), true);
}

#[test]
fn test_genre_artists() {
    let genre_ids = [0, 113, 152];
    let deezer = Deezer::new();
    let artists = vec![
        deezer.genre_artists(genre_ids[0]),
        deezer.genre_artists(genre_ids[1]),
        deezer.genre_artists(genre_ids[2]),
    ];
    print_errors_for_items(&genre_ids, &artists);
    assert_eq!(artists.iter().all(Result::is_ok), true);
}

#[test]
fn test_genre_radio() {
    let genre_ids = [0, 113, 152];
    let deezer = Deezer::new();
    let radios = vec![
        deezer.genre_radios(genre_ids[0]),
        deezer.genre_radios(genre_ids[1]),
        deezer.genre_radios(genre_ids[2]),
    ];
    print_errors_for_items(&genre_ids, &radios);
    assert_eq!(radios.iter().all(Result::is_ok), true);
}

#[test]
fn test_infos() {
    let deezer = Deezer::new();
    let genres = deezer.infos();
    if let Err(ref error) = genres {
        println!("Error {:?} ", error);
    }
    assert_eq!(genres.is_ok(), true);
}

#[test]
fn test_radios() {
    let deezer = Deezer::new();
    let radios = deezer.radios();
    if let Err(ref error) = radios {
        println!("Error {:?} ", error);
    }
    assert_eq!(radios.is_ok(), true);
}

#[test]
fn test_radio() {
    let radio_ids = [6, 132, 37151];
    let deezer = Deezer::new();
    let radios = vec![
        deezer.radio(radio_ids[0]),
        deezer.radio(radio_ids[1]),
        deezer.radio(radio_ids[2]),
    ];
    print_errors_for_items(&radio_ids, &radios);
    assert_eq!(radios.iter().all(Result::is_ok), true);
}

#[test]
fn test_radio_tracks() {
    let radio_ids = [6, 132, 37151];
    let deezer = Deezer::new();
    let radios = vec![
        deezer.radio_tracks(radio_ids[0]),
        deezer.radio_tracks(radio_ids[1]),
        deezer.radio_tracks(radio_ids[2]),
    ];
    print_errors_for_items(&radio_ids, &radios);
    assert_eq!(radios.iter().all(Result::is_ok), true);
}

#[test]
fn test_radio_genres() {
    let deezer = Deezer::new();
    let radios = deezer.radio_genres();
    if let Err(ref error) = radios {
        println!("Error {:?} ", error);
    }
    assert_eq!(radios.is_ok(), true);
}

#[test]
fn test_radio_top() {
    let deezer = Deezer::new();
    let radios = deezer.radio_top();
    if let Err(ref error) = radios {
        println!("Error {:?} ", error);
    }
    assert_eq!(radios.is_ok(), true);
}

#[test]
fn test_radio_lists() {
    let deezer = Deezer::new();
    let radios = deezer.radio_lists();
    if let Err(ref error) = radios {
        println!("Error {:?} ", error);
    }
    assert_eq!(radios.is_ok(), true);
}

#[test]
fn test_search() {
    let queries = ["Hans Zimmer", "OneRepublic", "Eric Prydz"];
    let deezer = Deezer::new();
    let searches = vec![
        deezer.search(queries[0], false),
        deezer.search(queries[1], false),
        deezer.search(queries[2], false),
    ];
    print_errors_for_items(&queries, &searches);
    assert_eq!(searches.iter().all(Result::is_ok), true);
}

#[test]
fn test_search_album() {
    let queries = ["Hans Zimmer", "OneRepublic", "Eric Prydz"];
    let deezer = Deezer::new();
    let searches = vec![
        deezer.search_album(queries[0], false),
        deezer.search_album(queries[1], false),
        deezer.search_album(queries[2], false),
    ];
    print_errors_for_items(&queries, &searches);
    assert_eq!(searches.iter().all(Result::is_ok), true);
}

#[test]
fn test_search_artist() {
    let queries = ["Hans Zimmer", "OneRepublic", "Eric Prydz"];
    let deezer = Deezer::new();
    let searches = vec![
        deezer.search_artist(queries[0], false),
        deezer.search_artist(queries[1], false),
        deezer.search_artist(queries[2], false),
    ];
    print_errors_for_items(&queries, &searches);
    assert_eq!(searches.iter().all(Result::is_ok), true);
}

#[test]
fn test_search_playlist() {
    let queries = ["Hans Zimmer", "OneRepublic", "Eric Prydz"];
    let deezer = Deezer::new();
    let searches = vec![
        deezer.search_playlist(queries[0], false),
        deezer.search_playlist(queries[1], false),
        deezer.search_playlist(queries[2], false),
    ];
    print_errors_for_items(&queries, &searches);
    assert_eq!(searches.iter().all(Result::is_ok), true);
}

#[test]
fn test_search_user() {
    let queries = ["Hans Zimmer", "OneRepublic", "Eric Prydz"];
    let deezer = Deezer::new();
    let searches = vec![
        deezer.search_user(queries[0], false),
        deezer.search_user(queries[1], false),
        deezer.search_user(queries[2], false),
    ];
    print_errors_for_items(&queries, &searches);
    assert_eq!(searches.iter().all(Result::is_ok), true);
}

#[test]
fn test_search_track() {
    let queries = [
        ("Candy Shop", "50 Cent", "The Massacre"),
        ("Starlight", "Muse", "Absolution"),
        ("CASTLE OF GLASS", "OneRepublic", "Night Visions"),
    ];
    let deezer = Deezer::new();
    let searches = vec![
        deezer.search_track(queries[0].0, queries[0].1, queries[0].2, false),
        deezer.search_track(queries[1].0, queries[1].1, queries[1].2, false),
        deezer.search_track(queries[2].0, queries[2].1, queries[2].2, false),
    ];
    print_errors_for_items(&queries, &searches);
    assert_eq!(searches.iter().all(Result::is_ok), true);
}
