use std::process::Command;

pub struct ytdownload {
    pub ytdlp_bin_path: String,
    pub cache_dir: String,
}

impl ytdownload {
    pub fn download(&self,vid_url : &str){
        let response = Command::new(&self.ytdlp_bin_path)
                        .args(["-x","--audio-format","opus","-P",&self.cache_dir.clone(),vid_url.clone()])
                        .output()
                        .expect("failed to run yt-dlp");
    }
}
