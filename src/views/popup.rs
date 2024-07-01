use ratatui::{layout::{Constraint, Layout, Rect}, widgets::{Block, Clear}, Frame};


/// Renders a pop-up in the centre of the passed rectangle with the specified percentage
/// and title.
///
/// Returns the rectangle inside the pop-up into which the view should be rendered
///
pub fn render(frame: &mut Frame, title: &str, percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let outer_block = Block::bordered();
    let inner_block = Block::bordered().title(title);
    let area = centred_rect(percent_x, percent_y, area);
    let inner_area = outer_block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(&outer_block, area);
    frame.render_widget(&inner_block, inner_area);
    inner_block.inner(inner_area)
}

fn centred_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
