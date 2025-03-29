use crate::app::{
    config::Config,
    state::CurrentList,
    ui::{Action, Item, View},
    Context,
};

use super::TasksView;

impl View for TasksView {
    fn item(&self) -> Item {
        Item::Task
    }

    fn current_list<'a>(&self, config: &'a Config) -> CurrentList<'a> {
        CurrentList::Tasks {
            project: self.project,
            column: &self.get_current_column(config).name,
            index: self.focused_task.focused_item(),
        }
    }

    fn handle_action(&mut self, action: &Action, _context: Context) {
        match action {
            Action::ShrinkList => self.focused_task.decrement_size(),
            Action::New(_) => self.focused_task.increment_size(),
            Action::SwitchToIndex(index) => self.focused_task.switch_to_index(*index),
            _ => {}
        }
    }
}
