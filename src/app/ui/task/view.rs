use crate::app::{
    Context,
    config::Config,
    state::CurrentList,
    ui::{Item, View},
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

    fn refresh_on_state_change(&mut self, context: Context) {
        self.focused_task
            .update_max_index(self.get_current_column_len(context).checked_sub(1));
    }

    fn switch_to_index(&mut self, index: usize) {
        self.focused_task.switch_to_index(index);
    }

    fn title(&self, context: Context) -> String {
        format!("Tasks in {}", context.state.projects()[self.project].title)
    }
}
