use std::env;
use gstreamer::glib::home_dir;
use ratatui::widgets::ListState;

use crate::playlists::playlist_helper;

pub mod youtube;
pub mod player;
pub mod playlists;
pub mod tui;
pub mod tui_helper;

fn main() {
    // TODO add error handeling
    let mut home_dir = env::home_dir().unwrap();
    home_dir.push(".local/share/ytui");
    let mut playlists = playlist_helper{
        playlists_path: home_dir
    };
    playlists.list_playlist();

}
