use kraban_config::Config;
use kraban_state::CurrentList;

use crate::{Context, Item, ViewTrait};

use super::MainView;

impl ViewTrait for MainView {
    fn item(&self) -> Item {
        Item::Project
    }

    fn current_list<'a>(&self, _config: &'a Config) -> CurrentList<'a> {
        match self {
            MainView::Projects(list_state) => CurrentList::Projects(list_state.focused_item()),
            MainView::DueTasks(list_state) => CurrentList::DueTasks(list_state.focused_item()),
        }
    }

    fn refresh_on_state_change(&mut self, context: Context) {
        let max_index = match self {
            MainView::Projects(_) => context.state.projects().len(),
            MainView::DueTasks(_) => context.state.due_tasks().len(),
        }
        .checked_sub(1);

        self.list_state_mut().update_max_index(max_index)
    }

    fn switch_to_index(&mut self, index: usize) {
        self.list_state_mut().switch_to_index(index);
    }

    fn title(&self, _context: Context) -> String {
        "Projects".to_string()
    }

    fn right_title(&self) -> Option<&'static str> {
        Some("Due tasks")
    }
}
