use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use color_eyre::owo_colors::CssColors::Teal;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tracing::Instrument;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
};
use crate::mode::Mode::Game;
use crate::tui::Event;

#[derive(Default, Clone)]
pub struct Home<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    titles: StatefulList<Text<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum HomeAction {
    Up,
    Down,
    Cancel,
    Into,
}

#[derive(Default, Clone)]
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl Home<'_> {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            titles: StatefulList {
                state: ListState::default(),
                items: vec![Text::raw("New Game"), Text::raw("Load Game"), Text::raw("New Config"), Text::raw("Exist")],
            },
        }
    }
}

impl Component for Home<'_> {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Home(HomeAction::Up) => {
                self.titles.previous()
            }
            Action::Home(HomeAction::Down) => {
                self.titles.next()
            }
            Action::Home(HomeAction::Cancel) => {
                self.titles.unselect()
            }
            _ => {}
        }
        Ok(None)
    }
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up => {
                Ok(Some(Action::Home(HomeAction::Up)))
            }
            KeyCode::Down => {
                Ok(Some(Action::Home(HomeAction::Down)))
            }
            KeyCode::Esc => {
                Ok(Some(Action::Home(HomeAction::Cancel)))
            }
            KeyCode::Enter=> {
                match self.titles.state.selected() {
                    Some(0) => Ok(Some(Action::Mode(Game))),

                    Some(3) => Ok(Some(Action::Quit)),
                    _ => Err(Report::msg("Index out of bound")),
                }

            }
                _ => Ok(None)
        }
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(20), Constraint::default()].as_ref())
            .split(area);


        let items: Vec<ListItem> = self.titles.items.iter().map(|i| ListItem::new(i.clone())).collect();

        let items = List::new(items)
            .block(Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)).highlight_style(
            Style::default()
                .bg(Color::Rgb(255,255,255))
                .fg(Color::Rgb(29, 198, 245))
                .add_modifier(Modifier::BOLD),
        )
            .highlight_symbol(">> ")
            .style(Style::default().bg(Color::Rgb(153, 153, 153)))
            .fg(Color::Rgb(255,255,255));
        f.render_stateful_widget(items, chunks[1], &mut self.titles.state);
        Ok(())
    }
}


