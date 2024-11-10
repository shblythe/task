use std::io::{Result, Stdout};

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Borders}, Frame, Terminal};

use crate::{taskdetailview, TaskEditView, TaskList, TaskListView};

pub struct MainView {
    tasks: TaskList,
    load_failed: bool,
    task_list_view: TaskListView,
    task_edit_view: TaskEditView,
    write_fails: i32,
    details_pane: bool,
    help_pane: bool,
}

impl Default for MainView {
    fn default() -> Self {
        Self::new()
    }
}

impl MainView {
    #[must_use]
    pub fn new() -> Self {
        let (tasks, load_failed) = TaskList::load().map_or_else(|_| (TaskList::default(), true), |tl| (tl, false));
        MainView {
            tasks,
            load_failed,
            task_list_view: TaskListView::default(),
            task_edit_view: TaskEditView::default(),
            write_fails: i32::default(),
            details_pane: bool::default(),
            help_pane: bool::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        loop {
            match self.tasks.pre_render() {
                Ok(()) => (),
                _ => self.write_fails += 1
            }
            self.render(terminal);
            match self.check_events() {
                Ok(true) => break,
                Ok(false) => (),
                _ => self.write_fails += 1
            }
        }
    }

    pub fn render_help(&self, frame: &mut Frame, area: Rect) {
        let block = Block::new().title("Help").borders(Borders::all());
        let inner = block.inner(area);
        if !self.task_edit_view.render_help(frame, inner) {
            self.task_list_view.render_help(frame, inner);
        }
        frame.render_widget(block, area);
    }

    fn render_panes(&mut self, frame: &mut Frame, area: Rect) {
        let panes = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Min(0),
                Constraint::Percentage(if self.details_pane { 30 } else { 0 }),
                Constraint::Length(if self.help_pane { 35 } else { 0 }),
            ]
            ).split(area);
        self.task_list_view.render(frame, panes[0], &self.tasks);
        taskdetailview::render(frame, panes[1], self.task_list_view.selected_uuid(), &self.tasks);
        self.render_help(frame, panes[2]);
    }

    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        let _ = terminal.draw(|frame| {
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
            self.render_panes(frame, main_layout[1]);
            self.task_edit_view.render(frame, main_layout[2]);
            frame.render_widget(Block::new().borders(Borders::TOP).title(
                    if self.write_fails > 0 {
                        format!("** ERROR: Write failed {0} times", self.write_fails)
                    }
                    else if self.load_failed {
                        "** ERROR: Load failed - started with empty task list".to_string()
                    } else {
                        "j/k = down/up, . = dot, q = quit".to_string()
                    }
                    ), main_layout[3]);
        });
    }

    /// Returns Ok(false) normally, Ok(true) if we're to quit.
    ///
    /// # Errors
    /// Returns an error if an activity results in a write fail
    fn check_events(&mut self) -> Result<bool> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                        && !self.task_edit_view.handle_key(key, &mut self.tasks, self.task_list_view.selected_uuid())?
                        && !self.task_list_view.handle_key(key, &mut self.tasks)?
                {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Char('p') => self.details_pane = !self.details_pane,
                        KeyCode::Char('h') => self.help_pane = !self.help_pane,
                        KeyCode::Char('f') => {
                            self.tasks.toggle_future_filter();
                            self.task_list_view.fix_selection(&self.tasks);
                        },
                        _ => ()
                    }
                }
            }
        }
        Ok(false)
    }

}
