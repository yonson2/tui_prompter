//! Application state and logic for the teleprompter.

use crate::config::{Config, parse_color};
use ratatui::style::Color;
use std::time::Instant;

/// Wraps text to fit within a given character width, preserving words when possible.
fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    if max_chars == 0 {
        return vec![];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if word.len() > max_chars {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }
            let mut remaining = word;
            while remaining.len() > max_chars {
                lines.push(remaining[..max_chars].to_string());
                remaining = &remaining[max_chars..];
            }
            if !remaining.is_empty() {
                current_line = remaining.to_string();
            }
        } else if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= max_chars {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() && !text.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Main application state for the teleprompter.
///
/// Holds the text content, scroll position, display settings, and runtime state.
pub struct App {
    /// Original lines from input text
    pub lines: Vec<String>,
    /// Lines wrapped to fit current terminal width
    pub wrapped_lines: Vec<String>,
    /// Current scroll position (fractional for smooth scrolling)
    pub scroll_offset: f64,
    /// Whether scrolling is paused
    pub paused: bool,
    /// Scroll speed in lines per second
    pub speed: f64,
    /// Font scale (1=small, 2=medium, 3=large)
    pub font_scale: u8,
    pub text_color: Color,
    pub background_color: Color,
    /// Horizontal padding as percentage of screen width
    pub horizontal_padding: u16,
    pub last_update: Instant,
    pub should_quit: bool,
    /// Number of visible lines (updated by UI on each render)
    pub visible_height: usize,
    /// Last terminal width, used to detect when rewrap is needed
    pub last_width: u16,
}

impl App {
    /// Creates a new App with the given text content and configuration.
    pub fn new(content: String, config: &Config) -> Self {
        let lines: Vec<String> = content.lines().map(String::from).collect();

        Self {
            lines,
            wrapped_lines: Vec::new(),
            scroll_offset: 0.0,
            paused: false,
            speed: config.scroll.speed,
            font_scale: config.display.font_scale,
            text_color: parse_color(&config.display.text_color),
            background_color: parse_color(&config.display.background_color),
            horizontal_padding: config.display.horizontal_padding,
            last_update: Instant::now(),
            should_quit: false,
            visible_height: 24,
            last_width: 0,
        }
    }

    fn max_scroll(&self) -> f64 {
        let total_content = if self.wrapped_lines.is_empty() {
            self.lines.len() as f64
        } else {
            self.wrapped_lines.len() as f64
        };
        total_content + self.visible_height as f64
    }

    /// Re-wraps all lines to fit within the given character width.
    pub fn update_wrap(&mut self, max_chars: usize) {
        self.wrapped_lines = self
            .lines
            .iter()
            .flat_map(|line| {
                if line.trim().is_empty() {
                    vec![String::new()]
                } else {
                    wrap_text(line, max_chars)
                }
            })
            .collect();
    }

    /// Advances the scroll position based on elapsed time since last update.
    /// Quits automatically when the last line scrolls out of view.
    pub fn update(&mut self) {
        if self.paused {
            self.last_update = Instant::now();
            return;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;

        self.scroll_offset += self.speed * elapsed;

        let total_lines = if self.wrapped_lines.is_empty() {
            self.lines.len()
        } else {
            self.wrapped_lines.len()
        };

        // Quit when the last line has scrolled out of view
        if self.scroll_offset >= total_lines as f64 + self.visible_height as f64 {
            self.should_quit = true;
        }
    }

    /// Toggles between paused and playing states.
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        if !self.paused {
            self.last_update = Instant::now();
        }
    }

    /// Increases scroll speed by 0.5 lines/second (max 20).
    pub fn speed_up(&mut self) {
        self.speed = (self.speed + 0.5).min(20.0);
    }

    /// Decreases scroll speed by 0.5 lines/second (min 0.5).
    pub fn speed_down(&mut self) {
        self.speed = (self.speed - 0.5).max(0.5);
    }

    /// Scrolls up by one line.
    pub fn scroll_up(&mut self) {
        self.scroll_offset = (self.scroll_offset - 1.0).max(0.0);
    }

    /// Scrolls down by one line.
    pub fn scroll_down(&mut self) {
        let max = self.max_scroll();
        self.scroll_offset = (self.scroll_offset + 1.0).min(max);
    }

    /// Resets scroll position to the beginning.
    pub fn reset(&mut self) {
        self.scroll_offset = 0.0;
        self.last_update = Instant::now();
    }
}
