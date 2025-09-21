use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

use std::io;
use std::vec::Vec;
use tui_textarea::{Input, Key};

mod ui;
use crate::ui::editors::EditPage;
use crate::ui::home::HomePage;
use crate::ui::{UIPage, draw_ui};

#[allow(unused_must_use)]
fn main() -> io::Result<()> {
    color_eyre::install();

    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);

    ratatui::restore();
    result
}

#[derive(Debug)]
pub struct App<'a> {
    running: bool,
    choices: Vec<String>,

    home_page: HomePage,
    edit_page: EditPage<'a>,

    current_input: String,
    current_index: String,
    current_screen: CurrentScreen,
    current_emode: CurrentEditMode,
    current_curpos: u16,
    current_curypos: u16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
    Creating,
    Deciding,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentEditMode {
    Data,
    Index,
    None,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            running: false,
            choices: Vec::new(),

            home_page: HomePage::default(),
            edit_page: EditPage::default(),

            current_input: String::new(),
            current_index: String::from('0'),
            current_screen: CurrentScreen::Main,
            current_emode: CurrentEditMode::None,
            current_curpos: 0,
            current_curypos: 0,
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| draw_ui(&mut self, frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.on_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match key.into() {
            Input {
                key: Key::Char('c') | Key::Char('C'),
                ctrl: true,
                alt: false,
                shift: false,
            } => self.quit(),
            Input {
                key: Key::Char('r') | Key::Char('R'),
                ctrl: true,
                alt: false,
                shift: false,
            } => {
                self.current_screen = CurrentScreen::Deciding;
            }
            Input {
                key: Key::Char('i') | Key::Char('I'),
                ctrl: true,
                alt: false,
                shift: false,
            } => match self.current_screen {
                CurrentScreen::Creating | CurrentScreen::Editing => {
                    self.current_emode = CurrentEditMode::Index
                }
                _ => self.current_emode = CurrentEditMode::None,
            },
            Input {
                key: Key::Char('d') | Key::Char('D'),
                ctrl: true,
                alt: false,
                shift: false,
            } => match self.current_screen {
                CurrentScreen::Creating | CurrentScreen::Editing => {
                    self.current_emode = CurrentEditMode::Data
                }
                _ => self.current_emode = CurrentEditMode::None,
            },
            Input { key: Key::Esc, .. } => self.switch_to_main_screen(),
            Input {
                key: Key::Char(to_insert),
                ctrl: false,
                alt: false,
                shift: false,
            } => match self.current_emode {
                CurrentEditMode::Data => {
                    self.current_input.push(to_insert);
                    self.edit_page.name_editor_input(Input {
                        key: Key::Char(to_insert),
                        ctrl: false,
                        alt: false,
                        shift: false,
                    });
                }
                CurrentEditMode::Index => {
                    if to_insert.is_numeric() {
                        self.current_index.push(to_insert);
                    }
                }
                CurrentEditMode::None => match to_insert {
                    'e' | 'E' => self.current_screen = CurrentScreen::Editing,
                    'n' | 'N' => self.current_screen = CurrentScreen::Creating,
                    _ => {}
                },
            },
            Input {
                key: Key::Enter, ..
            } => match self.current_screen {
                CurrentScreen::Creating => {
                    self.choices
                        .insert(self.current_index_to_int(), self.current_input.clone());
                    self.switch_to_main_screen();
                }
                CurrentScreen::Editing => {
                    let idx = self.current_index_to_int();
                    self.choices[idx] = self.current_input.clone();
                    self.switch_to_main_screen();
                }
                _ => {}
            },
            input => match self.current_emode {
                CurrentEditMode::Data => self.edit_page.name_editor_input(input),
                CurrentEditMode::Index => self.edit_page.index_editor_input(input),
                _ => {}
            },
        }
    }

    fn quit(&mut self) {
        if let CurrentScreen::Exiting = self.current_screen {
            self.running = false;
        } else if let CurrentScreen::Main = self.current_screen {
            self.current_screen = CurrentScreen::Exiting;
        }
    }

    fn current_index_to_int(&self) -> usize {
        return self
            .current_index
            .parse::<usize>()
            .expect("App.current_index is supposed to be always an integer");
    }

    fn switch_to_main_screen(&mut self) {
        self.current_input.clear();
        self.current_index = String::from('0');
        self.current_curpos = 0;
        self.current_curypos = 0;
        self.current_emode = CurrentEditMode::None;
        self.current_screen = CurrentScreen::Main;
    }
}
