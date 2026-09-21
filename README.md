# shareDrop

A local network file and text sharing tool, inspired by Apple AirDrop and [Snapdrop](https://snapdrop.net). Built with a Rust core.

## How it works

Devices connect to a signaling server. Peers discover each other automatically by subnet and establish direct WebRTC connections for file and text transfer — no accounts, no cloud, no relay.

## Stack

- **Backend:** Rust, [axum](https://github.com/tokio-rs/axum), tokio
- **Frontend:** HTML, CSS, JavaScript
- **Transfer:** WebRTC DataChannel (peer-to-peer)

## Running locally

```bash
cargo run
```

Then open `http://localhost:3000` in your browser.

## Development

```bash
cargo fmt       # format code
cargo clippy    # lint
cargo test      # run tests
```

## Project docs

- [`docs/STATUS.md`](docs/STATUS.md) — current state, milestones, what's next
- [`docs/DECISIONS.md`](docs/DECISIONS.md) — design decisions and their rationale
- [`CLAUDE.md`](CLAUDE.md) — orientation for AI-assisted sessions
