use std::iter;

use itertools::chain;
use kraban_config::Config;
use kraban_state::{Difficulty, State, Task};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::Stylize,
    text::Line
};
use strum::{EnumCount, IntoEnumIterator};

use crate::{
    keyhints::Keyhints,
    list::{List, ListQuery},
    prompt::Prompt,
    utils::difficulty_to_color
};

pub enum Response {
    Update(DifficultyPrompt),
    ModifyCurrentlyCreatedTask(Task),
    SetDifficulty(Option<Difficulty>)
}

pub struct DifficultyPrompt {
    list: List<DifficultyListQuery>,
    currently_creating: Option<Task>
}

impl DifficultyPrompt {
    pub fn new(currently_creating: Option<Task>) -> Self {
        let list = List::new(DifficultyListQuery);
        Self {
            list,
            currently_creating
        }
    }

    pub fn on_key(self, key: KeyEvent) -> Response {
        const NONE: KeyModifiers = KeyModifiers::NONE;
        match (key.code, self.currently_creating, self.list, key.modifiers) {
            (KeyCode::Enter, currently_creating, list, NONE) => {
                let priority = Difficulty::iter().nth(list.selected());
                Self::difficulty_selected(currently_creating, priority)
            }
            (KeyCode::Backspace, currenty_creating, _, NONE) => {
                Self::difficulty_selected(currenty_creating, None)
            }
            (_, currently_creating, mut list, _) => {
                list.on_key(key);
                Response::Update(Self {
                    list,
                    currently_creating
                })
            }
        }
    }

    fn difficulty_selected(
        currently_creating: Option<Task>,
        difficulty: Option<Difficulty>
    ) -> Response {
        match currently_creating {
            Some(mut task) => {
                task.difficulty = difficulty;
                Response::ModifyCurrentlyCreatedTask(task)
            }
            None => Response::SetDifficulty(difficulty)
        }
    }
}

impl Prompt for DifficultyPrompt {
    fn height(&self, _: &State, _: &Config) -> u16 { Difficulty::COUNT as u16 }
    fn title(&self) -> &'static str { "Change difficulty" }
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        self.list.render(area, buf, state, config);
    }
}

#[derive(Debug)]
struct DifficultyListQuery;
impl Keyhints for DifficultyPrompt {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        chain![
            self.list.keyhints(state, config),
            Difficulty::iter().map(|difficulty| {
                let str: &str = difficulty.into();
                (&str[0..1], str)
            }),
            iter::once(("Backspace", "None"))
        ]
    }
}

impl ListQuery for DifficultyListQuery {
    fn get_items<'a>(&self, _: &'a State, _: &'a Config) -> impl Iterator<Item = Line<'a>> {
        Difficulty::iter().map(|difficulty| {
            let str: &str = difficulty.into();
            Line::raw(str).fg(difficulty_to_color(difficulty))
        })
    }
}
