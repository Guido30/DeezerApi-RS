use crate::{Deezer, DeezerError};
use std::fmt::Debug;

pub fn print_errors_for_items<I: Debug, T: Debug>(items: &[I; 3], results: &Vec<Result<T, DeezerError>>) {
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
fn test_track_by_isrc() {
    let isrcs = ["TCAFP2196109", "GBUM71507634", "GBAYE0200771"];
    let deezer = Deezer::new();
    let tracks = vec![
        deezer.track_by_isrc(isrcs[0]),
        deezer.track_by_isrc(isrcs[1]),
        deezer.track_by_isrc(isrcs[2]),
    ];
    print_errors_for_items(&isrcs, &tracks);
    assert_eq!(tracks.iter().all(Result::is_ok), true);
}

#[test]
fn test_album_by_upc() {
    let upcs = [3700368446423, 634479384776, 707541948593];
    let deezer = Deezer::new();
    let albums = vec![
        deezer.album_by_upc(upcs[0]),
        deezer.album_by_upc(upcs[1]),
        deezer.album_by_upc(upcs[2]),
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
fn temporary() {
    let deezer = Deezer::new();
    let payload = deezer.search("OneRepublic", false).unwrap();
    println!("{:?}", payload.len());
    assert_eq!(false, true);
}
