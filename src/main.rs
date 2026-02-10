mod app;
mod cli;
mod event;
mod ping;
mod stats;
mod types;
mod ui;

use std::net::{IpAddr, ToSocketAddrs};

use clap::Parser;
use color_eyre::eyre::bail;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use dns_lookup::lookup_host;
use tokio_util::sync::CancellationToken;

use crate::app::App;
use crate::cli::Cli;
use crate::types::{HostInfo, HOST_COLORS};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    // Resolve hosts
    let hosts = resolve_hosts(&cli)?;
    if hosts.is_empty() {
        bail!("No hosts could be resolved");
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = ratatui::init();

    let cancel = CancellationToken::new();

    // Handle Ctrl+C at the OS level too
    let cancel_clone = cancel.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        cancel_clone.cancel();
    });

    let mut app = App::new(hosts, cli.graph_history);
    let result = app.run(&mut terminal, &cli, cancel).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    ratatui::restore();

    result
}

fn resolve_hosts(cli: &Cli) -> color_eyre::Result<Vec<HostInfo>> {
    let mut hosts = Vec::new();

    for (i, name) in cli.hosts.iter().enumerate() {
        let color = HOST_COLORS[i % HOST_COLORS.len()];

        // Try parsing as IP address first
        if let Ok(ip) = name.parse::<IpAddr>() {
            if should_use_ip(ip, cli) {
                hosts.push(HostInfo {
                    name: name.clone(),
                    ip,
                    color,
                });
                continue;
            }
        }

        // DNS resolution
        match lookup_host(name) {
            Ok(ips) => {
                let ip = ips
                    .into_iter()
                    .find(|ip| should_use_ip(*ip, cli))
                    .or_else(|| {
                        // Fallback: try ToSocketAddrs
                        format!("{name}:0")
                            .to_socket_addrs()
                            .ok()
                            .and_then(|mut addrs| {
                                addrs.find(|a| should_use_ip(a.ip(), cli)).map(|a| a.ip())
                            })
                    });

                match ip {
                    Some(ip) => {
                        hosts.push(HostInfo {
                            name: name.clone(),
                            ip,
                            color,
                        });
                    }
                    None => {
                        eprintln!("Warning: Could not resolve {name} with requested IP version");
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not resolve {name}: {e}");
            }
        }
    }

    Ok(hosts)
}

fn should_use_ip(ip: IpAddr, cli: &Cli) -> bool {
    if cli.ipv4 {
        ip.is_ipv4()
    } else if cli.ipv6 {
        ip.is_ipv6()
    } else {
        true // Accept either
    }
}
