pub struct song{
    pub id:String,
    pub name: String,
    pub author: String
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
