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
    let home_dir = env::home_dir().unwrap();
    let cache_dir = home_dir.join(".cache/ytui").to_string_lossy().to_string();
    let playlists_man = PlaylistHelper{
        playlists_path: home_dir.join(".local/share/ytui")
    };

    let yt_man = crate::youtube::YtMan{
        ytdlp_bin_path: "/bin/yt-dlp".to_string(),
        cache_dir: cache_dir.clone()
    };

    let placeholder_song = playlists::Song{
        id: "YTUI-TEST-ID".to_string(),
        name: "No playlist open!".to_string(),
        author: "Song Author".to_string()
    };

    let playlists = playlists_man.list_playlist();
    let tracklist = vec![placeholder_song];
    let tracklist_only_name = vec![];

    let list_state1 = ListState::default().with_selected(Some(0));
    let list_state2 = ListState::default().with_selected(Some(0));

    let pipeline = crate::player::init_player();
    let music_player = crate::player::MusicPlayer{
        cache_dir: cache_dir,
        pipeline: pipeline,
    };

    let mut app = tui::App{
        playlists: playlists,
        playlist_man: playlists_man,
        playlist_state: list_state1,
        tracklist: tracklist,
        tracklist_state: list_state2,
        tracklist_only_name:tracklist_only_name,
        state: tui::MenuState::Playlist,
        yt: yt_man,
        player: music_player
    };
    let _ = tui::start_tui(&mut app);
}
