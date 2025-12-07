# Architecture

This document describes the high-level architecture of tp (tui_prompter).
If you want to familiarize yourself with the codebase, you are in the right place!

## Bird's Eye View

tp is a terminal-based teleprompter application. It displays text scrolling from bottom to top using large, readable characters rendered with the `tui-big-text` crate. The text automatically wraps to fit the terminal width.

The application follows a straightforward architecture:

```
Input Sources → App State → Event Loop → Terminal UI
```

Text can come from three sources (stdin pipe, file, or interactive editor). Once loaded, the app enters a main loop that handles user input events and renders the scrolling big text to the terminal.

## Code Map

### `src/main.rs`

The entry point. Orchestrates the application lifecycle:
1. Parses CLI arguments
2. Loads configuration
3. Retrieves text content from the appropriate source
4. Sets up the terminal (using `/dev/tty` to support piped input)
5. Runs the main event loop
6. Restores the terminal on exit

### `src/cli.rs`

Command-line interface definition using clap's derive macros. Defines the `Args` struct with all supported flags and arguments:
- `file`: Optional path to text file
- `speed`, `scale`, `color`, `background`, `padding`: Display customization options

### `src/config.rs`

Configuration management. Contains:
- `Config`: Root configuration struct with `display` and `scroll` sections
- `DisplayConfig`: Font scale, colors, padding
- `ScrollConfig`: Speed and starting position
- `parse_color()`: Converts color strings (names or hex) to ratatui `Color`

The config system is designed for easy extension—new settings can be added to the appropriate struct without changing other code.

### `src/input.rs`

Text input handling. The `get_text_content()` function determines the input source:
1. Checks if stdin has piped data (using `atty` crate)
2. Falls back to file path if provided
3. Opens the default editor as last resort (using `edit` crate)

### `src/app.rs`

Application state. The `App` struct holds:
- Original text lines and wrapped lines for display
- Current scroll position (as `f64` for smooth scrolling)
- Playback state (paused, speed)
- Display settings (colors, scale, padding)
- Terminal dimensions for rewrapping on resize

Key methods:
- `update()`: Advances scroll position based on elapsed time
- `update_wrap()`: Re-wraps text when terminal width changes
- `toggle_pause()`, `speed_up()`, `speed_down()`: User controls
- `scroll_up()`, `scroll_down()`, `reset()`: Manual navigation

Also contains `wrap_text()` which handles word-wrapping for the big text display.

### `src/ui.rs`

Rendering logic using `ratatui` and `tui-big-text`. The `render()` function:
1. Fills the background
2. Calculates the padded content area
3. Checks if terminal width changed and triggers rewrap
4. Renders visible lines using `BigText` widget with configurable `PixelSize`
5. Draws a status bar showing controls and progress

The teleprompter effect is achieved by:
- Prepending blank lines equal to visible height (text starts off-screen at bottom)
- Incrementing scroll offset over time (text moves up)
- Using `tui-big-text` for large, readable characters

### `src/event.rs`

Keyboard event handling. Polls for input events with a 16ms timeout (for ~60fps animation) and maps keys to app actions:
- `Space`/`p`: Toggle pause
- `+`/`-`: Adjust speed
- Arrow keys/`j`/`k`: Manual scroll
- `q`/`Esc`: Quit

## Cross-Cutting Concerns

### Terminal Handling

The application uses `/dev/tty` directly instead of stdout. This is necessary to support piped input—when text is piped to stdin, stdout may also be redirected, so we need a direct terminal connection for the TUI.

### Big Text Rendering

Text is rendered using `tui-big-text` with the `font8x8` bitmap font:
- Scale 1 (Quadrant): 4×4 terminal cells per character
- Scale 2 (HalfHeight): 8×4 terminal cells per character (default)
- Scale 3 (Full): 8×8 terminal cells per character

### Text Wrapping

Since `tui-big-text` doesn't wrap text automatically, the app pre-wraps all lines based on terminal width and font scale. Wrapped lines are stored in `App::wrapped_lines` and recalculated when the terminal is resized.

### Smooth Scrolling

Scroll position is stored as `f64` to enable smooth scrolling. The `update()` method uses `Instant` to calculate elapsed time and advances the position proportionally to the configured speed.

### Configuration Priority

Settings are applied in order of increasing priority:
1. Built-in defaults
2. Config file (`~/.config/tui_prompter/config.toml`)
3. Command-line arguments

## Invariants

- The main loop always restores the terminal state, even on error
- Font scale is clamped to 1-3 to match available `PixelSize` variants
- Padding is clamped to 0-40% to ensure content remains visible
- Scroll speed has a minimum of 0.5 and maximum of 20.0 lines/second
- Text is re-wrapped whenever terminal width changes
