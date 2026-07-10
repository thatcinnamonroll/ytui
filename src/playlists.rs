use std::path::PathBuf;
use std::fs::{read_to_string, read_dir};
use serde_json::{json, Value};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct song{
    pub id:String,
    pub name: String,
    pub author: String
}

pub struct playlist_helper{
    pub playlists_path: PathBuf
}

impl playlist_helper{
    pub fn list_playlist(&self) -> Vec<String>{
        let mut playlists : Vec<String> = vec![];
        let files = read_dir(&self.playlists_path).unwrap();

        for mut file in files {
            let mut string_file_name = file.as_mut().unwrap().file_name().display().to_string();
            if !string_file_name.ends_with(".json"){
                continue;
            }
            playlists.push(string_file_name.trim_end_matches(".json").clone().to_string());
        }
        return playlists;
    }

    pub fn read_playlist(&self, playlist_name: String) -> Vec<song>{
        let mut playlist_path = self.playlists_path.clone();
        let playlist_name_json = playlist_name + ".json";
        playlist_path.push(playlist_name_json);
        let raw_playlist_data = read_to_string(playlist_path).unwrap();
        let raw_playlist_data_str = raw_playlist_data.as_str();

        let mut playlist = vec![];

        let playlist_data: Vec<song> = serde_json::from_str(raw_playlist_data_str).unwrap();
        for music in playlist_data{
            playlist.push(music);
        }
        return playlist;
    }

}
