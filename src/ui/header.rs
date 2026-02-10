use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let host_count = app.hosts.len();
    let elapsed = app.elapsed_str();

    let line = Line::from(vec![
        Span::styled(
            " pong ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ▏ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{host_count} host{}", if host_count != 1 { "s" } else { "" }),
            Style::default().fg(Color::White),
        ),
        Span::styled(" ▏ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("Elapsed: {elapsed}"),
            Style::default().fg(Color::White),
        ),
        Span::styled(" ▏ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} sent, {} recv", app.total_sent, app.total_received),
            Style::default().fg(Color::White),
        ),
    ]);

    frame.render_widget(line, area);
}
