use crate::tui::{focused_block, menu_state, app};

pub fn switch_focus(app: &mut app){
    match app.focused {
        focused_block::Music => {
            if app.state == menu_state::Playlist{
                app.focused = focused_block::Playlist;
            }else {
                app.focused = focused_block::TrackList;
            }
        }
        focused_block::Playlist | focused_block::TrackList => {app.focused = focused_block::Music}
    }
}

pub fn switch_state(app: &mut app) {
    match app.state {
        menu_state::Playlist => {
            if app.focused == focused_block::Playlist{
                app.focused = focused_block::TrackList;
            }
            app.state = menu_state::Tracklist;

        }
        menu_state::Tracklist => {
            if app.focused == focused_block::TrackList{
                app.focused = focused_block::Playlist;
            }
            app.state = menu_state::Playlist
        }
    }
}
