use std::collections::VecDeque;
use std::time::Duration;

use crate::types::{HostStatus, PingOutcome, PingResult};

/// Per-host statistics with incremental computation.
pub struct HostStats {
    pub sent: u64,
    pub received: u64,
    pub last_rtt: Option<Duration>,
    pub prev_rtt: Option<Duration>,

    // Welford's online algorithm state
    min_rtt: Option<f64>,
    max_rtt: Option<f64>,
    welford_mean: f64,
    welford_m2: f64,
    welford_count: u64,

    // Ring buffer for chart data: (seq, Option<rtt_ms>)
    pub history: VecDeque<(u16, Option<f64>)>,
    history_capacity: usize,

    // Consecutive failures for status
    consecutive_failures: u32,

    // Last error message (if any)
    pub last_error: Option<String>,

    // Elevated threshold in ms
    elevated_threshold_ms: f64,
}

impl HostStats {
    pub fn new(history_capacity: usize) -> Self {
        Self {
            sent: 0,
            received: 0,
            last_rtt: None,
            prev_rtt: None,
            min_rtt: None,
            max_rtt: None,
            welford_mean: 0.0,
            welford_m2: 0.0,
            welford_count: 0,
            history: VecDeque::with_capacity(history_capacity),
            history_capacity,
            consecutive_failures: 0,
            last_error: None,
            elevated_threshold_ms: 100.0,
        }
    }

    pub fn record(&mut self, result: &PingResult) {
        self.sent += 1;

        match &result.outcome {
            PingOutcome::Success { rtt } => {
                self.received += 1;
                self.consecutive_failures = 0;
                self.prev_rtt = self.last_rtt;
                self.last_rtt = Some(*rtt);

                let rtt_ms = rtt.as_secs_f64() * 1000.0;

                // Welford's update
                self.welford_count += 1;
                let delta = rtt_ms - self.welford_mean;
                self.welford_mean += delta / self.welford_count as f64;
                let delta2 = rtt_ms - self.welford_mean;
                self.welford_m2 += delta * delta2;

                // Min/max
                self.min_rtt = Some(match self.min_rtt {
                    Some(prev) => prev.min(rtt_ms),
                    None => rtt_ms,
                });
                self.max_rtt = Some(match self.max_rtt {
                    Some(prev) => prev.max(rtt_ms),
                    None => rtt_ms,
                });

                self.push_history(result.seq, Some(rtt_ms));
            }
            PingOutcome::Timeout => {
                self.consecutive_failures += 1;
                self.prev_rtt = self.last_rtt;
                self.last_rtt = None;
                self.last_error = None;
                self.push_history(result.seq, None);
            }
            PingOutcome::Error(msg) => {
                self.consecutive_failures += 1;
                self.prev_rtt = self.last_rtt;
                self.last_rtt = None;
                self.last_error = Some(msg.clone());
                self.push_history(result.seq, None);
            }
        }
    }

    fn push_history(&mut self, seq: u16, rtt_ms: Option<f64>) {
        if self.history.len() >= self.history_capacity {
            self.history.pop_front();
        }
        self.history.push_back((seq, rtt_ms));
    }

    pub fn status(&self) -> HostStatus {
        if self.sent == 0 {
            return HostStatus::Unknown;
        }
        if self.consecutive_failures >= 5 {
            return HostStatus::Down;
        }
        if let Some(rtt) = self.last_rtt {
            if rtt.as_secs_f64() * 1000.0 > self.elevated_threshold_ms {
                return HostStatus::Elevated;
            }
            HostStatus::Up
        } else if self.received > 0 {
            // Had successes before but last one failed
            HostStatus::Elevated
        } else {
            HostStatus::Down
        }
    }

    pub fn packet_loss_pct(&self) -> f64 {
        if self.sent == 0 {
            return 0.0;
        }
        (self.sent - self.received) as f64 / self.sent as f64 * 100.0
    }

    pub fn min_ms(&self) -> Option<f64> {
        self.min_rtt
    }

    pub fn max_ms(&self) -> Option<f64> {
        self.max_rtt
    }

    pub fn avg_ms(&self) -> Option<f64> {
        if self.welford_count == 0 {
            None
        } else {
            Some(self.welford_mean)
        }
    }

    pub fn stddev_ms(&self) -> Option<f64> {
        if self.welford_count < 2 {
            None
        } else {
            Some((self.welford_m2 / (self.welford_count - 1) as f64).sqrt())
        }
    }

    pub fn jitter_ms(&self) -> Option<f64> {
        match (self.last_rtt, self.prev_rtt) {
            (Some(a), Some(b)) => {
                let diff = (a.as_secs_f64() - b.as_secs_f64()).abs() * 1000.0;
                Some(diff)
            }
            _ => None,
        }
    }

    /// Data points for chart rendering: Vec of (index, `rtt_ms`).
    /// Only includes successful pings.
    pub fn chart_data(&self, max_points: usize) -> Vec<(f64, f64)> {
        let skip = self.history.len().saturating_sub(max_points);
        self.history
            .iter()
            .skip(skip)
            .enumerate()
            .filter_map(|(i, (_, rtt))| rtt.map(|r| (i as f64, r)))
            .collect()
    }

    /// Max RTT in visible chart data for auto-scaling.
    pub fn chart_max_rtt(&self, max_points: usize) -> f64 {
        let skip = self.history.len().saturating_sub(max_points);
        self.history
            .iter()
            .skip(skip)
            .filter_map(|(_, rtt)| *rtt)
            .fold(0.0_f64, f64::max)
    }

    pub fn reset(&mut self) {
        self.sent = 0;
        self.received = 0;
        self.last_rtt = None;
        self.prev_rtt = None;
        self.min_rtt = None;
        self.max_rtt = None;
        self.welford_mean = 0.0;
        self.welford_m2 = 0.0;
        self.welford_count = 0;
        self.history.clear();
        self.consecutive_failures = 0;
        self.last_error = None;
    }
}
