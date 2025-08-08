use ratatui::prelude::Stylize;

#[derive(Default)]
pub struct TaskNextView {
    last_rendered_area: Option<ratatui::layout::Rect>,
}

impl TaskNextView {
    #[must_use]
    pub fn new() -> Self {
        Self { last_rendered_area: None }
    }

    /// Renders view to a frame area
    pub fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, task_list: &crate::TaskList) {
        let text = if let Some (task) = task_list.last_dotted_task() {
            format!("Next: {}", task.description())
        } else {
            "No Next Task".to_string()
        };
        let block = ratatui::widgets::Block::new()
                .title(text.light_yellow())
                .borders(ratatui::widgets::Borders::TOP);
        frame.render_widget(block, area);
        self.last_rendered_area = Some(area);
    }
}