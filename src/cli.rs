use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "pong", about = "Terminal UI ping tool with multi-host support")]
pub struct Cli {
    /// Hostnames or IP addresses to ping
    #[arg(required = true)]
    pub hosts: Vec<String>,

    /// Stop after sending COUNT pings per host
    #[arg(short = 'c', long = "count")]
    pub count: Option<u64>,

    /// Seconds between pings (default: 1.0)
    #[arg(short = 'i', long = "interval", default_value = "1.0")]
    pub interval: f64,

    /// Timeout per ping in seconds (default: 2.0)
    #[arg(short = 'W', long = "timeout", default_value = "2.0")]
    pub timeout: f64,

    /// Payload size in bytes (default: 56)
    #[arg(short = 's', long = "size", default_value = "56")]
    pub size: usize,

    /// Time to Live
    #[arg(short = 't', long = "ttl")]
    pub ttl: Option<u32>,

    /// Force IPv4
    #[arg(short = '4', long = "ipv4", conflicts_with = "ipv6")]
    pub ipv4: bool,

    /// Force IPv6
    #[arg(short = '6', long = "ipv6", conflicts_with = "ipv4")]
    pub ipv6: bool,

    /// Network interface to use
    #[arg(short = 'I', long = "interface")]
    pub interface: Option<String>,

    /// Maximum data points in graph (default: 1000)
    #[arg(long = "graph-history", default_value = "1000")]
    pub graph_history: usize,

    /// UI refresh rate in milliseconds (default: 100)
    #[arg(long = "refresh-rate", default_value = "100")]
    pub refresh_rate: u64,
}
