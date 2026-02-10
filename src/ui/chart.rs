use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Latency (ms) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    if app.hosts.is_empty() {
        frame.render_widget(block, area);
        return;
    }

    // Calculate chart width for point windowing
    let chart_width = area.width.saturating_sub(10) as usize; // rough estimate for axis labels
    let max_points = chart_width.max(20);

    // Collect chart data for all hosts
    let chart_data: Vec<Vec<(f64, f64)>> = app
        .stats
        .iter()
        .map(|s| s.chart_data(max_points))
        .collect();

    // Auto-scale Y-axis
    let max_rtt = app
        .stats
        .iter()
        .map(|s| s.chart_max_rtt(max_points))
        .fold(0.0_f64, f64::max);

    let y_max = nice_ceil(max_rtt * 1.2 * app.zoom).max(10.0);

    // Find max x across all datasets
    let x_max = chart_data
        .iter()
        .flat_map(|d| d.iter().map(|(x, _)| *x))
        .fold(0.0_f64, f64::max)
        .max(10.0);

    let datasets: Vec<Dataset> = app
        .hosts
        .iter()
        .enumerate()
        .map(|(i, host)| {
            Dataset::default()
                .name(host.name.clone())
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(host.color))
                .data(&chart_data[i])
        })
        .collect();

    let x_axis = Axis::default()
        .style(Style::default().fg(Color::DarkGray))
        .bounds([0.0, x_max]);

    let y_labels = y_axis_labels(y_max);
    let y_axis = Axis::default()
        .title(Span::styled(
            "ms",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        ))
        .style(Style::default().fg(Color::DarkGray))
        .bounds([0.0, y_max])
        .labels(y_labels);

    let chart = Chart::new(datasets)
        .block(block)
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart, area);
}

/// Round up to a "nice" number for Y-axis.
fn nice_ceil(val: f64) -> f64 {
    if val <= 0.0 {
        return 10.0;
    }
    let steps = [1.0, 2.0, 2.5, 5.0, 10.0];
    let magnitude = 10.0_f64.powf((val.log10()).floor());
    for &step in &steps {
        let candidate = step * magnitude;
        if candidate >= val {
            return candidate;
        }
    }
    10.0 * magnitude
}

fn y_axis_labels(max: f64) -> Vec<Span<'static>> {
    let steps = 4;
    (0..=steps)
        .map(|i| {
            let val = max * i as f64 / steps as f64;
            Span::styled(
                format!("{val:.0}"),
                Style::default().fg(Color::DarkGray),
            )
        })
        .collect()
}
