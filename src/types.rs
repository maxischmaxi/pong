use std::net::IpAddr;
use std::time::Duration;

use crossterm::event::KeyEvent;
use ratatui::style::Color;

/// Result of a single ping attempt.
#[derive(Debug, Clone)]
pub struct PingResult {
    pub host_index: usize,
    pub seq: u16,
    pub outcome: PingOutcome,
}

#[derive(Debug, Clone)]
pub enum PingOutcome {
    Success { rtt: Duration },
    Timeout,
    Error(String),
}

/// Resolved host information.
#[derive(Debug, Clone)]
pub struct HostInfo {
    pub name: String,
    pub ip: IpAddr,
    pub color: Color,
}

/// Status of a monitored host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostStatus {
    Up,
    Elevated,
    Down,
    Unknown,
}

impl HostStatus {
    pub fn symbol(self) -> &'static str {
        match self {
            HostStatus::Up => "▲ Up",
            HostStatus::Elevated => "▲ Elevated",
            HostStatus::Down => "▼ Down",
            HostStatus::Unknown => "? Unknown",
        }
    }

    pub fn color(self) -> Color {
        match self {
            HostStatus::Up => Color::Green,
            HostStatus::Elevated => Color::Yellow,
            HostStatus::Down => Color::Red,
            HostStatus::Unknown => Color::DarkGray,
        }
    }
}

/// All events flowing through the main channel.
#[derive(Debug)]
pub enum AppEvent {
    Ping(PingResult),
    Key(KeyEvent),
    Resize,
    Tick,
    /// A host finished all pings (count reached).
    HostDone(usize),
}

/// Color palette for hosts.
pub const HOST_COLORS: [Color; 8] = [
    Color::Cyan,
    Color::Magenta,
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Red,
    Color::LightCyan,
    Color::LightMagenta,
];
