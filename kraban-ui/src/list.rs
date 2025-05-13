use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, widgets::ListState as RatatuiListState};

use crate::Context;

use super::{Action, Component, keyhints::KeyHints};

#[derive(Debug, Default, Clone, Copy)]
pub struct ListState(Option<WrappingUsize>);
impl ListState {
    pub fn new(max_index: Option<usize>) -> Self {
        ListState(max_index.map(WrappingUsize::new))
    }

    pub const fn with_default_index(default_index: usize, max_index: usize) -> Self {
        ListState(Some(WrappingUsize {
            value: default_index,
            max: max_index,
        }))
    }

    pub fn focused_item(&self) -> Option<usize> {
        self.0.map(|wrapping| wrapping.value)
    }

    pub fn update_max_index(&mut self, max_index: Option<usize>) {
        self.0 = max_index.map(|max_index| WrappingUsize {
            max: max_index,
            value: (self
                .0
                .map(|wrapping_usize| wrapping_usize.value)
                .unwrap_or_default())
                % (max_index + 1),
        });
        info!("{:?}", self)
    }

    pub fn switch_to_index(&mut self, index: usize) {
        self.0 = self.0.map(|wrapping_usize| {
            if wrapping_usize.max < index {
                panic!("There should always be a correct index passed");
            }
            WrappingUsize {
                value: index,
                ..wrapping_usize
            }
        })
    }
}

impl Component for ListState {
    fn on_key(&mut self, key_event: KeyEvent, _context: Context) -> Option<Action> {
        match key_event.code {
            KeyCode::Up => {
                self.0 = self.0.map(|num| num.decrement());
                None
            }
            KeyCode::Down => {
                self.0 = self.0.map(|num| num.increment());
                None
            }
            _ => None,
        }
    }

    fn key_hints(&self, _context: Context) -> KeyHints {
        vec![("Up/Down", "Select previous/next")]
    }

    fn render(&self, _area: Rect, _buf: &mut Buffer, _context: Context) {
        unimplemented!()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WrappingUsize {
    value: usize,
    max: usize,
}

impl WrappingUsize {
    pub const fn new(max: usize) -> Self {
        Self::with_value(0, max)
    }

    pub const fn with_value(value: usize, max: usize) -> Self {
        Self { value, max }
    }

    #[must_use = "Method takes self"]
    pub const fn increment(self) -> Self {
        Self {
            value: (self.value + 1) % (self.max + 1),
            ..self
        }
    }

    #[must_use = "Method takes self"]
    pub const fn decrement(self) -> Self {
        Self {
            value: if self.value == 0 {
                self.max
            } else {
                self.value - 1
            },
            ..self
        }
    }

    pub fn max(&self) -> usize {
        self.max
    }
}

impl From<WrappingUsize> for RatatuiListState {
    fn from(value: WrappingUsize) -> Self {
        RatatuiListState::default().with_selected(Some(value.into()))
    }
}

impl From<WrappingUsize> for usize {
    fn from(value: WrappingUsize) -> Self {
        value.value
    }
}

impl From<ListState> for RatatuiListState {
    fn from(value: ListState) -> Self {
        RatatuiListState::default().with_selected(value.focused_item())
    }
}
