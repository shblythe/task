#![warn(clippy::pedantic, clippy::all, clippy::unwrap_used)]
use std::io::{stdout, Result, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use ratatui::{backend::CrosstermBackend, Terminal};
use task::MainView;

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

fn main() -> Result<()> {
    let mut terminal = setup_ratatui()?;
    let mut mainview = MainView::new();
    mainview.run(&mut terminal);
    shutdown_ratatui()?;
    Ok(())
}
