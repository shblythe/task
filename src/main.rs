#![warn(clippy::pedantic, clippy::all, clippy::unwrap_used)]
use std::io::{stdout, Result, Stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}, Terminal};
use task::{Task, TaskList, TaskListView, TaskAddView};

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
          task_add_view: &TaskAddView,
          task_list: &TaskList) -> Result<()> {
    terminal.draw(|frame| {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            ).split(frame.size());
        frame.render_widget(Block::new().borders(Borders::TOP).title("Tasks"), main_layout[0]);
        task_list_view.render(frame, main_layout[1], task_list);
        task_add_view.render(frame, main_layout[2]);
        frame.render_widget(Block::new().borders(Borders::TOP).title("j/k = down/up, . = dot, q = quit"), main_layout[3]);
    })?;
    Ok(())
}

fn check_events(task_list_view: &mut TaskListView, task_add_view: &mut TaskAddView, task_list: &mut TaskList) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && !task_add_view.handle_key(key.code, task_list) {
                match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Char('j') => task_list_view.move_down(task_list),
                    KeyCode::Char('k') => task_list_view.move_up(),
                    KeyCode::Char('.') => task_list_view.toggle_dot(task_list),
                    KeyCode::Char('d') => task_list_view.complete(task_list),
                    _ => ()
                }
            }
        }
    }
    Ok(true)
}

fn main() -> Result<()> {
    let mut tasks = TaskList::default();
    let mut task_list_view = TaskListView::default();
    let mut task_add_view = TaskAddView::default();
    tasks.add(Task::new("Do something"));
    tasks.add(Task::new("Do something else"));
    tasks.add(Task::new("What about this?"));
    tasks.get_mut(0).expect("Task impossibly didn't exist").toggle_dot();

    let mut terminal = setup_ratatui()?;
    loop {
        render(&mut terminal, &mut task_list_view, &task_add_view, &tasks)?;
        if !check_events(&mut task_list_view, &mut task_add_view, &mut tasks)? {
            break;
        }
    }
    shutdown_ratatui()?;
    Ok(())
}
