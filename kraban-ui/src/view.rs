use std::borrow::Cow;

use crossterm::event::KeyEvent;
use enum_dispatch::enum_dispatch;
use kraban_state::CurrentList;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    Component, Context, action::Action, keyhints::KeyHints, main_view::MainView, task::TasksView,
};

#[enum_dispatch]
pub(crate) trait ViewTrait {
    fn item(&self) -> Item;
    fn title(&self, context: Context) -> Cow<'static, str>;
    fn right_title(&self) -> Option<&'static str>;
    fn current_list(&self) -> CurrentList;
    fn refresh_max_indexes(&mut self, context: Context);
    fn switch_to_index(&mut self, index: usize);
}

#[derive(strum_macros::IntoStaticStr, Debug, Clone, Copy, PartialEq, Eq)]
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
