use std::{
    io,
    panic::{set_hook, take_hook},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, CompletedFrame, Frame, Terminal};

pub struct Term {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Term {
    pub fn new() -> io::Result<Self> {
        let original_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            // intentionally ignore errors here since we're already in a panic
            let _ = restore();
            original_hook(panic_info);
        }));

        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        let t = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Term { terminal: t })
    }

    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)
    }
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = restore();
    }
}
