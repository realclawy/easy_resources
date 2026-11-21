use std::io::{self, Write};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, poll, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Paragraph, Wrap},
    Terminal,
};

use crate::structs::WidgetDisplay;

// Convert color name to ratatui Color
fn color_to_ratatui_color(color_name: &str) -> Color {
    match color_name.to_lowercase().as_str() {
        "black" => Color::Rgb(0, 0, 0),
        "red" => Color::Rgb(204, 24, 30),
        "green" => Color::Rgb(16, 211, 26),
        "orange" => Color::Rgb(255, 165, 0),
        "blue" => Color::Rgb(29, 100, 224),
        "purple" => Color::Rgb(123, 22, 191),
        "aqua" => Color::Rgb(13, 224, 224),
        "white" => Color::Rgb(250, 250, 250),
        _ => Color::Rgb(250, 250, 250) // fallback
    }
}

// Calculate the display width of a string
fn display_width(s: &str) -> usize {
    unicode_width::UnicodeWidthStr::width(s)
}

// Calculate the wrapped height of text given a width
fn calculate_wrapped_height(text: &str, width: u16) -> u16 {
    let width = width as usize;
    if width == 0 {
        return 0;
    }

    let mut height = 0;
    for line in text.lines() {
        let line_w = display_width(line);
        if line_w == 0 {
            height += 1;
        } else {
            height += line_w.div_ceil(width);
        }
    }

    if height == 0 {
        1 // At least 1 line for empty text
    } else {
        height as u16
    }
}

pub fn clear() {
    print!("\x1B[2J\x1B[H"); // This clears the screen and puts the cursor at the start
    flush();
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

pub fn init() -> std::io::Result<Option<Terminal<CrosstermBackend<io::Stdout>>>> {
    // Set up terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(Some(terminal))
}

pub fn render(
    terminal: &mut Option<Terminal<CrosstermBackend<io::Stdout>>>,
    date: &str,
    date_color: &str,
    widget_1: &WidgetDisplay,
    widget_2: &WidgetDisplay,
    widget_3: &WidgetDisplay,
    widget_4: &WidgetDisplay,
) -> std::io::Result<()> {
    let terminal = terminal.as_mut().expect("Terminal not initialized");

    terminal.draw(|frame| {
        // Get the size of the terminal
        let size = frame.area();

        // Create vertical layout: first line for date, rest for widgets
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // First line for date
                Constraint::Min(0),    // Rest for widgets
            ])
            .split(size);

        // Date paragraph on the first line, left-aligned
        let date_text = Span::raw(date);

        // Determine date color from the config
        let date_color_enum = color_to_ratatui_color(date_color);
        let date_paragraph = Paragraph::new(date_text)
            .style(Style::default().fg(date_color_enum).add_modifier(Modifier::BOLD));
        frame.render_widget(date_paragraph, main_layout[0]);

        // Build the widgets text
        let widgets: Vec<&str> = vec![
            widget_1.current.as_str(),
            widget_2.current.as_str(),
            widget_3.current.as_str(),
            widget_4.current.as_str(),
        ]
        .into_iter()
        .filter(|w| !w.is_empty())
        .collect();

        // Calculate available space for centered widgets
        let widgets_layout_rect = main_layout[1];

        // Calculate the actual wrapped height and max width for each widget
        let mut widget_heights: Vec<u16> = Vec::new();
        let mut max_widget_width = 0u16;

        for widget in &widgets {
            // Calculate wrapped height for this widget using the available width
            let available_width = widgets_layout_rect.width.saturating_sub(2); // Subtract 2 for padding
            if available_width > 0 {
                let h = calculate_wrapped_height(widget, available_width);
                widget_heights.push(h);
            } else {
                widget_heights.push(1);
            }

            // Find the maximum display width among widgets to center them properly
            let max_line_width = widget.lines().map(display_width).max().unwrap_or(0) as u16;
            if max_line_width > max_widget_width {
                max_widget_width = max_line_width;
            }
        }

        // Calculate the total height needed for all widgets
        let widgets_block_height: u16 = widget_heights.iter().sum();
        let num_widgets = widgets.len() as u16;

        // Create a layout to center the widgets block both horizontally and vertically
        if num_widgets > 0 {
            // Create a centered layout for the widgets vertically
            let vertical_spacing_top = (widgets_layout_rect.height.saturating_sub(widgets_block_height)) / 2;
            let vertical_spacing_bottom = widgets_layout_rect.height.saturating_sub(widgets_block_height) - vertical_spacing_top;

            let center_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(vertical_spacing_top),
                    Constraint::Length(widgets_block_height),
                    Constraint::Length(vertical_spacing_bottom),
                ])
                .split(widgets_layout_rect);

            // Create a centered layout for the widgets horizontally
            // Ensure minimum width of 10 to avoid division issues
            let min_width = 10u16;
            let effective_width = std::cmp::max(max_widget_width, min_width);
            let horizontal_spacing_left = (widgets_layout_rect.width.saturating_sub(effective_width)) / 2;
            let horizontal_spacing_right = widgets_layout_rect.width.saturating_sub(effective_width) - horizontal_spacing_left;

            let middle_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(horizontal_spacing_left),
                    Constraint::Length(effective_width),
                    Constraint::Length(horizontal_spacing_right),
                ])
                .split(center_layout[1]);

            // Create a layout for individual widget paragraphs with dynamic heights
            let widget_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(widget_heights.iter().map(|&h| Constraint::Length(h)).collect::<Vec<Constraint>>())
                .split(middle_layout[1]);

            // Create paragraphs for each widget with their individual colors
            for (i, widget) in widgets.iter().copied().enumerate() {
                // Determine the color for this specific widget
                let widget_color = match i {
                    0 => color_to_ratatui_color(&widget_1.color),
                    1 => color_to_ratatui_color(&widget_2.color),
                    2 => color_to_ratatui_color(&widget_3.color),
                    3 => color_to_ratatui_color(&widget_4.color),
                    _ => Color::White,
                };

                let widget_paragraph = Paragraph::new(widget)
                    .style(Style::default().fg(widget_color))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                frame.render_widget(widget_paragraph, widget_layout[i]);
            }
        }
    })?;

    Ok(())
}

pub fn check_quit(timeout: std::time::Duration) -> bool {
    if let Ok(Event::Key(key)) = read() && poll(timeout).unwrap_or(false) &&
        (key.code == KeyCode::Esc
         || key.code == KeyCode::Char('q')
         || (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL))
            {
                return true;
            }
    false
}

pub fn cleanup() -> std::io::Result<()> {
    // Initialize the terminal... Once more... (sorry this was the easiest fix)
    let terminal = init()?;
    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    if let Some(mut terminal) = terminal {
        terminal.show_cursor()?;
    }

    Ok(())
}
