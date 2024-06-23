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
            task_list.tasks().iter().map(|t| format!("{t}"))
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

    pub fn toggle_dot(&self, task_list: &mut TaskList) {
        if let Some(current) = self.state.selected() {
            if let Some(task) = task_list.get_mut(current) {
                task.toggle_dot();
            }
        }
    }

    pub fn complete(&self, task_list: &mut TaskList) {
        if let Some(current) = self.state.selected() {
            if let Some(task) = task_list.get_mut(current) {
                task.complete();
            }
        }
    }
}

