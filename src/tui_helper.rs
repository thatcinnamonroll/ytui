use crate::tui::{focused_block, menu_state, app};

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
