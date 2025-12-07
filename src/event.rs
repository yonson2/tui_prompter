//! Keyboard event handling.

use crate::app::App;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

/// Polls for keyboard events and dispatches them to the app.
///
/// Uses a 16ms timeout for ~60fps animation smoothness.
pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(16))?
        && let Event::Key(key) = event::read()?
    {
        handle_key_event(app, key);
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true
        }
        KeyCode::Char(' ') | KeyCode::Char('p') => app.toggle_pause(),
        KeyCode::Char('+') | KeyCode::Char('=') => app.speed_up(),
        KeyCode::Char('-') | KeyCode::Char('_') => app.speed_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Char('r') | KeyCode::Home => app.reset(),
        KeyCode::PageUp => {
            for _ in 0..10 {
                app.scroll_up();
            }
        }
        KeyCode::PageDown => {
            for _ in 0..10 {
                app.scroll_down();
            }
        }
        KeyCode::End => {
            app.scroll_offset = app.wrapped_lines.len() as f64;
        }
        _ => {}
    }
}
