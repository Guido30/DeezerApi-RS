#[cfg(test)]
mod tests {
    use crate::Deezer;
    use rand::Rng;

    #[test]
    fn test_refresh_token() {
        let deezer = Deezer::new();
        let token = deezer.refresh_token();
        assert_ne!(token, String::from("null"));
    }

    #[test]
    fn test_gw_track() {
        let mut rng = rand::thread_rng();
        let song_ids = [
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
        ];
        let deezer = Deezer::new();
        let tracks = vec![
            deezer.gw_track(song_ids[0]),
            deezer.gw_track(song_ids[1]),
            deezer.gw_track(song_ids[2]),
        ];
        for (index, result) in tracks.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", song_ids[index], error);
            }
        }
        assert_eq!(tracks.iter().all(Result::is_ok), true);
    }

    #[test]
    fn test_gw_song() {
        let mut rng = rand::thread_rng();
        let song_ids = [
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
        ];
        let deezer = Deezer::new();
        let songs = vec![
            deezer.gw_song(song_ids[0]),
            deezer.gw_song(song_ids[1]),
            deezer.gw_song(song_ids[2]),
        ];
        for (index, result) in songs.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", song_ids[index], error);
            }
        }
        assert_eq!(songs.iter().all(Result::is_ok), true);
    }

    #[test]
    fn test_gw_songs() {
        let mut rng = rand::thread_rng();
        let song_ids = vec![
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
        ];
        let deezer = Deezer::new();
        let song_list = deezer.gw_songs(&song_ids);
        if let Err(error) = &song_list {
            println!("Error for {:?}: {:?}", song_ids, error);
        }
        assert_eq!(song_list.is_ok(), true);
    }

    #[test]
    fn test_gw_songs_by_album() {
        let mut rng = rand::thread_rng();
        let album_id = rng.gen_range(1..=100_000) as u64;
        let deezer = Deezer::new();
        let song_list = deezer.gw_songs_by_album(album_id);
        if let Err(error) = &song_list {
            println!("Error for {:?}: {:?}", album_id, error);
        }
        assert_eq!(song_list.is_ok(), true);
    }

    #[test]
    fn test_gw_album() {
        let mut rng = rand::thread_rng();
        let album_ids = [
            rng.gen_range(1..=100_000),
            rng.gen_range(1..=100_000),
            rng.gen_range(1..=100_000),
        ];
        let deezer = Deezer::new();
        let albums = vec![
            deezer.gw_album(album_ids[0]),
            deezer.gw_album(album_ids[1]),
            deezer.gw_album(album_ids[2]),
        ];
        for (index, result) in albums.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", album_ids[index], error);
            }
        }
        assert_eq!(albums.iter().all(Result::is_ok), true);
    }

    #[test]
    fn test_gw_lyrics() {
        let mut rng = rand::thread_rng();
        let song_ids = [
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
        ];
        let deezer = Deezer::new();
        let lyrics = vec![
            deezer.gw_lyrics(song_ids[0]),
            deezer.gw_lyrics(song_ids[1]),
            deezer.gw_lyrics(song_ids[2]),
        ];
        for (index, result) in lyrics.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", song_ids[index], error);
            }
        }
        assert_eq!(lyrics.iter().all(Result::is_ok), true);
    }

    /*
    #[test]
    fn test_gw_artist() {
        let mut rng = rand::thread_rng();
        let artist_ids = [
            rng.gen_range(1..=1_500),
            rng.gen_range(1..=1_500),
            rng.gen_range(1..=1_500),
        ];
        let deezer = Deezer::new();
        let artists = vec![
            deezer.gw_artist(artist_ids[0], "en"),
            deezer.gw_artist(artist_ids[1], "en"),
            deezer.gw_artist(artist_ids[2], "en"),
        ];
        for (index, result) in artists.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", artist_ids[index], error);
            }
        }
        assert_eq!(artists.iter().all(Result::is_ok), true);
    }
    */

    #[test]
    fn test_track() {
        let mut rng = rand::thread_rng();
        let song_ids = [
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
            rng.gen_range(250_000..=350_000),
        ];
        let deezer = Deezer::new();
        let tracks = vec![
            deezer.track(song_ids[0]),
            deezer.track(song_ids[1]),
            deezer.track(song_ids[2]),
        ];
        for (index, result) in tracks.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", song_ids[index], error);
            }
        }
        assert_eq!(tracks.iter().all(Result::is_ok), true);
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
        for (index, result) in tracks.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", isrcs[index], error);
            }
        }
        assert_eq!(tracks.iter().all(Result::is_ok), true);
    }

    #[test]
    fn test_album() {
        let mut rng = rand::thread_rng();
        let album_ids = [
            rng.gen_range(1..=100_000),
            rng.gen_range(1..=100_000),
            rng.gen_range(1..=100_000),
        ];
        let deezer = Deezer::new();
        let albums = vec![
            deezer.album(album_ids[0]),
            deezer.album(album_ids[1]),
            deezer.album(album_ids[2]),
        ];
        for (index, result) in albums.iter().enumerate() {
            if let Err(error) = result {
                println!("Error for {:?}: {:?} ", album_ids[index], error);
            }
        }
        assert_eq!(albums.iter().all(Result::is_ok), true);
    }
}
