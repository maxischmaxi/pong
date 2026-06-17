use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::cli::Cli;
use crate::event::spawn_event_reader;
use crate::ping::{spawn_ping_task, PingConfig};
use crate::stats::HostStats;
use crate::types::{AppEvent, HostInfo};
use crate::ui;

pub struct App {
    pub hosts: Vec<HostInfo>,
    pub stats: Vec<HostStats>,
    pub selected_host: usize,
    pub start_time: Instant,
    pub zoom: f64,
    pub should_quit: bool,
    pub quit_after: Option<Instant>,
    pub hosts_done: Vec<bool>,
    pub total_sent: u64,
    pub total_received: u64,
}

impl App {
    pub fn new(hosts: Vec<HostInfo>, graph_history: usize) -> Self {
        let n = hosts.len();
        Self {
            hosts,
            stats: (0..n).map(|_| HostStats::new(graph_history)).collect(),
            selected_host: 0,
            start_time: Instant::now(),
            zoom: 1.0,
            should_quit: false,
            quit_after: None,
            hosts_done: vec![false; n],
            total_sent: 0,
            total_received: 0,
        }
    }

    pub async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        cli: &Cli,
        cancel: CancellationToken,
    ) -> color_eyre::Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel::<AppEvent>();

        // Spawn event reader
        spawn_event_reader(tx.clone(), cancel.clone());

        // Spawn ping tasks
        for (i, host) in self.hosts.iter().enumerate() {
            let config = PingConfig {
                host_index: i,
                ip: host.ip,
                interval: Duration::from_secs_f64(cli.interval),
                timeout: Duration::from_secs_f64(cli.timeout),
                payload_size: cli.size,
                ttl: cli.ttl,
                count: cli.count,
                interface: cli.interface.clone(),
            };
            spawn_ping_task(config, tx.clone(), cancel.clone());
        }

        // Spawn tick timer
        let tick_tx = tx.clone();
        let tick_rate = Duration::from_millis(cli.refresh_rate);
        let tick_cancel = cancel.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if tick_tx.send(AppEvent::Tick).is_err() {
                            break;
                        }
                    }
                    () = tick_cancel.cancelled() => break,
                }
            }
        });

        // Drop our copy of tx so channel closes when all senders are done
        drop(tx);

        // Initial draw
        terminal.draw(|frame| ui::draw(frame, self))?;

        // Main event loop
        while let Some(event) = rx.recv().await {
            match event {
                AppEvent::Ping(result) => {
                    let idx = result.host_index;
                    if idx < self.stats.len() {
                        self.total_sent += 1;
                        if matches!(
                            result.outcome,
                            crate::types::PingOutcome::Success { .. }
                        ) {
                            self.total_received += 1;
                        }
                        self.stats[idx].record(&result);
                    }
                }
                AppEvent::Key(key) => {
                    self.handle_key(key);
                }
                AppEvent::Tick => {
                    if let Some(quit_at) = self.quit_after {
                        if Instant::now() >= quit_at {
                            self.should_quit = true;
                        }
                    }
                    terminal.draw(|frame| ui::draw(frame, self))?;
                }
                AppEvent::Resize => {
                    terminal.draw(|frame| ui::draw(frame, self))?;
                }
                AppEvent::HostDone(idx) => {
                    if idx < self.hosts_done.len() {
                        self.hosts_done[idx] = true;
                    }
                    // If all hosts done, schedule quit after a brief delay
                    if self.hosts_done.iter().all(|d| *d) {
                        self.quit_after = Some(Instant::now() + Duration::from_millis(500));
                    }
                }
            }

            if self.should_quit {
                cancel.cancel();
                break;
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
                if !self.hosts.is_empty() {
                    self.selected_host = (self.selected_host + 1) % self.hosts.len();
                }
            }
            KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => {
                if !self.hosts.is_empty() {
                    self.selected_host = if self.selected_host == 0 {
                        self.hosts.len() - 1
                    } else {
                        self.selected_host - 1
                    };
                }
            }
            KeyCode::Char('r') => {
                for stat in &mut self.stats {
                    stat.reset();
                }
                self.total_sent = 0;
                self.total_received = 0;
                self.start_time = std::time::Instant::now();
            }
            KeyCode::Char('+' | '=') => {
                self.zoom = (self.zoom * 0.8).max(0.1);
            }
            KeyCode::Char('-') => {
                self.zoom = (self.zoom * 1.25).min(10.0);
            }
            _ => {}
        }
    }

    pub fn elapsed_str(&self) -> String {
        let secs = self.start_time.elapsed().as_secs();
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        format!("{h:02}:{m:02}:{s:02}")
    }
}
