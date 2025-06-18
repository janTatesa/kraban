use std::borrow::Cow;

use kraban_state::CurrentList;

use crate::{Context, Item, ViewTrait};

use super::TasksView;

impl ViewTrait for TasksView {
    fn item(&self) -> Item {
        Item::Task
    }

    fn current_list(&self) -> CurrentList {
        let tab = self.focused_tab.value();
        let (column, index) = self.tabs[tab].get_column_and_task_index();
        CurrentList::Tasks {
            project: self.project_index,
            column,
            index,
        }
    }

    fn refresh_max_indexes(&mut self, context: Context) {
        let tab = self.focused_tab.value();
        self.tabs[tab].update_column_max_index(context);
    }

    fn switch_to_index(&mut self, index: usize) {
        self.tabs[self.focused_tab.value()].set_task_index(index);
    }

    fn title(&self, context: Context) -> Cow<'static, str> {
        format!(
            "Tasks in {}",
            context.state.projects()[self.project_index].title
        )
        .into()
    }

    fn right_title(&self) -> Option<&'static str> {
        None
    }
}
