use std::env;
use ratatui::widgets::ListState;

use crate::playlists::PlaylistHelper;

pub mod youtube;
pub mod player;
pub mod playlists;
pub mod tui;
pub mod tui_helper;

fn main() {
    // TODO add error handeling
    let mut home_dir = env::home_dir().unwrap();
    home_dir.push(".local/share/ytui");
    let playlists_man = PlaylistHelper{
        playlists_path: home_dir
    };

    let placeholder_song = playlists::Song{
        id: "Some id".to_string(),
        name: "No playlist open!".to_string(),
        author: "Song Author".to_string()
    };

    let playlists = playlists_man.list_playlist();
    let tracklist = vec![placeholder_song];
    let tracklist_only_name = vec![];

    let list_state1 = ListState::default().with_selected(Some(0));
    let list_state2 = ListState::default().with_selected(Some(0));

    let mut app = tui::App{
        playlists: playlists,
        playlist_man: playlists_man,
        playlist_state: list_state1,
        tracklist: tracklist,
        tracklist_state: list_state2,
        tracklist_only_name:tracklist_only_name,
        state: tui::MenuState::Playlist,
    };
    let _ = tui::start_tui(&mut app);
}
