//! # tp - Terminal Teleprompter
//!
//! A terminal-based teleprompter application that displays scrolling text
//! using large, readable characters.

mod app;
mod cli;
mod config;
mod event;
mod input;
mod ui;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::fs::File;

use app::App;
use clap::Parser;
use cli::Args;
use config::Config;
use input::get_text_content;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut config = Config::load().unwrap_or_default();

    if let Some(speed) = args.speed {
        config.scroll.speed = speed;
    }
    if let Some(scale) = args.scale {
        config.display.font_scale = scale.clamp(1, 3);
    }
    if let Some(color) = args.color {
        config.display.text_color = color;
    }
    if let Some(background) = args.background {
        config.display.background_color = background;
    }
    if let Some(padding) = args.padding {
        config.display.horizontal_padding = padding.clamp(0, 40);
    }

    let (content, _source) = get_text_content(args.file.as_deref())?;

    if content.trim().is_empty() {
        anyhow::bail!("No content to display");
    }

    let mut app = App::new(content, &config);

    // Use /dev/tty directly so TUI works even when stdin is piped
    let mut tty = File::options().read(true).write(true).open("/dev/tty")?;

    enable_raw_mode()?;
    execute!(tty, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<File>>, app: &mut App) -> Result<()> {
    loop {
        app.update();
        terminal.draw(|frame| ui::render(frame, app))?;
        event::handle_events(app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
