use ratatui::{layout::Rect, widgets::{Block, Borders, Paragraph, Wrap}, Frame};
use uuid::Uuid;

use crate::TaskList;

pub fn render(frame: &mut Frame, area: Rect, task_uuid: Option<Uuid>, task_list: &TaskList) {
    let task = if let Some(task_uuid) = task_uuid {
        task_list.get(task_uuid)
    } else {
        None
    };
    let task_text = if let Some(task) = task {
        Paragraph::new(task.detail_string()).wrap(Wrap { trim: false })
    } else {
        Paragraph::new("Invalid task selected")
    };
    let block = Block::new().title("Task Details").borders(Borders::all());
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(task_text, inner);
}

