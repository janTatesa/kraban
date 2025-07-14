use std::borrow::Cow;

use kraban_state::CurrentItem;

use crate::{Context, ViewTrait};

use super::{FocusedList, MainView};

impl ViewTrait for MainView {
    fn current_item(&self) -> CurrentItem {
        match self.focused_list {
            FocusedList::Projects => CurrentItem::Project(self.projects.selected()),
            FocusedList::DueTasks => CurrentItem::DueTask(self.projects.selected()),
        }
    }

    fn title(&self, _context: Context) -> Cow<'static, str> {
        "Projects".into()
    }

    fn right_title(&self) -> Option<&'static str> {
        Some("Due tasks")
    }
}
