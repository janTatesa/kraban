use std::iter;

use itertools::chain;
use kraban_config::Config;
use kraban_state::{Priority, SetPriority, State};
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
    utils::priority_to_color
};

pub enum Response<T> {
    Update(PriorityPrompt<T>),
    ModifyCurrentlyCreatedItem(T),
    SetPriority(Option<Priority>)
}

pub struct PriorityPrompt<T> {
    list: List<PriorityListQuery>,
    currently_creating: Option<T>
}

impl<T: SetPriority> PriorityPrompt<T> {
    pub fn new(currently_creating: Option<T>) -> Self {
        let list = List::new(PriorityListQuery);
        Self {
            list,
            currently_creating
        }
    }

    pub fn on_key(self, key: KeyEvent, config: &Config) -> Response<T> {
        const NONE: KeyModifiers = KeyModifiers::NONE;
        match (key.code, self.currently_creating, self.list, key.modifiers) {
            (KeyCode::Enter, currently_creating, list, NONE) => {
                let priority = Priority::iter().nth(list.selected());
                Self::priority_selected(currently_creating, priority, config)
            }
            (KeyCode::Backspace, currenty_creating, _, NONE) => {
                Self::priority_selected(currenty_creating, None, config)
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

    fn priority_selected(
        currently_creating: Option<T>,
        priority: Option<Priority>,
        config: &Config
    ) -> Response<T> {
        match currently_creating {
            Some(mut item) => {
                item.set_priority(priority, config);
                Response::ModifyCurrentlyCreatedItem(item)
            }
            None => Response::SetPriority(priority)
        }
    }
}

impl<T> Prompt for PriorityPrompt<T> {
    fn height(&self, _: &State, _: &Config) -> u16 { Priority::COUNT as u16 }
    fn title(&self) -> &'static str { "Change priority" }
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &State, config: &Config) {
        self.list.render(area, buf, state, config);
    }
}

#[derive(Debug)]
struct PriorityListQuery;

impl<T> Keyhints for PriorityPrompt<T> {
    fn keyhints(&self, state: &State, config: &Config) -> impl IntoIterator<Item = (&str, &str)> {
        chain![
            self.list.keyhints(state, config),
            Priority::iter().map(|priority| {
                let str: &str = priority.into();
                (&str[0..1], str)
            }),
            iter::once(("Backspace", "None"))
        ]
    }
}

impl ListQuery for PriorityListQuery {
    fn get_items<'a>(&self, _: &'a State, _: &'a Config) -> impl Iterator<Item = Line<'a>> {
        Priority::iter().map(|priority| {
            let str: &str = priority.into();
            Line::raw(str).fg(priority_to_color(priority))
        })
    }
}
