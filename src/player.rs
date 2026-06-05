use gstreamer::prelude::*;
use gstreamer::MessageView;

pub fn play(music_file_path: String){
    gstreamer::init().unwrap();

    let _pipeline = gstreamer::parse::launch(&format!("playbin uri=file://{}", music_file_path)).unwrap();

    _pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to start playing");

    let bus = _pipeline.bus().unwrap();
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

    _pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to stop playing");
}

