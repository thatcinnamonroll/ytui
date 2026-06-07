use std::env;

pub mod youtube;
pub mod player;
pub mod playlists;
pub mod tui;

fn main() {
    tui::start_tui();
}
