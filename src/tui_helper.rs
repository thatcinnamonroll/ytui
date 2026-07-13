use crate::tui::{MenuState, App};

pub fn switch_state(app: &mut App) {
    match app.state {
        MenuState::Playlist => {app.state = MenuState::Tracklist;}
        MenuState::Tracklist => {app.state = MenuState::Playlist;}
    }
}

pub fn refresh_playlists(app: &mut App){
    let playlists = app.playlist_man.list_playlist();
    app.playlists = playlists;
}

pub fn open_playlist(app: &mut App) {
    let index_of_selected_playlist = app.playlist_state.selected().unwrap();
    let selected_playlist = &app.playlists[index_of_selected_playlist];

    let opened_playlist = app.playlist_man.read_playlist(selected_playlist.to_string());
    app.tracklist = opened_playlist.to_vec();
    switch_state(app);
}
