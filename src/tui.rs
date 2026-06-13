use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType};
use ratatui::Frame;
use std::io::Result;

pub enum focused_block{
    Playlist,
    TrackList,
    Music
}

pub enum menu_state {
    Playlist,
    Tracklist
}

pub struct app {
    pub playlists: Option<Vec<String>>,
    pub tracklist: Option<Vec<crate::playlists::song>>,
    pub focused: focused_block,
    pub state: menu_state,
}

pub fn start_tui(app: app) -> Result<()>{
    ratatui::run(|terminal| loop {

        terminal.draw(|frame| app::render(frame));

        if let Some(key) = event::read()?.as_key_press_event() {
            match app.focused {
                 focused_block::Music => match key.code {

                    KeyCode::Char('q') => {return Ok(())}
                    _ => {}

            }
            _ => {}
            }

        }
    })
}

impl app{
    fn render(frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
        let [top, bottom] = frame.area().layout(&vertical);

        Self::render_track_list_block(frame, top);
        // Self::render_playlist_block(frame, top);
        Self::render_music_progress_bar_block(frame, bottom);
    }

    fn render_track_list_block(frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("TrackList");
        frame.render_widget(block, area);
    }

    fn render_playlist_block(frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Playlists");
        frame.render_widget(block, area);
    }

    fn render_music_progress_bar_block(frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().magenta())
        .title("Music");
        frame.render_widget(block, area);
    }

}
