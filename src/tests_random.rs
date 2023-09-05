use crate::tests::print_errors_for_items;
use crate::Deezer;
use rand::distributions::uniform::SampleRange;
use rand::Rng;

fn gen_three_rand_nums<T: SampleRange<u64> + Clone>(range: T) -> [u64; 3] {
    let mut rng = rand::thread_rng();
    [
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
    ]
}

#[test]
fn rand_gw_track() {
    let song_ids = gen_three_rand_nums(250_000..=350_000);
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.gw_track(song_ids[0]),
        deezer.gw_track(song_ids[1]),
        deezer.gw_track(song_ids[2]),
    ];
    print_errors_for_items(&song_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn rand_gw_song() {
    let song_ids = gen_three_rand_nums(250_000..=350_000);
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
fn rand_gw_songs() {
    let song_ids = gen_three_rand_nums(250_000..=350_000).to_vec();
    let deezer = Deezer::new();
    let song_list = deezer.gw_songs(&song_ids);
    if let Err(error) = &song_list {
        println!("Error for {:?}: {:?}", song_ids, error);
    }
    assert_eq!(song_list.is_ok(), true);
}

#[test]
fn rand_gw_songs_from_album() {
    let album_ids = gen_three_rand_nums(1..=100_000);
    let deezer = Deezer::new();
    let song_list = vec![
        deezer.gw_songs_from_album(album_ids[0]),
        deezer.gw_songs_from_album(album_ids[1]),
        deezer.gw_songs_from_album(album_ids[2]),
    ];
    print_errors_for_items(&album_ids, &song_list);
    assert_eq!(song_list.iter().all(Result::is_ok), true);
}

#[test]
fn rand_gw_album() {
    let album_ids = gen_three_rand_nums(1..=100_000);
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
fn rand_gw_lyrics() {
    let song_ids = gen_three_rand_nums(250_000..=350_000);
    let deezer = Deezer::new();
    let lyrics = vec![
        deezer.gw_lyrics(song_ids[0]),
        deezer.gw_lyrics(song_ids[1]),
        deezer.gw_lyrics(song_ids[2]),
    ];
    print_errors_for_items(&song_ids, &lyrics);
    assert_eq!(lyrics.iter().all(Result::is_ok), true);
}

/*
#[test]
fn rand_gw_artist() {
    let artist_ids = gen_three_rand_nums(0..=30_000);
    let deezer = Deezer::new();
    let artists = vec![
        deezer.gw_artist(artist_ids[0], "en"),
        deezer.gw_artist(artist_ids[1], "en"),
        deezer.gw_artist(artist_ids[2], "en"),
    ];
    print_errors_for_items(&artist_ids, &artists);
    assert_eq!(artists.iter().all(Result::is_ok), true);
}
*/

#[test]
fn rand_track() {
    let song_ids = gen_three_rand_nums(250_000..=350_000);
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.track(song_ids[0]),
        deezer.track(song_ids[1]),
        deezer.track(song_ids[2]),
    ];
    print_errors_for_items(&song_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn rand_album() {
    let album_ids = gen_three_rand_nums(1..=100_000);
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
fn rand_album_tracks() {
    let album_ids = gen_three_rand_nums(1..=100_000);
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.album_tracks(album_ids[0]),
        deezer.album_tracks(album_ids[1]),
        deezer.album_tracks(album_ids[2]),
    ];
    print_errors_for_items(&album_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn rand_artist() {
    let artist_ids = gen_three_rand_nums(1..=50_000);
    let deezer = Deezer::new();
    let artists = vec![
        deezer.artist(artist_ids[0]),
        deezer.artist(artist_ids[1]),
        deezer.artist(artist_ids[2]),
    ];
    print_errors_for_items(&artist_ids, &artists);
    assert_eq!(artists.iter().all(Result::is_ok), true);
}

#[test]
fn rand_artist_albums() {
    let artist_ids = gen_three_rand_nums(1..=50_000);
    let deezer = Deezer::new();
    let albums = vec![
        deezer.artist_albums(artist_ids[0]),
        deezer.artist_albums(artist_ids[1]),
        deezer.artist_albums(artist_ids[2]),
    ];
    print_errors_for_items(&artist_ids, &albums);
    assert_eq!(albums.iter().all(Result::is_ok), true);
}

#[test]
fn rand_artist_top_tracks() {
    let artist_ids = gen_three_rand_nums(1..=50_000);
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.artist_top_tracks(artist_ids[0]),
        deezer.artist_top_tracks(artist_ids[1]),
        deezer.artist_top_tracks(artist_ids[2]),
    ];
    print_errors_for_items(&artist_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn rand_artist_related_artists() {
    let artist_ids = gen_three_rand_nums(1..=50_000);
    let deezer = Deezer::new();
    let artists = vec![
        deezer.artist_related_artists(artist_ids[0]),
        deezer.artist_related_artists(artist_ids[1]),
        deezer.artist_related_artists(artist_ids[2]),
    ];
    print_errors_for_items(&artist_ids, &artists);
    assert_eq!(artists.iter().all(Result::is_ok), true);
}

#[test]
fn rand_artist_radio() {
    let artist_ids = gen_three_rand_nums(1..=50_000);
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.artist_radio(artist_ids[0]),
        deezer.artist_radio(artist_ids[1]),
        deezer.artist_radio(artist_ids[2]),
    ];
    print_errors_for_items(&artist_ids, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn rand_artist_playlists() {
    let artist_ids = gen_three_rand_nums(1..=50_000);
    let deezer = Deezer::new();
    let playlists = vec![
        deezer.artist_playlists(artist_ids[0]),
        deezer.artist_playlists(artist_ids[1]),
        deezer.artist_playlists(artist_ids[2]),
    ];
    print_errors_for_items(&artist_ids, &playlists);
    assert_eq!(playlists.iter().all(Result::is_ok), true);
}
