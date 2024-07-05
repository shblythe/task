#![warn(clippy::pedantic, clippy::all, clippy::unwrap_used)]
use std::io::{stdout, Result, Stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}, Terminal};
use task::{TaskEditView, TaskList, TaskListView};

fn setup_ratatui() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

fn shutdown_ratatui() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render(terminal: &mut Terminal<CrosstermBackend<Stdout>>,
          task_list_view: &mut TaskListView,
          task_edit_view: &TaskEditView,
          task_list: &TaskList,
          load_failed: bool,
          write_fails: i32) -> Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            ).split(area);
        frame.render_widget(Block::new().borders(Borders::TOP).title("Tasks"), main_layout[0]);
        task_list_view.render(frame, main_layout[1], task_list);
        task_edit_view.render(frame, main_layout[2]);
        frame.render_widget(Block::new().borders(Borders::TOP).title(
                if write_fails > 0 {
                    format!("** ERROR: Write failed {write_fails} times")
                }
                else if load_failed {
                    "** ERROR: Load failed - started with empty task list".to_string()
                } else {
                    "j/k = down/up, . = dot, q = quit".to_string()
                }
                ), main_layout[3]);
    })?;
    Ok(())
}

fn check_events(task_list_view: &mut TaskListView, task_edit_view: &mut TaskEditView, task_list: &mut TaskList) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && !task_edit_view.handle_key(key.code, task_list, task_list_view.selected_uuid())? {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('g') => task_list_view.move_start(task_list),
                    KeyCode::Char('G') => task_list_view.move_end(task_list),
                    KeyCode::Char('j') => task_list_view.move_down(task_list),
                    KeyCode::Char('k') => task_list_view.move_up(task_list),
                    KeyCode::Char('.') => task_list_view.toggle_dot(task_list)?,
                    KeyCode::Char('d') => task_list_view.complete(task_list)?,
                    KeyCode::Char('r') => task_list_view.recur_daily(task_list)?,
                    KeyCode::Char('z') => task_list_view.snooze_tomorrow(task_list)?,
                    _ => ()
                }
            }
        }
    }
    Ok(false)
}

fn main() -> Result<()> {
    let (mut tasks, task_load_failed) = TaskList::load().map_or_else(|_| (TaskList::default(), true), |tl| (tl, false));
    let mut task_list_view = TaskListView::default();
    let mut task_edit_view = TaskEditView::default();
    let mut write_fails = 0;

    let mut terminal = setup_ratatui()?;
    loop {
        render(&mut terminal, &mut task_list_view, &task_edit_view, &tasks, task_load_failed, write_fails)?;
        match check_events(&mut task_list_view, &mut task_edit_view, &mut tasks) {
            Ok(true) => break,
            Ok(false) => (),
            _ => write_fails += 1
        }
    }
    shutdown_ratatui()?;
    Ok(())
}
