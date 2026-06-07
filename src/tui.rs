use crossterm::event;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType};
use ratatui::Frame;

enum focused_block{
    Playlist,
    TrackList,
    Progress_bar
}

enum menu_state {
    Playlist,
    Tracklist
}

pub fn start_tui(){
    ratatui::run(|terminal| loop {
        terminal.draw(render);
        if event::read().unwrap().is_key_press() {
            break;
        }
    });
}

fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, middle, right] = main.layout(&horizontal);

    let title = Line::from_iter([
        Span::from("Block Widget").bold(),
                                Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_track_list_block(frame, left);
    render_playlist_block(frame, middle);
    render_music_progress_bar_block(frame, right);
}

pub fn render_track_list_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title("Bordered block");
    frame.render_widget(block, area);
}

pub fn render_playlist_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
    .style(Style::new().blue().on_black().bold().italic())
    .title("Styled block");
    frame.render_widget(block, area);
}

pub fn render_music_progress_bar_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
    .border_type(BorderType::Rounded)
    .border_style(Style::new().red())
    .title("Custom borders");
    frame.render_widget(block, area);
}
