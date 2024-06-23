use ratatui::{layout::Rect, widgets::{List, ListState}, Frame};
use uuid::Uuid;

use crate::{Task, TaskList};

#[derive(Default)]
pub struct TaskListView {
    state: ListState,
    selected_uuid: Option<Uuid>
}

impl TaskListView {
    /// Renders view to a frame area
    pub fn render(&mut self, frame: &mut Frame, area: Rect, task_list: &TaskList) {
        let mut filtered_tasks = task_list.filtered_tasks().peekable();
        if self.state.selected().is_none() && filtered_tasks.peek().is_some() {
            if let Some(pos) = task_list.filtered_tasks().rev().position(Task::dot) {
                self.select(task_list, task_list.filtered_tasks().count()-1 - pos);
            } else {
                self.select(task_list, 0);
            }
        }
        let list = List::new(
            filtered_tasks.map(ToString::to_string)
        ).highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    #[must_use]
    pub fn selected_uuid(&self) -> Option<Uuid> {
        self.selected_uuid
    }

    fn select(&mut self, task_list: &TaskList, index: usize) {
        if let Some(task) = task_list.filtered_tasks().nth(index) {
            self.selected_uuid = Some(task.uuid());
        } else {
            self.selected_uuid = None;
        }
        self.state.select(Some(index));
    }

    /// Fix the selection after a change in a task, or filtering, by attempting
    /// to re-select the current index, and defaulting to the bottom task
    /// otherwise.
    fn fix_selection(&mut self, task_list: &TaskList) {
        let last_index = task_list.filtered_tasks().count() - 1;
        let index = usize::min(self.state.selected().unwrap_or(last_index), last_index);
        self.select(task_list, index);
    }

    pub fn move_up(&mut self, task_list: &TaskList) {
        if let Some(current) = self.state.selected() {
            if current > 0 {
                self.select(task_list, current - 1);
            }
        }
    }

    pub fn move_down(&mut self, task_list: &TaskList) {
        if let Some(current) = self.state.selected() {
            if current < task_list.filtered_tasks().count()-1 {
                self.select(task_list, current + 1);
            }
        }
    }

    /// Toggles the 'dot' on the currently selected task, and attempt to
    /// write the updated task list to storage.
    /// Silently ignores failures caused by the lack of a valid current task.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails.
    pub fn toggle_dot(&mut self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(selected_uuid) = self.selected_uuid {
            if let Some(task) = task_list.get(selected_uuid) {
                let mut task = task.clone();
                task.toggle_dot();
                task_list.replace(selected_uuid, task)?;
                self.fix_selection(task_list);
            }
        }
        Ok(())
    }

    /// Sets the currently selected task to completed, and attempts to
    /// write the updated task list to storage.
    /// Silently ignores failures caused by the lack of a valid current task.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails.
    pub fn complete(&mut self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(selected_uuid) = self.selected_uuid {
            if let Some(task) = task_list.get(selected_uuid) {
                let mut task = task.clone();
                task.complete();
                task_list.replace(selected_uuid, task)?;
                self.fix_selection(task_list);
            }
        }
        Ok(())
    }
}

