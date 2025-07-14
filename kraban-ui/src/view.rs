use std::borrow::Cow;

use enum_dispatch::enum_dispatch;
use kraban_state::CurrentItem;
use ratatui::crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    Component, Context, action::Action, keyhints::KeyHints, main_view::MainView, task::TasksView,
};

#[enum_dispatch]
pub(crate) trait ViewTrait {
    fn title(&self, context: Context) -> Cow<'static, str>;
    fn right_title(&self) -> Option<&'static str>;
    fn current_item(&self) -> CurrentItem;
}

#[enum_dispatch(ViewTrait, Component)]
#[derive(Debug)]
pub enum View<'a> {
    MainView,
    TasksView(TasksView<'a>),
}

impl Default for View<'_> {
    fn default() -> Self {
        Self::MainView(MainView::with_focused_project(0))
    }
}
