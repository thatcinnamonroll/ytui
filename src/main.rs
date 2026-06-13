use std::env;

pub mod youtube;
pub mod player;
pub mod playlists;
pub mod tui;

fn main() {
    let mut app = tui::app{
        playlists: None,
        tracklist: None,
        focused: tui::focused_block::Music,
        state: tui::menu_state::Playlist,
    };
    tui::start_tui(app);
}
