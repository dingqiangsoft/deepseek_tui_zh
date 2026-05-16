//! Virtual Scrolling Optimizer for Long Outputs
//!
//! This module provides virtual scrolling optimization for rendering large
//! amounts of text in the terminal. It only renders visible lines, reducing
//! memory usage and improving performance for long outputs.
//!
//! Inspired by Claude Code's `useVirtualList` pattern, adapted for ratatui.

use ratatui::{
    text::{Line, Span, Text},
    style::{Style, Color},
};

/// Configuration for virtual list rendering
#[derive(Debug, Clone)]
pub struct VirtualListConfig {
    /// Number of lines to render above the visible area (overscan)
    pub overscan_top: usize,
    /// Number of lines to render below the visible area (overscan)
    pub overscan_bottom: usize,
    /// Maximum lines to render (safety limit)
    pub max_lines: usize,
}

impl Default for VirtualListConfig {
    fn default() -> Self {
        Self {
            overscan_top: 10,
            overscan_bottom: 10,
            max_lines: 1000,
        }
    }
}

/// Render a subset of lines based on visible area (virtual scrolling)
///
/// This function implements the virtual list pattern:
/// - Only renders lines in the visible area plus overscan
/// - Truncates output if it exceeds max_lines
/// - Adds truncation notices when needed
pub fn render_virtual_list(
    all_lines: &[Line<'static>],
    visible_top: usize,
    visible_count: usize,
    config: &VirtualListConfig,
) -> Text<'static> {
    let total_lines = all_lines.len();
    
    // Calculate render range (visible + overscan)
    let render_start = visible_top.saturating_sub(config.overscan_top);
    let render_end = (visible_top + visible_count + config.overscan_bottom).min(total_lines);
    
    // Apply safety limit
    let render_count = render_end - render_start;
    if render_count > config.max_lines {
        // Truncate to max_lines
        let actual_end = render_start + config.max_lines;
        return render_truncated_list(&all_lines[render_start..actual_end], total_lines, render_start);
    }
    
    // Normal render
    let lines_to_render = &all_lines[render_start..render_end];
    
    // Add top truncation notice if needed
    let mut result_lines = Vec::new();
    if render_start > 0 {
        result_lines.push(Line::from(vec![
            Span::styled(
                format!("... ({} lines above)", render_start),
                Style::default().fg(Color::Yellow),
            )
        ]));
    }
    
    result_lines.extend(lines_to_render.iter().cloned());
    
    // Add bottom truncation notice if needed
    if render_end < total_lines {
        result_lines.push(Line::from(vec![
            Span::styled(
                format!("... ({} lines below)", total_lines - render_end),
                Style::default().fg(Color::Yellow),
            )
        ]));
    }
    
    Text::from(result_lines)
}

/// Render a truncated list with notices
fn render_truncated_list(
    lines: &[Line<'static>],
    total_lines: usize,
    start_index: usize,
) -> Text<'static> {
    let mut result_lines = Vec::new();
    
    // Top truncation notice
    if start_index > 0 {
        result_lines.push(Line::from(vec![
            Span::styled(
                format!("... ({} lines above, truncated)", start_index),
                Style::default().fg(Color::Red).bold(),
            )
        ]));
    }
    
    result_lines.extend(lines.iter().cloned());
    
    // Bottom truncation notice
    let rendered = start_index + lines.len();
    if rendered < total_lines {
        result_lines.push(Line::from(vec![
            Span::styled(
                format!("... ({} more lines not shown)", total_lines - rendered),
                Style::default().fg(Color::Red).bold(),
            )
        ]));
    }
    
    Text::from(result_lines)
}

/// Smart render: automatically applies virtual scrolling for large outputs
pub fn smart_render_lines(
    all_lines: &[Line<'static>],
    visible_count: usize,
    scroll_offset: usize,
) -> Text<'static> {
    // Only apply virtual scrolling for large outputs (> 2x visible area)
    let threshold = visible_count * 2;
    
    if all_lines.len() <= threshold {
        // Small output - render everything
        return Text::from(all_lines.to_vec());
    }
    
    // Large output - use virtual scrolling
    let config = VirtualListConfig::default();
    render_virtual_list(all_lines, scroll_offset, visible_count, &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lines(count: usize) -> Vec<Line<'static>> {
        (0..count)
            .map(|i| Line::from(format!("Line {}", i)))
            .collect()
    }

    #[test]
    fn test_virtual_list_renders_visible_plus_overscan() {
        let lines = create_test_lines(100);
        let config = VirtualListConfig {
            overscan_top: 5,
            overscan_bottom: 5,
            max_lines: 1000,
        };
        
        let result = render_virtual_list(&lines, 50, 20, &config);
        
        // Should render lines 45-75 (50-5 to 50+20+5)
        assert_eq!(result.lines.len(), 31); // 30 lines + no truncation notices
    }

    #[test]
    fn test_virtual_list_adds_truncation_notices() {
        let lines = create_test_lines(200);
        let config = VirtualListConfig {
            overscan_top: 5,
            overscan_bottom: 5,
            max_lines: 50,
        };
        
        let result = render_virtual_list(&lines, 100, 20, &config);
        
        // Should have truncation notices
        assert!(result.lines.len() <= 52); // 50 lines + 2 notices
    }

    #[test]
    fn test_smart_render_small_output() {
        let lines = create_test_lines(10);
        let result = smart_render_lines(&lines, 20, 0);
        
        // Should render all lines
        assert_eq!(result.lines.len(), 10);
    }

    #[test]
    fn test_smart_render_large_output() {
        let lines = create_test_lines(100);
        let result = smart_render_lines(&lines, 20, 0);
        
        // Should use virtual scrolling (less than total)
        assert!(result.lines.len() < 100);
    }
}
