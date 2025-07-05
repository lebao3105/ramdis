use crate::{App, CurrentScreen};
use ratatui::prelude::*;
use ratatui::widgets::{Borders, Clear};
use ratatui::{
    Frame,
    widgets::{Block, Paragraph},
};

pub mod editors;
pub mod home;

/// So-called page's skeleton.
pub trait UIPage {
    fn default() -> Self;
    fn ready_ui(&mut self, frame: &mut Frame, choices: &[String], current_screen: &CurrentScreen);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn create_exit_ask_view(frame: &mut Frame, _app: &App) {
    let popup_block = Block::bordered()
        .title("Question")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red));

    let exit_text = Text::styled(
        "Are you sure want to exit the application?\n\
        Press Ctrl-C` again to confirm. `ESC` to cancel.",
        Style::default().fg(Color::White),
    );

    frame.render_widget(
        Paragraph::new(exit_text).block(popup_block),
        centered_rect(60, 25, frame.area()),
    );
}

pub fn draw_ui(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Clear, frame.area());
    let current_screen = app.current_screen.clone();
    match current_screen {
        CurrentScreen::Main | CurrentScreen::Deciding => {
            app.home_page.ready_ui(frame, &app.choices, &current_screen)
        }
        CurrentScreen::Exiting => create_exit_ask_view(frame, app),
        CurrentScreen::Editing | CurrentScreen::Creating => {
            app.edit_page.ready_ui(frame, &app.choices, &current_screen)
        }
    }
}
