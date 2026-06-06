use std::env;

pub mod youtube;
pub mod player;
pub mod playlists;

fn main() {
    let mut playlist = playlists::read_playlist();
    let mut playlist_iter = playlist.iter();
    for song in playlist_iter {
        println!("Song : {}",song.name);
    }
}
