use std::env;
use ratatui::widgets::ListState;

pub mod youtube;
pub mod player;
pub mod playlists;
pub mod tui;

fn main() {
    let mut list_state1 = ListState::default().with_selected(Some(0));
    let mut app = tui::app{
        playlists: None,
        tracklist: None,
        list_state: list_state1,
        focused: tui::focused_block::Music,
        state: tui::menu_state::Playlist,
    };
    tui::start_tui(&mut app);
}
