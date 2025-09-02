use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use std::io::stdout;

mod banner;
mod task_ui;
use banner::tui_banner;
mod add_task;
mod timer;
mod util;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; // enter TUI mode
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // Run app
    let result = tui_banner(terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    result
}
