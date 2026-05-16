//! Shell Tool - UI Rendering
//!
//! This module handles the visual presentation of shell command results
//! in the terminal UI, including ANSI color support and virtual scrolling.

use ratatui::{
    text::{Line, Span, Text},
    style::{Style, Color},
    widgets::Paragraph,
};

/// Render shell command output with ANSI color support
pub fn render_shell_output(output: &str, max_lines: usize) -> Text<'static> {
    let lines: Vec<&str> = output.lines().collect();
    let display_lines = if lines.len() > max_lines {
        &lines[lines.len() - max_lines..]
    } else {
        &lines[..]
    };
    
    let mut text_lines = Vec::new();
    
    for line in display_lines {
        let spans = parse_ansi_line(line);
        text_lines.push(Line::from(spans));
    }
    
    // Add truncation notice if needed
    if lines.len() > max_lines {
        text_lines.push(Line::from(vec![
            Span::styled(
                format!("... ({} lines truncated, showing last {})", lines.len(), max_lines),
                Style::default().fg(Color::Yellow),
            )
        ]));
    }
    
    Text::from(text_lines)
}

/// Parse a line with ANSI escape codes into colored spans
fn parse_ansi_line(line: &str) -> Vec<Span<'static>> {
    // Simple ANSI parser - in production, use the `vte` or `ansi_term` crate
    let mut spans = Vec::new();
    let mut current_color = None;
    let mut current_text = String::new();
    
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '[' {
            // ANSI escape sequence
            if !current_text.is_empty() {
                spans.push(Span::styled(
                    current_text.clone(),
                    Style::default().fg(current_color.unwrap_or(Color::White)),
                ));
                current_text.clear();
            }
            
            // Parse the escape code
            let mut j = i + 2;
            while j < chars.len() && chars[j] != 'm' {
                j += 1;
            }
            
            if j < chars.len() {
                let params: String = chars[i+2..j].iter().collect();
                current_color = parse_color_code(&params);
                i = j + 1;
            } else {
                current_text.push(chars[i]);
                i += 1;
            }
        } else {
            current_text.push(chars[i]);
            i += 1;
        }
    }
    
    if !current_text.is_empty() {
        spans.push(Span::styled(
            current_text,
            Style::default().fg(current_color.unwrap_or(Color::White)),
        ));
    }
    
    if spans.is_empty() {
        spans.push(Span::styled(
            line.to_string(),
            Style::default().fg(Color::White),
        ));
    }
    
    spans
}

/// Parse ANSI color code to ratatui Color
fn parse_color_code(params: &str) -> Option<Color> {
    match params {
        "30" => Some(Color::Black),
        "31" => Some(Color::Red),
        "32" => Some(Color::Green),
        "33" => Some(Color::Yellow),
        "34" => Some(Color::Blue),
        "35" => Some(Color::Magenta),
        "36" => Some(Color::Cyan),
        "37" => Some(Color::White),
        "90" => Some(Color::DarkGray),
        "91" => Some(Color::LightRed),
        "92" => Some(Color::LightGreen),
        "93" => Some(Color::LightYellow),
        "94" => Some(Color::LightBlue),
        "95" => Some(Color::LightMagenta),
        "96" => Some(Color::LightCyan),
        "97" => Some(Color::LightGray),
        "0" | "" => None,
        _ => None,
    }
}

/// Create a Paragraph widget for shell output
pub fn create_shell_output_widget(output: &str, width: u16) -> Paragraph<'static> {
    let text = render_shell_output(output, 100);
    Paragraph::new(text)
        .wrap(ratatui::widgets::Wrap { trim: false })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_shell_output_short() {
        let output = "line1\nline2\nline3";
        let text = render_shell_output(output, 10);
        assert_eq!(text.lines.len(), 3);
    }

    #[test]
    fn test_render_shell_output_truncates() {
        let output = (1..=20).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        let text = render_shell_output(&output, 5);
        assert_eq!(text.lines.len(), 6); // 5 lines + truncation notice
    }

    #[test]
    fn test_parse_color_code_basic() {
        assert_eq!(parse_color_code("31"), Some(Color::Red));
        assert_eq!(parse_color_code("32"), Some(Color::Green));
        assert_eq!(parse_color_code("0"), None);
    }
}
