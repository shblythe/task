use ratatui::{layout::Rect, text::Text, widgets::{Block, Borders}, Frame};
use uuid::Uuid;

use crate::TaskList;

pub fn render(frame: &mut Frame, area: Rect, task_uuid: Option<Uuid>, task_list: &TaskList) {
    let task = if let Some(task_uuid) = task_uuid {
        task_list.get(task_uuid)
    } else {
        None
    };
    let task_text = if let Some(task) = task {
        Text::from(task.detail_string())
    } else {
        Text::from("Invalid task selected")
    };
    let block = Block::new().title("Task Details").borders(Borders::all());
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(task_text, inner);
}

