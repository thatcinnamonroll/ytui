use std::process::Command;
use std::path::PathBuf;

pub struct YtMan {
    pub ytdlp_bin_path: String,
    pub cache_dir: String,
}

impl YtMan {
    pub fn download(&self,yt_id : &str){
        let vid_url = "https://www.youtube.com/watch?v=".to_owned() + yt_id;
        let _response = Command::new(&self.ytdlp_bin_path)
                        .args(["-x","--audio-format","opus","-P",&self.cache_dir.clone(),"-o","%(id)s",&vid_url])
                        .output()
                        .expect("failed to run yt-dlp");
    }

    pub fn ensure_downloaded(&self,yt_id: String) -> bool{
        let song_file_path = self.cache_dir.clone() + "/"+ &yt_id + ".opus";
        let song_path = PathBuf::from(song_file_path);
        let song_exist = song_path.exists();
        return song_exist;
    }
}
