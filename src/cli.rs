//! Command-line argument parsing.

use clap::Parser;
use std::path::PathBuf;

/// A terminal-based teleprompter application.
#[derive(Parser, Debug)]
#[command(name = "tp")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to a text file to display
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Scroll speed (lines per second)
    #[arg(short, long, value_name = "SPEED")]
    pub speed: Option<f64>,

    /// Font scale factor (1-3)
    #[arg(short = 'S', long, value_name = "SCALE")]
    pub scale: Option<u8>,

    /// Text color (e.g., white, green, #FF0000)
    #[arg(short, long, value_name = "COLOR")]
    pub color: Option<String>,

    /// Background color (e.g., black, blue, #000033)
    #[arg(short, long, value_name = "COLOR")]
    pub background: Option<String>,

    /// Horizontal padding as percentage of screen width (0-40)
    #[arg(short, long, value_name = "PERCENT")]
    pub padding: Option<u16>,
}
