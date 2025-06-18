use std::borrow::Cow;

use kraban_state::CurrentList;

use crate::{Context, Item, ViewTrait};

use super::{FocusedList, MainView};

impl ViewTrait for MainView {
    fn item(&self) -> Item {
        match self.focused_list {
            FocusedList::Projects => Item::Project,
            FocusedList::DueTasks => Item::Task,
        }
    }

    fn current_list(&self) -> CurrentList {
        match self.focused_list {
            FocusedList::Projects => CurrentList::Projects(self.projects.selected()),
            FocusedList::DueTasks => CurrentList::DueTasks(self.projects.selected()),
        }
    }

    fn refresh_max_indexes(&mut self, context: Context) {
        match self.focused_list {
            FocusedList::Projects => self.projects.update_max_index(context),
            FocusedList::DueTasks => self.due_tasks.update_max_index(context),
        }
    }

    fn switch_to_index(&mut self, index: usize) {
        match self.focused_list {
            FocusedList::Projects => self.projects.select(index),
            FocusedList::DueTasks => self.due_tasks.select(index),
        }
    }

    fn title(&self, _context: Context) -> Cow<'static, str> {
        "Projects".into()
    }

    fn right_title(&self) -> Option<&'static str> {
        Some("Due tasks")
    }
}
