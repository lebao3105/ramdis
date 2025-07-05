use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Block;
use tui_textarea::{Input, TextArea};

use crate::{CurrentScreen, ui::UIPage};

#[derive(Debug)]
pub struct EditPage<'a> {
    layout: Layout,
    name_editor: TextArea<'a>,
    index_editor: TextArea<'a>,
}

impl<'a> UIPage for EditPage<'a> {
    fn default() -> Self {
        EditPage {
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)]),
            name_editor: TextArea::new(vec!["".to_string()]),
            index_editor: TextArea::new(vec!["0".to_string()]),
        }
    }

    fn ready_ui(
        &mut self,
        frame: &mut ratatui::Frame,
        _choices: &[String],
        _current_screen: &CurrentScreen,
    ) {
        let splits = self.layout.split(frame.area());

        {
            let left_title = Line::from("Name").bold().centered();
            self.name_editor
                .set_block(Block::bordered().title(left_title));
            frame.render_widget(&self.name_editor, splits[0]);
        } // End left pane

        {
            let right_title = Line::from("Index").bold().centered();
            self.index_editor
                .set_block(Block::bordered().title(right_title));
            frame.render_widget(&self.index_editor, splits[1]);
        } // End right pane
    }
}

impl EditPage<'_> {
    pub fn name_editor_input(&mut self, input: impl Into<Input>) {
        self.name_editor.input(input);
    }

    pub fn index_editor_input(&mut self, input: impl Into<Input>) {
        self.index_editor.input(input);
    }
}
