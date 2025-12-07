//! Text input handling from various sources.

use anyhow::{Context, Result};
use std::io::{self, Read};
use std::path::Path;

/// Indicates where the text content came from.
#[allow(dead_code)]
pub enum TextSource {
    Stdin,
    File(String),
    Editor,
}

/// Gets text content from the appropriate source.
///
/// Priority: stdin (if piped) > file argument > interactive editor
pub fn get_text_content(file_path: Option<&Path>) -> Result<(String, TextSource)> {
    if !atty::is(atty::Stream::Stdin) {
        let content = read_from_stdin()?;
        return Ok((content, TextSource::Stdin));
    }

    if let Some(path) = file_path {
        let content = read_from_file(path)?;
        return Ok((content, TextSource::File(path.display().to_string())));
    }

    let content = open_editor()?;
    Ok((content, TextSource::Editor))
}

fn read_from_stdin() -> Result<String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .context("Failed to read from stdin")?;
    Ok(content)
}

fn read_from_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

fn open_editor() -> Result<String> {
    let template = "# Enter your teleprompter text below\n# Lines starting with # will be removed\n# Save and close the editor when done\n\n";

    let edited = edit::edit(template).context("Failed to open editor")?;

    let content: String = edited
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    if content.trim().is_empty() {
        anyhow::bail!("No content provided");
    }

    Ok(content)
}
