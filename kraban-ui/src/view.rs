use crossterm::event::KeyEvent;
use enum_dispatch::enum_dispatch;
use kraban_config::Config;
use kraban_state::CurrentList;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{Action, Component, Context, keyhints::KeyHints, main_view::MainView, task::TasksView};

#[enum_dispatch]
pub(crate) trait ViewTrait {
    fn item(&self) -> Item;
    fn title(&self, context: Context) -> String;
    fn right_title(&self) -> Option<&'static str>;
    fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a>;
    fn refresh_on_state_change(&mut self, context: Context);
    fn switch_to_index(&mut self, index: usize);
}

#[derive(strum_macros::Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Item {
    Project,
    Task,
}

#[enum_dispatch(ViewTrait, Component)]
#[derive(Debug)]
pub enum View {
    MainView,
    TasksView,
}
