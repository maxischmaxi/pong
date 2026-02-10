use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Hosts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    for (i, host) in app.hosts.iter().enumerate() {
        let stat = &app.stats[i];
        let status = stat.status();
        let selected = i == app.selected_host;

        let indicator = if selected { "▸" } else { " " };
        let bullet_color = host.color;

        let rtt_str = match stat.last_rtt {
            Some(rtt) => format!("{:.1}ms", rtt.as_secs_f64() * 1000.0),
            None => "---".to_string(),
        };

        let line = Line::from(vec![
            Span::styled(
                format!(" {indicator} "),
                Style::default().fg(if selected {
                    Color::White
                } else {
                    Color::DarkGray
                }),
            ),
            Span::styled("● ", Style::default().fg(bullet_color)),
            Span::styled(
                host.name.clone(),
                Style::default()
                    .fg(if selected { Color::White } else { Color::Gray })
                    .add_modifier(if selected {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ),
        ]);
        lines.push(line);

        let detail = Line::from(vec![
            Span::raw("     "),
            Span::styled(
                format!("{rtt_str:>8}"),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled(
                status.symbol(),
                Style::default().fg(status.color()),
            ),
        ]);
        lines.push(detail);

        // Add spacing between hosts
        if i < app.hosts.len() - 1 {
            lines.push(Line::raw(""));
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
