# tp

A terminal-based teleprompter with big, readable text.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![Demo](demo.gif)

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-orange" alt="Rust">
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macos-blue" alt="Platform">
</p>

## Features

- Large, readable text using bitmap font rendering
- Smooth scrolling from bottom to top
- Automatic text wrapping to fit terminal width
- Multiple input methods: pipe, file, or interactive editor
- Adjustable scroll speed with real-time controls
- Customizable colors and display settings
- Vim-style keyboard navigation
- Configuration file support

## Installation

Using [just](https://github.com/casey/just) (recommended):

```bash
just install    # Installs binary and man page
```

Or with cargo only (no man page):

```bash
cargo install --path .
```

### Other Commands

```bash
just              # List available commands
just build        # Build release binary
just uninstall    # Remove binary and man page
just demo         # Quick demo
```

After installation, view the man page with `man tp`.

## Quick Start

```bash
# Display a file
tp speech.txt

# Pipe text
echo "Hello, World!" | tp

# Open editor to write text
tp
```

## Usage

```
Usage: tp [OPTIONS] [FILE]

Arguments:
  [FILE]  Path to a text file to display

Options:
  -s, --speed <SPEED>       Scroll speed (lines per second)
  -S, --scale <SCALE>       Font scale factor (1-3)
  -c, --color <COLOR>       Text color (e.g., white, green, #FF0000)
  -b, --background <COLOR>  Background color (e.g., black, blue, #000033)
  -p, --padding <PERCENT>   Horizontal padding as percentage of screen width (0-40)
  -h, --help                Print help
  -V, --version             Print version
```

### Input Methods

**From a file:**
```bash
tp speech.txt
tp ~/Documents/presentation.txt
```

**Piping text:**
```bash
cat script.txt | tp
echo "Breaking news!" | tp
curl -s https://example.com/script.txt | tp
```

**Interactive editor:**
```bash
tp  # Opens $EDITOR or $VISUAL
```

### Examples

```bash
# Green text for that classic teleprompter look
tp -c green script.txt

# Slow scroll for careful reading
tp -s 1.0 speech.txt

# Smaller text to fit more content
tp -S 1 long_document.txt

# Largest text for maximum readability
tp -S 3 notes.txt

# Custom colors with hex values
tp -c '#00FF00' -b '#001100' notes.txt

# Extra padding for centered look
tp -p 20 presentation.txt
```

## Keyboard Controls

| Key | Action |
|-----|--------|
| `Space` / `p` | Pause/Resume |
| `+` / `=` | Speed up |
| `-` / `_` | Slow down |
| `Up` / `k` | Scroll up |
| `Down` / `j` | Scroll down |
| `PageUp` | Scroll up 10 lines |
| `PageDown` | Scroll down 10 lines |
| `Home` | Go to beginning |
| `End` | Go to end |
| `r` | Reset to start |
| `q` / `Esc` | Quit |

## Configuration

tp looks for a config file at `~/.config/tui_prompter/config.toml`:

```toml
[display]
font_scale = 2          # 1=small, 2=medium (default), 3=large
text_color = "white"
background_color = "black"
horizontal_padding = 10  # percentage

[scroll]
speed = 2.0             # lines per second
```

### Font Scales

| Scale | Size | Best For |
|-------|------|----------|
| 1 | 4×4 cells/char | Long documents, small terminals |
| 2 | 8×4 cells/char | General use (default) |
| 3 | 8×8 cells/char | Maximum readability, short text |

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for details on the codebase structure.

## Built With

- [ratatui](https://github.com/ratatui/ratatui) - Terminal UI framework
- [tui-big-text](https://github.com/joshka/tui-big-text) - Large text rendering
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [edit](https://github.com/twilligon/edit) - Editor integration

## License

MIT License - see [LICENSE](LICENSE) for details.
