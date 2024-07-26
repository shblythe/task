use ratatui::{layout::{Constraint, Direction, Layout, Rect}, widgets::{List, ListState}, Frame};
use uuid::Uuid;

use crate::{helpview, taskdetailview, Task, TaskList};

pub struct TaskListView {
    state: ListState,
    selected_uuid: Option<Uuid>,
    details_pane: bool,
    help_pane: bool,
    future_filter:bool
}

impl Default for TaskListView  {
    fn default() -> Self {
        TaskListView {
            state: ListState::default(),
            selected_uuid: Option::default(),
            details_pane: bool::default(),
            help_pane: bool::default(),
            future_filter: true
        }
    }
}

impl TaskListView {

    /// Processes data before rendering.
    /// MUST be called before any call to ``render()``
    ///
    /// # Errors
    /// Returns an error if write to disk failed
    pub fn pre_render(&mut self, task_list: &mut TaskList) -> std::io::Result<()> {
        task_list.pre_render()
    }

    /// Renders view to a frame area
    pub fn render(&mut self, frame: &mut Frame, area: Rect, task_list: &TaskList) {
        let panes = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Min(0),
                Constraint::Percentage(if self.details_pane { 30 } else { 0 }),
                Constraint::Length(if self.help_pane { helpview::WIDTH } else { 0 }),
            ]
            ).split(area);
        let mut filtered_tasks = task_list.filtered_tasks(self.future_filter).peekable();
        if self.state.selected().is_none() && filtered_tasks.peek().is_some() {
            if let Some(pos) = task_list.filtered_tasks(self.future_filter).rev().position(Task::dot) {
                self.select(task_list, task_list.filtered_tasks(self.future_filter).count()-1 - pos);
            } else {
                self.select(task_list, 0);
            }
        }
        let list = List::new(
            filtered_tasks.map(ToString::to_string)
        ).highlight_symbol(">> ");
        frame.render_stateful_widget(list, panes[0], &mut self.state);
        taskdetailview::render(frame, panes[1], self.selected_uuid(), task_list);
        helpview::render(frame, panes[2]);
    }

    #[must_use]
    pub fn selected_uuid(&self) -> Option<Uuid> {
        self.selected_uuid
    }

    fn select(&mut self, task_list: &TaskList, index: usize) {
        if let Some(task) = task_list.filtered_tasks(self.future_filter).nth(index) {
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
        let last_index = task_list.filtered_tasks(self.future_filter).count() - 1;
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
            if current < task_list.filtered_tasks(self.future_filter).count()-1 {
                self.select(task_list, current + 1);
            }
        }
    }

    pub fn move_start(&mut self, task_list: &TaskList) {
        self.select(task_list, 0);
    }

    pub fn move_end(&mut self, task_list: &TaskList) {
        self.select(task_list, task_list.filtered_tasks(self.future_filter).count()-1);
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
                if task.dot() {
                    task_list.replace(selected_uuid, task)?;
                } else {
                    // undotted task, so move to bottom
                    task_list.replace_at_bottom(selected_uuid, task)?;
                }
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


    /// Set the currently selected task to recur daily, and attempts to
    /// write the updated task list to storage.
    /// Silently ignores failures caused by the lack of a valid current task.
    ///
    /// If the currently selected task is already set to recur, removes recurrence
    ///
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails.
    pub fn recur_daily(&mut self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(selected_uuid) = self.selected_uuid {
            if let Some(task) = task_list.get(selected_uuid) {
                let mut task = task.clone();
                if task.is_recurring() {
                    task.clear_recur();
                } else {
                    task.set_recur_daily();
                }
                task_list.replace(selected_uuid, task)?;
                self.fix_selection(task_list);
            }
        }
        Ok(())
    }

    /// Snoozes the currently selected task until tomorrow, and attempts to
    /// write the updated task list to storage.
    /// Silently ignores failures caused by the lack of a valid current task.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails.
    pub fn snooze_tomorrow(&mut self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(selected_uuid) = self.selected_uuid {
            if let Some(task) = task_list.get(selected_uuid) {
                let mut task = task.clone();
                task.snooze_tomorrow();
                task_list.replace(selected_uuid, task)?;
                self.fix_selection(task_list);
            }
        }
        Ok(())
    }

    /// Snoozes the currently selected task for 1s, and attempts to
    /// write the updated task list to storage.
    /// Silently ignores failures caused by the lack of a valid current task.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails.
    pub fn snooze_1s(&mut self, task_list: &mut TaskList) -> std::io::Result<()> {
        if let Some(selected_uuid) = self.selected_uuid {
            if let Some(task) = task_list.get(selected_uuid) {
                let mut task = task.clone();
                task.snooze_1s();
                task_list.replace(selected_uuid, task)?;
                self.fix_selection(task_list);
            }
        }
        Ok(())
    }

    pub fn toggle_details_pane(&mut self) {
        self.details_pane = !self.details_pane;
    }

    pub fn toggle_help_pane(&mut self) {
        self.help_pane = !self.help_pane;
    }

    pub fn toggle_future_filter(&mut self, task_list: &TaskList) {
        self.future_filter = !self.future_filter;
        self.fix_selection(task_list);
    }

}

