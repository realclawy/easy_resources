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

// Strip ANSI escape codes from a string
fn strip_ansi_codes(text: &str) -> String {
    // ANSI escape codes match the pattern: \x1b[...m or \x1b[...K etc.
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip until we find 'm' or 'K' or other ANSI terminators
            let mut found_m = false;
            while let Some(&next_char) = chars.peek() {
                chars.next();
                if next_char == 'm' || next_char == 'K' || next_char == 'J' {
                    found_m = true;
                    break;
                }
                if next_char.is_alphanumeric() || next_char == ';' || next_char == '<' || next_char == '=' || next_char == '>' {
                    // Continue skipping
                } else {
                    break;
                }
            }
            if !found_m && !result.is_empty() {
                // If we didn't find a proper ANSI code terminator, restore the \x1b
                result.push('\x1b');
            }
        } else {
            result.push(c);
        }
    }

    // Remove any remaining \x1b[m style codes more thoroughly
    let mut clean_result = String::new();
    let mut chars = result.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if let Some(&next) = chars.peek() {
                if next == '[' {
                    let mut skip = true;
                    while let Some(&next_char) = chars.peek() {
                        chars.next();
                        if next_char == 'm' {
                            skip = false;
                            break;
                        }
                        if next_char == 'K' || next_char == 'J' || next_char == 'A' || next_char == 'B' || next_char == 'C' || next_char == 'D' {
                            skip = false;
                            break;
                        }
                        if !next_char.is_ascii() || !next_char.is_alphanumeric() && next_char != ';' && next_char != '<' && next_char != '=' && next_char != '>' {
                            break;
                        }
                    }
                    if skip {
                        continue;
                    }
                }
            }
        }
        clean_result.push(c);
    }

    clean_result
}

// Convert color name to ratatui Color
fn color_to_ratatui_color(color_name: &str) -> Color {
    match color_name.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "orange" => Color::Rgb(255, 165, 0), // Orange RGB
        "blue" => Color::Blue,
        "purple" => Color::Magenta,
        "aqua" => Color::Cyan,
        "white" => Color::White,
        _ => Color::White, // Default fallback
    }
}

pub fn clear() {
    print!("\x1B[2J\x1B[H"); // This clears the screen and puts the cursor at the start
    std::io::stdout().flush().unwrap();
}

pub fn flush() {
    // flush is now handled by ratatui's terminal.draw()
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
    date: &String,
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
        // Strip ANSI codes from date string
        let clean_date = strip_ansi_codes(date);
        let date_text = Span::raw(clean_date.as_str());

        // Determine date color from the config
        let date_color_enum = color_to_ratatui_color(date_color);
        let date_paragraph = Paragraph::new(date_text)
            .style(Style::default().fg(date_color_enum).add_modifier(Modifier::BOLD));
        frame.render_widget(date_paragraph, main_layout[0]);

        // Build the widgets text
        let widgets: Vec<String> = vec![
            widget_1.current.clone(),
            widget_2.current.clone(),
            widget_3.current.clone(),
            widget_4.current.clone(),
        ]
        .into_iter()
        .filter(|w| !w.is_empty())
        .collect();

        // Calculate available space for centered widgets
        let widgets_layout_rect = main_layout[1];

        // Find the maximum width among widgets to center them properly
        let max_widget_width = widgets.iter().map(|w| {
            // Handle potential multi-line widgets by taking the max line length
            w.lines().map(|line| line.len()).max().unwrap_or(0)
        }).max().unwrap_or(0) as u16;

        // Calculate the number of lines in the widgets block
        let num_widgets = widgets.len() as u16;

        // Create a layout to center the widgets block both horizontally and vertically
        if num_widgets > 0 {
            // Calculate the height needed for all widgets
            let widgets_block_height = num_widgets;

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
                .split(center_layout[2]);

            // Create a layout for individual widget paragraphs
            let widget_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints((0..num_widgets).map(|_| Constraint::Length(1)).collect::<Vec<Constraint>>())
                .split(middle_layout[1]);

            // Create paragraphs for each widget with their individual colors
            for (i, widget) in widgets.iter().enumerate() {
                let clean_widget = strip_ansi_codes(widget);

                // Determine the color for this specific widget
                let widget_color = match i {
                    0 => color_to_ratatui_color(&widget_1.color),
                    1 => color_to_ratatui_color(&widget_2.color),
                    2 => color_to_ratatui_color(&widget_3.color),
                    3 => color_to_ratatui_color(&widget_4.color),
                    _ => Color::White,
                };

                let widget_paragraph = Paragraph::new(clean_widget.as_str())
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
    if poll(timeout).unwrap() {
        if let Ok(Event::Key(key)) = read() {
            if key.code == KeyCode::Esc
                || key.code == KeyCode::Char('q')
                || (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL)
            {
                return true;
            }
        }
    }
    false
}

pub fn cleanup() -> std::io::Result<()> {
    // Initialize the terminal... Once more... (sorry this was the easiest fix)
    let mut terminal = init().unwrap_or_else(|_| panic!("AHHH"));
    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    if let Some(mut terminal) = terminal.take() {
        terminal.show_cursor()?;
    }

    Ok(())
}
