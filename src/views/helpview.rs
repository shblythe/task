use ratatui::{layout::Rect, text::Text, widgets::{Block, Borders}, Frame};

const HELP_TEXT : &str =
" Keyboard commands
 -----------------
 q - Quit
 
 g - Move to start
 k - Move up
 j - Move down
 G - Move to end
 
 a - add task
 . - Toggle dot
 d - Mark as done
 r - Toggle daily recurring
 z - Snooze until tomorrow
 Z - Snooze for 1s (test)
 
 h - Toggle help pane
 p - Toggle details pane
";

pub const WIDTH : u16 = 32;

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::new().title("Help").borders(Borders::all());
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(Text::from(HELP_TEXT), inner);
}

