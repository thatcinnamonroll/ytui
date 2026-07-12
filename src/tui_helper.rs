use crate::tui::{focused_block, menu_state, app};
use crate::playlists::playlist_helper;

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

pub fn refresh_playlists(app: &mut app){
    let playlists = app.playlist_man.list_playlist();
    app.playlists = playlists;
}

pub fn open_playlist(app: &mut app) {
    let index_of_selected_playlist = app.playlist_state.selected().unwrap();
    let selected_playlist = &app.playlists[index_of_selected_playlist];

    let opened_playlist = app.playlist_man.read_playlist(selected_playlist.to_string());
    app.tracklist = opened_playlist.to_vec();
    switch_state(app);
}
