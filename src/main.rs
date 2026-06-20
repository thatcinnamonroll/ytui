use std::env;
use ratatui::widgets::ListState;

pub mod youtube;
pub mod player;
pub mod playlists;
pub mod tui;
pub mod tui_helper;

fn main() {
    let mut list_state1 = ListState::default().with_selected(Some(0));
    let mut list_state2 = ListState::default().with_selected(Some(0));
    let mut app = tui::app{
        playlists: None,
        playlist_state: list_state1,
        tracklist: None,
        tracklist_state: list_state2,
        focused: tui::focused_block::Music,
        state: tui::menu_state::Playlist,
    };
    tui::start_tui(&mut app);
}
