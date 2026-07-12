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
    let playlists_man = playlist_helper{
        playlists_path: home_dir
    };

    let placeholder_song = playlists::song{
        id: "Some id".to_string(),
        name: "No playlist open!".to_string(),
        author: "Song Author".to_string()
    };

    let playlists = playlists_man.list_playlist();
    let tracklist = vec![placeholder_song];

    let mut list_state1 = ListState::default().with_selected(Some(0));
    let mut list_state2 = ListState::default().with_selected(Some(0));

    let mut app = tui::app{
        playlists: playlists,
        playlist_man: playlists_man,
        playlist_state: list_state1,
        tracklist: tracklist,
        tracklist_state: list_state2,
        focused: tui::focused_block::Playlist,
        state: tui::menu_state::Playlist,
    };
    tui::start_tui(&mut app);
}
