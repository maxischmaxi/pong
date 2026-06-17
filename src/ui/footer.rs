use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, _app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled(" Tab", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("/", Style::default().fg(Color::DarkGray)),
        Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("/", Style::default().fg(Color::DarkGray)),
        Span::styled("j/k", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(": select  ", Style::default().fg(Color::Gray)),
        Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("/", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(": quit  ", Style::default().fg(Color::Gray)),
        Span::styled("r", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(": reset  ", Style::default().fg(Color::Gray)),
        Span::styled("+/-", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(": zoom", Style::default().fg(Color::Gray)),
    ]);

    frame.render_widget(line, area);
}
