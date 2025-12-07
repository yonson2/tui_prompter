//! Configuration loading and color parsing.

use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Root configuration structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub scroll: ScrollConfig,
}

/// Display-related settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Font scale: 1=small (4x4), 2=medium (8x4), 3=large (8x8)
    pub font_scale: u8,
    /// Text color name or hex (e.g., "white", "#FF0000")
    pub text_color: String,
    /// Background color name or hex
    pub background_color: String,
    /// Horizontal padding as percentage of screen width (0-40)
    pub horizontal_padding: u16,
}

/// Scroll behavior settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollConfig {
    /// Lines to scroll per second
    pub speed: f64,
    /// Starting position (currently unused)
    pub start_position: f64,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            font_scale: 2,
            text_color: "white".to_string(),
            background_color: "black".to_string(),
            horizontal_padding: 10,
        }
    }
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            speed: 2.0,
            start_position: 1.0,
        }
    }
}

impl Config {
    /// Loads configuration from the default path, or returns defaults if not found.
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Returns the default config file path (~/.config/tui_prompter/config.toml).
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("tui_prompter").join("config.toml"))
    }
}

/// Parses a color string into a ratatui Color.
///
/// Supports named colors (e.g., "red", "lightblue") and hex codes (e.g., "#FF0000").
pub fn parse_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        s if s.starts_with('#') && s.len() == 7 => {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(255);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(255);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(255);
            Color::Rgb(r, g, b)
        }
        _ => Color::White,
    }
}
