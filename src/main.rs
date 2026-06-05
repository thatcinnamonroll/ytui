use std::env;

pub mod youtube;
pub mod player;

fn main() {
    let cache_dir = format!("{}/.cache/ytui",env::home_dir().unwrap().as_path().to_str().unwrap());
    let downloader = youtube::ytdownload{
        ytdlp_bin_path:String::from("/bin/yt-dlp"),
        cache_dir:String::from(cache_dir)
    };
}
