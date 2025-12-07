//! Terminal UI rendering using ratatui and tui-big-text.

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

/// Renders the complete teleprompter UI.
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let main_block = Block::default().style(Style::default().bg(app.background_color));
    frame.render_widget(main_block, area);

    let horizontal_pad = (area.width as u32 * app.horizontal_padding as u32 / 100) as u16;
    let content_area = Rect {
        x: area.x + horizontal_pad,
        y: area.y,
        width: area.width.saturating_sub(horizontal_pad * 2),
        height: area.height.saturating_sub(1),
    };

    render_teleprompter(frame, app, content_area);
    render_status_bar(frame, app, area);
}

fn get_pixel_size(scale: u8) -> PixelSize {
    match scale {
        1 => PixelSize::Quadrant,
        2 => PixelSize::HalfHeight,
        _ => PixelSize::Full,
    }
}

/// Returns character width in terminal columns for given scale (based on font8x8).
fn get_char_width(scale: u8) -> u16 {
    match scale {
        1 => 4,
        _ => 8,
    }
}

/// Returns line height in terminal rows for given scale (based on font8x8).
fn get_line_height(scale: u8) -> u16 {
    match scale {
        1 | 2 => 4,
        _ => 8,
    }
}

fn render_teleprompter(frame: &mut Frame, app: &mut App, area: Rect) {
    let line_height = get_line_height(app.font_scale);
    let char_width = get_char_width(app.font_scale);
    let visible_lines = (area.height / line_height) as usize;
    let max_chars = ((area.width / char_width) as usize)
        .saturating_sub(1)
        .max(1);

    if app.last_width != area.width {
        app.update_wrap(max_chars);
        app.last_width = area.width;
    }

    app.visible_height = visible_lines;

    let total_wrapped_lines = app.wrapped_lines.len();
    let pixel_size = get_pixel_size(app.font_scale);
    let style = Style::default().fg(app.text_color).bg(app.background_color);
    let scroll_line = app.scroll_offset as isize;

    for i in 0..visible_lines {
        let line_idx = scroll_line - (visible_lines as isize) + (i as isize);

        let text = if line_idx >= 0 && (line_idx as usize) < total_wrapped_lines {
            app.wrapped_lines[line_idx as usize].as_str()
        } else {
            ""
        };

        if text.is_empty() {
            continue;
        }

        let line_area = Rect {
            x: area.x,
            y: area.y + (i as u16 * line_height),
            width: area.width,
            height: line_height,
        };

        if line_area.y + line_height <= area.y + area.height {
            let big_text = BigText::builder()
                .pixel_size(pixel_size)
                .style(style)
                .lines(vec![text.into()])
                .centered()
                .build();

            frame.render_widget(big_text, line_area);
        }
    }
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };

    let status_style = Style::default()
        .fg(app.text_color)
        .bg(app.background_color)
        .add_modifier(Modifier::DIM);

    let pause_indicator = if app.paused { "[PAUSED] " } else { "" };
    let total_lines = app.wrapped_lines.len();
    let current_line = (app.scroll_offset as usize).min(total_lines);

    let status_text = format!(
        "{}Speed: {:.1} | {}/{} | [Space] Pause | [↑/↓] Scroll | [+/-] Speed | [r] Reset | [q] Quit",
        pause_indicator, app.speed, current_line, total_lines
    );

    let status = Paragraph::new(status_text)
        .style(status_style)
        .alignment(Alignment::Center);

    frame.render_widget(status, status_area);
}
