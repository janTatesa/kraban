use std::borrow::Cow;

use kraban_state::CurrentItem;

use crate::{Context, ViewTrait};

use super::TasksView;

impl ViewTrait for TasksView<'_> {
    fn current_item(&self) -> CurrentItem {
        let tab = self.focused_tab.value();
        let (column, index) = self.tabs[tab].get_column_and_task_index();
        CurrentItem::Task {
            project: self.project_index,
            column,
            task: index,
        }
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
