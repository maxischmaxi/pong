mod chart;
mod footer;
mod header;
mod host_list;
mod stats_panel;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Top-level: Header (1) | Chart (60%) | Bottom (40%) | Footer (1)
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),        // Header
            Constraint::Percentage(55),   // Chart
            Constraint::Percentage(45),   // Bottom
            Constraint::Length(1),        // Footer
        ])
        .split(area);

    header::draw(frame, app, main_layout[0]);
    chart::draw(frame, app, main_layout[1]);

    // Bottom split: Host list | Stats panel
    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(65),
        ])
        .split(main_layout[2]);

    host_list::draw(frame, app, bottom_layout[0]);
    stats_panel::draw(frame, app, bottom_layout[1]);

    footer::draw(frame, app, main_layout[3]);
}
