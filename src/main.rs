use mpris::PlayerFinder;
use rustfm_scrobble::{Scrobble, Scrobbler};
use std::process;

mod config;

fn main() {
    let api_keys = match config::load_config() {
        Ok(api_keys) => api_keys,
        Err(err) => {
            println!("Error while loading config: {}", err);
            process::exit(1);
        },
    };

    let mut scrobbler = Scrobbler::new(api_keys.api_key, api_keys.api_secret);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Could not read username");
    let username = input.trim().to_owned();

    input.clear();

    std::io::stdin().read_line(&mut input)
        .expect("Could not read password");
    let password = input.trim().to_owned();

    scrobbler.authenticate_with_password(username, password)
        .expect("Could not authenticate with Last.fm");

    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    let meta = player.get_metadata()
        .expect("Could not get metadata");

    let length = meta.length_in_microseconds()
        .expect("No length in microseconds found for this track");

    if length < 30 * 1000 * 1000 {
        println!("Not allowed to scrobble this track :(");
        return;
    }

    let artist = meta.artists()
        .expect("No artist list found for this track")
        .first()
        .expect("No artist found for this track");
    let title = meta.title()
        .expect("No title found for this track");
    let album = meta.album_name()
        .expect("No album name found for this track");

    println!("Now playing: {} - {} ({})", artist, title, album);

    let scrobble = Scrobble::new(artist.clone(), title.to_owned(), album.to_owned());
    let np_result = scrobbler.now_playing(scrobble.clone())
        .unwrap();
    let scrobble_result = scrobbler.scrobble(scrobble)
        .unwrap();

    println!("Now playing result:\n{:#?}", np_result);
    println!("Scrobble result:\n{:#?}", scrobble_result);
}
