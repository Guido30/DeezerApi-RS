# DeezerApi-Rs

This library allows you to quickly search tracks, albums, and more from deezer public apis, including the non documented gw-light endpoint which provides more information and includes lyrics

## Installation

Add this crate into your project cargo.toml as follows:

```text
[dependencies]
deezerapi-rs = { git = "https://github.com/Guido30/DeezerApi-RS.git", branch = "main" }
```

## Usage

Initialize the Deezer object and call the api using one of the available methods

```rust
use deezerapi-rs::Deezer;

let deezer = Deezer::new();
let track = deezer.track(534534).unwrap();
let track_title = ;
println!("Track Title is: {}", track.title);
```

All methods return a `Result<T, DeezerError>` where T is the deserialized json from deezer into a rust object, or DeezerError in case the request fails or the json can not be serialized into the expected value.  
You should always appropriately handle the error, avoid using unwrap.  
Some methods like `deezer.album_tracks(94009);` will return a `Result<Vec<T>, DeezerError>` instead.

You can find a list of the available methods [here](https://github.com/Guido30/DeezerApi-RS/blob/main/DOCUMENTATION.md)
