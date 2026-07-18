use gstreamer::prelude::*;
use gstreamer::MessageView;
use gstreamer::Element;

pub fn init_player() -> Element{
    gstreamer::init().unwrap();

    let pipeline = gstreamer::parse::launch("playbin").unwrap();
    return pipeline;
}

pub struct MusicPlayer {
    pub cache_dir: String,
    pub pipeline: Element,
}

impl  MusicPlayer {
    pub fn play_file(&self,music_id: &str){
        self.pipeline
            .set_state(gstreamer::State::Paused);

        let file_path = self.cache_dir.clone() + "/" + music_id + ".opus";

        self.pipeline.set_property("uri", &format!("file://{}", file_path));

        self.pipeline
            .set_state(gstreamer::State::Playing);


        let bus = self.pipeline.bus().unwrap();
        for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                            err.src().map(|s| s.path_string()),
                            err.error(),
                            err.debug()
                    );
                    break;
                }
                _ => (),
            }
        }

        self.pipeline
            .set_state(gstreamer::State::Null)
            .expect("Unable to stop playing");
    }
}


