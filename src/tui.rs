use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Modifier, Color};
use ratatui::widgets::{Block, BorderType, List, ListState, Gauge, Paragraph};
use ratatui::Frame;
use std::io::Result;
use crate::{playlists, tui_helper};

#[derive(PartialEq)]
pub enum focused_block{
    TrackList,
    Playlist
}

#[derive(PartialEq)]
pub enum menu_state {
    Playlist,
    Tracklist
}

pub struct app {
    pub playlists: Vec<String>,
    pub playlist_man: playlists::playlist_helper,
    pub playlist_state: ListState,
    pub tracklist: Option<Vec<crate::playlists::song>>,
    pub tracklist_state: ListState,
    pub focused: focused_block,
    pub state: menu_state,
}

pub fn start_tui(app: &mut app) -> Result<()>{
    ratatui::run(|terminal| loop {
        terminal.draw(|frame| app::render(app,frame));
        // keyboard input parser
        if let Some(key) = event::read()?.as_key_press_event() {
            if app.focused == focused_block::TrackList{
                match key.code {
                    KeyCode::Esc | KeyCode::End => {return Ok(())}
                    KeyCode::Char('e') => {tui_helper::switch_state(app);}
                    KeyCode::Char('w') | KeyCode::Up => {app.tracklist_state.select_previous();}
                    KeyCode::Char('s') | KeyCode::Down => {app.tracklist_state.select_next();}
                    _ => {}
                }
            }else{
                match key.code {
                    KeyCode::Esc | KeyCode::End => {return Ok(())}
                    KeyCode::Char('e') => {tui_helper::switch_state(app);}
                    KeyCode::Char('w') | KeyCode::Up => {app.playlist_state.select_previous();}
                    KeyCode::Char('s') | KeyCode::Down => {app.playlist_state.select_next();}
                    // KeyCode::Enter => {tui_helper::open_playlist(app);}
                    _ => {}
                }
            }
        }
    })
}

impl app{
    fn render(&mut self,frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Percentage(20),Constraint::Percentage(80)]);
        let [top, bottom] = frame.area().layout(&vertical);

        match self.state {
            menu_state::Playlist => {Self::render_playlist_block(frame,bottom,self);}
            menu_state::Tracklist => {Self::render_track_list_block(frame,bottom, &mut self.tracklist_state);}
            _ => {}
        }
        Self::render_music_progress_bar_block(frame, top);
    }

    fn render_track_list_block(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().magenta())
        .title("TrackList");

        let tracklist = ["song1","song2"];
        let list = List::new(tracklist)
        .highlight_style(Modifier::REVERSED)
        .block(block);

        frame.render_stateful_widget(list, area, list_state);
    }

    fn render_playlist_block(frame: &mut Frame, area: Rect, app: &mut app) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().magenta())
        .title("Playlists");

        tui_helper::refresh_playlists(app);

        let playlist = app.playlists.clone();

        let list = List::new(playlist)
        .highlight_style(Modifier::REVERSED)
        .block(block);

        frame.render_stateful_widget(list, area, &mut app.playlist_state);
    }

    fn render_music_progress_bar_block(frame: &mut Frame, area: Rect,) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Music");

        frame.render_widget(block,area);

        let chunks = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .margin(1)
        .split(area);

        let progress_bar = Gauge::default()
            .percent(20)
            .label("")
            .gauge_style(Style::new().magenta().on_black());

        let music_title_label = Paragraph::new("Some cool song");

        let music_artist_label = Paragraph::new("Very good artist");

        frame.render_widget(music_title_label,chunks[0]);
        frame.render_widget(music_artist_label,chunks[1]);
        frame.render_widget(progress_bar,chunks[2]);
    }

}
