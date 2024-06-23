use ratatui::{layout::Rect, widgets::{List, ListState}, Frame};

use crate::TaskList;

#[derive(Default)]
pub struct TaskListView {
    state: ListState
}

impl TaskListView {
    /// Renders view to a frame area
    pub fn render(&mut self, frame: &mut Frame, area: Rect, task_list: &TaskList) {
        if self.state.selected().is_none() && !task_list.tasks().is_empty() {
            self.state.select(Some(0));
        }
        let list = List::new(
            task_list.tasks().iter().map(ToString::to_string)
        ).highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn move_up(&mut self) {
        if let Some(current) = self.state.selected() {
            if current > 0 {
                self.state.select(Some(current - 1));
            }
        }
    }

    pub fn move_down(&mut self, task_list: &TaskList) {
        if let Some(current) = self.state.selected() {
            if current < task_list.tasks().len()-1 {
                self.state.select(Some(current + 1));
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
    pub fn toggle_dot(&self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(current) = self.state.selected() {
            if let Some(task) = task_list.get(current) {
                let mut task = task.clone();
                task.toggle_dot();
                task_list.replace(current, task)?;
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
    pub fn complete(&self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(current) = self.state.selected() {
            if let Some(task) = task_list.get(current) {
                let mut task = task.clone();
                task.complete();
                task_list.replace(current, task)?;
            }
        }
        Ok(())
    }
}

