use std::path::PathBuf;
use std::fs::{read_to_string, read_dir};
use serde_json::json;

pub struct song{
    pub id:String,
    pub name: String,
    pub author: String
}

pub struct playlist_helper{
    pub playlists_path: PathBuf
}

impl playlist_helper{
    pub fn list_playlist(self){
        let files = read_dir(self.playlists_path).unwrap();

        for file in files {
            println!("Name: {}", file.unwrap().path().display());
        }
    }

}

// This is debug func, as of right now it gives back test func
// TODO add here "playlist name" argument
pub fn read_playlist() -> Vec<song>{
    let mut playlist = vec![];
    let song1 = song{
        id: String::from("SOME id"),
        name:String::from("Very cool song"),
        author:String::from("Cool guy"),
    };
    let song2 = song{
        id:String::from("Diffrent id"),
        name:String::from("Song about life"),
        author: String::from("Some good artist"),
    };
    playlist.push(song1);
    playlist.push(song2);
    return playlist;
}
