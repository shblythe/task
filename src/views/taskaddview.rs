use crossterm::event::KeyCode;
use ratatui::{layout::Rect, style::Style, text::{Line, Span, Text}, Frame};

use crate::{Task, TaskList};

#[derive(Default)]
pub enum InputMode {
    #[default]  Normal,
                Editing
}

#[derive(Default)]
pub struct TaskAddView {
    input: String,
    index: usize,
    mode: InputMode
}

impl TaskAddView {

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            match self.mode {
                InputMode::Normal => Text::from("NORMAL MODE"),
                InputMode::Editing => {
                    let left_of_cursor = Span::raw(self.input.chars().take(self.index).collect::<String>());
                    let mut cursor_str = self.input.chars().skip(self.index).take(1).collect::<String>();
                    if cursor_str.is_empty() {
                        cursor_str.push(' ');
                    }
                    let cursor = Span::styled(
                        cursor_str,
                        Style::new()
                            .fg(ratatui::style::Color::Black)
                            .bg(ratatui::style::Color::White)
                        );
                    let right_of_cursor = Span::raw(self.input.chars().skip(self.index+1).collect::<String>());
                    let line = Line::from(vec![
                                          left_of_cursor,
                                          cursor,
                                          right_of_cursor,
                    ]);
                    Text::from(vec![line])
                }
            },
            area);
    }


    // Lots of these functions were pulled from
    // https://github.com/ratatui-org/ratatui/blob/main/examples/user_input.rs

    fn cursor_left(&mut self) {
        let cursor_moved_left = self.index.saturating_sub(1);
        self.index = self.clamp_cursor(cursor_moved_left);
    }

    fn cursor_right(&mut self) {
        let cursor_moved_right = self.index.saturating_add(1);
        self.index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.cursor_right();
    }

    /// Returns byte index based on char pos
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i,_)| i)
            .nth(self.index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.index = 0;
    }

    fn save_task(&mut self, task_list: &mut TaskList) {
        task_list.add(Task::new(&self.input));
        self.input.clear();
        self.reset_cursor();
        self.mode = InputMode::Normal;
    }

    fn delete_char(&mut self) {
        // Delete the character by taking the sections to the left and right of it
        // and concatenating them, to avoid multi-byte char turmoil.
        if self.index > 0 {
            let current_index = self.index;
            let from_left_to_current = current_index - 1;
            let left_section = self.input.chars().take(from_left_to_current);
            let right_section = self.input.chars().skip(current_index);
            self.input = left_section.chain(right_section).collect();
            self.cursor_left();
        }
    }

    /// Attempts to handle keyboard input
    /// Returns true if it was handled, false if caller should handle
    pub fn handle_key(&mut self, code: KeyCode, task_list: &mut TaskList) -> bool {
        match self.mode {
            InputMode::Normal => {
                match code {
                    KeyCode::Char('a') => {
                        self.mode = InputMode::Editing;
                        true
                    },
                    _ => false
                }
            },
            InputMode::Editing => {
                match code {
                    KeyCode::Enter => self.save_task(task_list),
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.cursor_left(),
                    KeyCode::Right => self.cursor_right(),
                    KeyCode::Esc => self.mode = InputMode::Normal,
                    _ => ()
                };
                true
            }
        }
    }
}

