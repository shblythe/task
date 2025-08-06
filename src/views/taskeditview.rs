use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, style::Style, text::{Line, Span, Text}, Frame};
use uuid::Uuid;

use crate::{Task, TaskList, TaskListView};

#[derive(Default)]
pub enum InputMode {
    #[default]  Normal,
                Editing
}

#[derive(Default)]
pub struct TaskEditView {
    input: String,
    index: usize,
    mode: InputMode,
    task_uuid: Option<Uuid> // is None if we're creating a new task
}

impl TaskEditView {

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            match self.mode {
                InputMode::Normal => Text::from("NORMAL MODE"),
                InputMode::Editing => {
                    let left_of_cursor = Span::raw(self.input.chars().take(self.index).collect::<String>());
                    let cursor_str = self.input.chars().nth(self.index).unwrap_or(' ').to_string();
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

    fn save_task(&mut self, task_list: &mut TaskList, task_list_view: &mut TaskListView) -> std::io::Result<()> {
        let result = if let Some(task_uuid) = self.task_uuid {
            let task = task_list.get(task_uuid).expect("Couldn't retrieve uuid'd task in TaskEditView::save_task\n{task_uuid}");
            let mut task = task.clone();
            task.update_description(&self.input);
            task_list.replace(task_uuid, task)
        } else {
            let at_end = task_list_view.is_at_end(task_list);
            let add_result = task_list.add(Task::new(&self.input));
            if at_end {
                task_list_view.move_end(task_list);
            }
            add_result
        };
        self.input.clear();
        self.reset_cursor();
        self.mode = InputMode::Normal;
        result
    }

    fn delete_char_at(&mut self, index: usize) {
        // Delete the character by taking the sections to the left and right of it
        // and concatenating them, to avoid multi-byte char turmoil.
        if index < self.input.len() {
            let left_section = self.input.chars().take(index);
            let right_section = self.input.chars().skip(index + 1);
            self.input = left_section.chain(right_section).collect();
        }
    }

    fn backspace_delete(&mut self) {
        if self.index > 0 {
            self.delete_char_at(self.index-1);
            self.cursor_left();
        }
    }

    fn delete_at_cursor(&mut self) {
        self.delete_char_at(self.index);
    }

    /// Attempts to handle keyboard input
    /// Returns true if it was handled, false if caller should handle
    ///
    /// # Errors
    ///
    /// Returns `Err` if we attempted to add a task, but the write to storage fails
    pub fn handle_key(
            &mut self,
            key: KeyEvent,
            task_list: &mut TaskList,
            task_list_view: &mut TaskListView)
                -> std::io::Result<bool> {
        match self.mode {
            InputMode::Normal => {
                if key.modifiers.is_empty() {
                    match key.code {
                        KeyCode::Char('a') => {
                            self.mode = InputMode::Editing;
                            self.task_uuid = None;
                            Ok(true)
                        },
                        KeyCode::Char('m') => {
                            if let Some(task_uuid) = task_list_view.selected_uuid() {
                                self.mode = InputMode::Editing;
                                self.task_uuid = Some(task_uuid);
                                if let Some(task) = task_list.get(task_uuid) {
                                    self.input = task.description().to_string();
                                    self.index = self.input.len();
                                }
                                Ok(true)
                            } else {
                                Ok(false)
                            }
                        }
                        _ => Ok(false)
                    }
                } else {
                    Ok(false)
                }
            },
            InputMode::Editing => {
                match key.code {
                    KeyCode::Enter => self.save_task(task_list, task_list_view)?,
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.backspace_delete(),
                    KeyCode::Left => self.cursor_left(),
                    KeyCode::Right => self.cursor_right(),
                    KeyCode::Esc => self.mode = InputMode::Normal,
                    KeyCode::Home => self.index = 0,
                    KeyCode::End => self.index = self.input.len(),
                    KeyCode::Delete => self.delete_at_cursor(),
                    _ => ()
                };
                Ok(true)
            }
        }
    }

    pub fn render_help(&self, frame: &mut Frame, area: Rect) -> bool {
        match self.mode {
            InputMode::Editing => {
                frame.render_widget(Text::from(
            " Edit mode help
 --------------
 ENT  - Save task
 Esc  - cancel edit/create
 
 Home - Move to start
 End  - Move to end
 Bksp - Delete before cursor
 Del  - Delete at cursor

 Use cursor keys to move cursor
 left and right
"
                        ), area);
                true
            }
            InputMode::Normal => false,
        }
    }

}

