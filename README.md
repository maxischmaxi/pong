# pong

A terminal UI ping tool with multi-host support, built with Rust and [Ratatui](https://ratatui.rs).

<!-- screenshot placeholder -->

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/maxischmaxi/ping/main/install.sh | sudo bash
```

`sudo` is required to install to `/usr/local/bin` and set raw socket capabilities.

### Build from Source

```bash
git clone https://github.com/maxischmaxi/ping.git
cd ping
cargo build --release
sudo cp target/release/pong /usr/local/bin/
sudo setcap cap_net_raw+ep /usr/local/bin/pong   # Linux only
```

## Usage

```bash
pong google.com
pong google.com cloudflare.com 8.8.8.8
pong -c 100 google.com                # stop after 100 pings
pong -i 0.5 google.com                # ping every 0.5s
pong -s 120 google.com                # 120 byte payload
pong -4 google.com                    # force IPv4
pong -6 google.com                    # force IPv6
pong -W 5 google.com                  # 5s timeout
pong -t 64 google.com                 # set TTL
pong -I eth0 google.com               # bind to interface
```

## Keybindings

| Key | Action |
|---|---|
| `q` / `Esc` | Quit |
| `Tab` / `Shift+Tab` | Cycle through hosts |
| `↑` / `↓` | Select host |

## Permissions

Pong uses raw sockets which require elevated privileges. On Linux the install script sets `cap_net_raw` via `setcap` so you can run `pong` without `sudo`. On macOS raw sockets are available without extra configuration.
