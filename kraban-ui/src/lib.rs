mod action;
mod component;
mod context;
mod keyhints;
mod list;
mod main_view;
mod prompt;
mod table;
mod task;
mod view;
mod widgets;

use action::StateAction;
use arrayvec::ArrayVec;
use cli_log::error;
use component::Component;
pub use context::Context;
use crossterm::event::{KeyCode, KeyEvent};
use derivative::Derivative;
use kraban_state::CurrentList;
use main_view::MainView;
use prompt::Prompt;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
};
use std::fmt::Debug;
use time::{Date, OffsetDateTime};
use view::{Item, View, ViewTrait};

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Ui {
    #[derivative(Default(value = "MainView::default().into()"))]
    view: View,
    prompt_stack: ArrayVec<Prompt, 4>,
}

impl Ui {
    // Context cannot be used because state would be referenced both mutably and immutably
    pub fn current_list(&self) -> CurrentList {
        self.view.current_list()
    }

    pub fn redraw(&self, area: Rect, buf: &mut Buffer, context: Context) {
        self.render(area, buf, context, true);
    }

    pub fn in_main_view(&self) -> bool {
        matches!(self.view, View::MainView(_))
    }

    pub fn switch_to_index(&mut self, index: usize) {
        self.view.switch_to_index(index);
    }

    pub fn refresh_list_max_indexes(&mut self, context: Context) {
        self.view.refresh_max_indexes(context);
    }
}

trait KeyNoModifiers {
    fn keycode_without_modifiers(self) -> Option<KeyCode>;
}

impl KeyNoModifiers for KeyEvent {
    fn keycode_without_modifiers(self) -> Option<KeyCode> {
        self.modifiers.is_empty().then_some(self.code)
    }
}

pub fn date_to_line(date: Date) -> Line<'static> {
    let duration = date
        - OffsetDateTime::now_local()
            .inspect_err(|e| error!("Failed to get local timezone {e}, using utc"))
            .unwrap_or(OffsetDateTime::now_utc())
            .date();

    let color = match duration.whole_days() {
        ..0 => Color::Red,
        0 => Color::Yellow,
        1..7 => Color::Green,
        7..30 => Color::Blue,
        _ => Color::Magenta,
    };

    date.to_string().fg(color).underlined().into()
}

fn no_property() -> Line<'static> {
    "None".italic().dim().into()
}
