use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    if app.hosts.is_empty() {
        return;
    }

    let idx = app.selected_host;
    let host = &app.hosts[idx];
    let stat = &app.stats[idx];

    let title = format!(" Statistics: {} ", host.name);
    let block = Block::default()
        .title(title)
        .title_style(Style::default().fg(host.color).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let fmt = |label: &str, value: String| -> Line {
        Line::from(vec![
            Span::styled(
                format!("  {label:<14}"),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(value, Style::default().fg(Color::White)),
        ])
    };

    let fmt_ms = |v: Option<f64>| -> String {
        match v {
            Some(ms) => format!("{ms:.1} ms"),
            None => "---".to_string(),
        }
    };

    let min_max_avg = match (stat.min_ms(), stat.avg_ms(), stat.max_ms()) {
        (Some(min), Some(avg), Some(max)) => {
            format!("{min:.1} / {avg:.1} / {max:.1} ms")
        }
        _ => "---".to_string(),
    };

    let lines = vec![
        fmt("IP:", host.ip.to_string()),
        fmt("Sent/Recv:", format!("{} / {}", stat.sent, stat.received)),
        fmt(
            "Packet Loss:",
            format!("{:.2}%", stat.packet_loss_pct()),
        ),
        fmt("Min/Avg/Max:", min_max_avg),
        fmt("Std Dev:", fmt_ms(stat.stddev_ms())),
        fmt("Jitter:", fmt_ms(stat.jitter_ms())),
        fmt(
            "Status:",
            stat.status().symbol().to_string(),
        ),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
