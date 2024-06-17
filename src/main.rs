mod app;
mod control;
mod speedscope;
mod tui;
mod ui;

use clap::Parser;

use crossterm::event::{Event, KeyCode};
use ratatui::Frame;

use crate::{app::App, control::is_ctrlc, tui::Term, ui::ui};
use std::{error::Error, io, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("file {:?}", cli.file);
    match cli.verbose {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
    let mut term = tui::Term::new()?;
    let data = speedscope::Data::from_path(&cli.file)?;
    let mut app = App::new(data);
    _ = run_app(&mut term, &mut app);
    Ok(())
}

fn run_app(term: &mut Term, app: &mut App) -> io::Result<()> {
    loop {
        term.draw(|f| ui(f, app))?;

        let event = crossterm::event::read()?;
        if is_ctrlc(&event) {
            // panic!("Handle gracefully I guess...");
            break;
        }
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Enter => {
                    app.view.toggle_expand();
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    app.view.next_stack();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.view.prev_stack();
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    app.view.prev_frame();
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    app.view.next_frame();
                }
                _ => {}
            }
        }
    }
    Ok(())
}


