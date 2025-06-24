use ratatui::{layout::Rect, style::Stylize, widgets::{List}, Frame};

use crate::{TaskList};

#[derive(Default)]
pub struct TaskDoneView {
    last_rendered_area: Option<Rect>
}

impl TaskDoneView {

    /// Renders view to a frame area
    pub fn render(&mut self, frame: &mut Frame, area: Rect, task_list: &TaskList) {
        let filtered_tasks = task_list.tasks_done_today();
        // let count_tasks = filtered_tasks.size();
        // if count_tasks > area.height {
        //     filtered_tasks.take(area.height - count_tasks);
        // }
        let list = List::new(
            filtered_tasks.map(ToString::to_string)
        ).green();
        frame.render_widget(list, area);
        self.last_rendered_area = Some(area);
    }

}
