use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Cell, List, Paragraph, Row, Table, TableState};

use rand::prelude::*;
use std::collections::HashMap;

use crate::{CurrentScreen, ui::UIPage};

#[derive(Debug)]
pub struct HomePage {
    layout: Layout,
    rng: ThreadRng,
    table_state: TableState,
    random_results: HashMap<String, u8>,
}

impl UIPage for HomePage {
    fn default() -> Self {
        HomePage {
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)]),
            rng: rand::rng(),
            table_state: TableState::new(),
            random_results: HashMap::new(),
        }
    }

    fn ready_ui(
        &mut self,
        frame: &mut ratatui::Frame,
        choices: &[String],
        current_screen: &CurrentScreen,
    ) {
        let splits = self.layout.split(frame.area());

        {
            let left_title = Line::from("Options").bold().centered();
            let list = List::new(choices.to_vec())
                .white()
                .highlight_style(Style::new().yellow())
                .highlight_symbol("=>")
                .block(Block::bordered().title(left_title));

            frame.render_widget(list, splits[0]);
        }
        // End Left Pane

        {
            let right_title = Line::from("ramdis").bold().centered();

            if *current_screen == CurrentScreen::Deciding {
                let choosen = self
                    .random_results
                    .entry(
                        choices
                            .choose(&mut self.rng)
                            .expect("Error randomizing")
                            .to_string(),
                    )
                    .or_insert(0);
                *choosen += 1;

                let mut rows = vec![];
                for (key, value) in &self.random_results {
                    rows.push(Row::new(vec![key.clone(), value.to_string()]));
                }

                let table = Table::new(
                    rows,
                    [Constraint::Percentage(70), Constraint::Percentage(30)],
                )
                .block(Block::bordered().title(right_title))
                .highlight_symbol(">>")
                .header(
                    ["Name", "Count"]
                        .into_iter()
                        .map(Cell::from)
                        .collect::<Row>()
                        .height(1),
                );
                frame.render_stateful_widget(table, splits[1], &mut self.table_state);
            } else {
                let text = "This is not something related to ram-disk.\n\
                    This. Is. Distro. Hopping. Killer.\n\n\n\
                    Press `Ctrl-C` to quit.\n\
                    Press `Ctrl-R` to start randomizing.\n\
                    Press `n` to create a new choice.\n\
                    Press `e` to edit a choice.\n\n\
                    While in editing/modifying mode:\n\
                    `Ctrl` + `i` to change the item's index;\n\
                    `Ctrl` + `d` to change the choice's data.\n\
                    `Enter` to apply the changes.\n\n\
                    All shortcuts are in-case-sensitive.\n\
                    The list is zero-indexed.";

                frame.render_widget(
                    Paragraph::new(text)
                        .block(Block::bordered().title(right_title))
                        .centered(),
                    splits[1],
                )
            }
        } // End Right Pane
    }
}
