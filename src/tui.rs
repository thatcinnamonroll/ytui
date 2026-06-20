use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{Block, BorderType, List, ListState};
use ratatui::Frame;
use std::io::Result;
use crate::tui_helper;

#[derive(PartialEq)]
pub enum focused_block{
    TrackList,
    Playlist,
    Music
}

#[derive(PartialEq)]
pub enum menu_state {
    Playlist,
    Tracklist
}

pub struct app {
    pub playlists: Option<Vec<String>>,
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
                    KeyCode::Char('q') => {tui_helper::switch_focus(app);}
                    KeyCode::Char('w') | KeyCode::Up => {app.tracklist_state.select_previous();}
                    KeyCode::Char('s') | KeyCode::Down => {app.tracklist_state.select_next();}
                    _ => {}

                }
            }else if  app.focused == focused_block::Playlist {
                match key.code {
                    KeyCode::Esc | KeyCode::End => {return Ok(())}
                    KeyCode::Char('e') => {tui_helper::switch_state(app);}
                    KeyCode::Char('q') => {tui_helper::switch_focus(app);}
                    KeyCode::Char('w') | KeyCode::Up => {app.playlist_state.select_previous();}
                    KeyCode::Char('s') | KeyCode::Down => {app.playlist_state.select_next();}
                    _ => {}
                }
            }else {
                match key.code {
                    KeyCode::Esc | KeyCode::End => {return Ok(())}
                    KeyCode::Char('e') => {tui_helper::switch_state(app);}
                    KeyCode::Char('q') => {tui_helper::switch_focus(app);}
                    _ => {}

                }
            }
        }
    })
}

impl app{
    fn render(&mut self,frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
        let [top, bottom] = frame.area().layout(&vertical);

        let mut top_block_style = Style::new();
        let mut music_block_style = Style::new();

        match self.focused {
            focused_block::Playlist | focused_block::TrackList => {top_block_style = Style::new().magenta();}
            focused_block::Music => {music_block_style = Style::new().magenta();}
        }

        match self.state {
            menu_state::Playlist => {Self::render_playlist_block(frame,top,top_block_style,&mut self.playlist_state);}
            menu_state::Tracklist => {Self::render_track_list_block(frame,top,top_block_style, &mut self.tracklist_state);}
            _ => {}
        }
        Self::render_music_progress_bar_block(frame, bottom,music_block_style);
    }

    fn render_track_list_block(frame: &mut Frame, area: Rect,block_style: Style, list_state: &mut ListState) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(block_style)
        .title("TrackList");

        let tracklist = ["song1","song2"];
        let list = List::new(tracklist)
        .highlight_style(Modifier::REVERSED)
        .block(block);

        frame.render_stateful_widget(list, area, list_state);
    }

    fn render_playlist_block(frame: &mut Frame, area: Rect,block_style: Style, list_state: &mut ListState) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(block_style)
        .title("Playlists");

        let playlists = ["playlist1","playlist2"];
        let list = List::new(playlists)
        .highlight_style(Modifier::REVERSED)
        .block(block);

        frame.render_stateful_widget(list, area, list_state);
    }

    fn render_music_progress_bar_block(frame: &mut Frame, area: Rect,block_style: Style) {
        let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(block_style)
        .title("Music");
        frame.render_widget(block, area);
    }

}
