use std::net::IpAddr;
use std::time::Duration;

use rand::random;
use surge_ping::{Client, Config, PingIdentifier, PingSequence, SurgeError, ICMP};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::types::{AppEvent, PingOutcome, PingResult};

pub struct PingConfig {
    pub host_index: usize,
    pub ip: IpAddr,
    pub interval: Duration,
    pub timeout: Duration,
    pub payload_size: usize,
    pub ttl: Option<u32>,
    pub count: Option<u64>,
    pub interface: Option<String>,
}

/// Spawn a ping task for a single host.
pub fn spawn_ping_task(
    config: PingConfig,
    tx: mpsc::UnboundedSender<AppEvent>,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        if let Err(e) = run_ping(&config, tx.clone(), cancel).await {
            let _ = tx.send(AppEvent::Ping(PingResult {
                host_index: config.host_index,
                seq: 0,
                outcome: PingOutcome::Error(e.to_string()),
            }));
        }
    });
}

async fn run_ping(
    config: &PingConfig,
    tx: mpsc::UnboundedSender<AppEvent>,
    cancel: CancellationToken,
) -> color_eyre::Result<()> {
    let mut builder = Config::builder();

    if let Some(iface) = &config.interface {
        builder = builder.interface(iface);
    }
    if let Some(ttl) = config.ttl {
        builder = builder.ttl(ttl);
    }
    if config.ip.is_ipv6() {
        builder = builder.kind(ICMP::V6);
    }

    let client_config = builder.build();
    let client = Client::new(&client_config)?;

    let mut pinger = client.pinger(config.ip, PingIdentifier(random())).await;
    pinger.timeout(config.timeout);

    let payload = vec![0xAA; config.payload_size];
    let mut seq: u16 = 0;
    let mut sent_count: u64 = 0;

    loop {
        if cancel.is_cancelled() {
            break;
        }

        if let Some(count) = config.count {
            if sent_count >= count {
                let _ = tx.send(AppEvent::HostDone(config.host_index));
                break;
            }
        }

        let outcome = match pinger.ping(PingSequence(seq), &payload).await {
            Ok((_, dur)) => PingOutcome::Success { rtt: dur },
            Err(SurgeError::Timeout { .. }) => PingOutcome::Timeout,
            Err(e) => PingOutcome::Error(e.to_string()),
        };

        let result = PingResult {
            host_index: config.host_index,
            seq,
            outcome,
        };

        if tx.send(AppEvent::Ping(result)).is_err() {
            break;
        }

        sent_count += 1;
        seq = seq.wrapping_add(1);

        tokio::select! {
            () = tokio::time::sleep(config.interval) => {}
            () = cancel.cancelled() => break,
        }
    }

    Ok(())
}
