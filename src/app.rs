use std::fs::File;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use serde::Deserialize;

use crate::{ui::render_main_menu, Result};

#[derive(Debug, Clone, Default)]
pub enum MenuState {
    #[default]
    Main,
    InGame,
    Options,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonMenus {
    #[serde(rename = "Menu")]
    pub menu: Vec<String>,
    #[serde(rename = "InGame")]
    pub in_game: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum GameState {
    Menu(MenuState),
    Story,
    Combat,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu(MenuState::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub state: GameState,
    pub choices: Vec<String>,
    pub selected: usize,

    pub should_quit: bool,
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.state {
            GameState::Menu(ref menu) => match menu {
                MenuState::Main => render_main_menu(&self.choices, self.selected, area, buf),
                _ => (),
            },
            _ => (),
        }
    }
}

impl Game {
    pub fn load_menu_by_state(&mut self) -> Result<()> {
        let file = File::open("res/choices.json")?;
        let choices: JsonMenus = serde_json::from_reader(file)?;
        match self.state {
            GameState::Menu(MenuState::Main) => {
                self.choices = choices.menu;
                self.selected = 0;
            }
            GameState::Menu(MenuState::InGame) => {
                self.choices = choices.menu;
                self.selected = 0;
            }
            _ => (),
        }
        Ok(())
    }
    pub fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('w') | KeyCode::Up => self.selected = self.selected.saturating_sub(1),
                KeyCode::Char('s') | KeyCode::Down => {
                    if self.selected < self.choices.len() - 1 {
                        self.selected = self.selected.saturating_add(1);
                    }
                }
                KeyCode::Enter => {}
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    pub fn draw(&self, f: &mut Frame) {
        f.render_widget(self, f.area());
    }

    pub fn handle_choice(&mut self) -> Result<()> {
        let choise = self.choices[self.selected].clone();

        Ok(())
    }
}
