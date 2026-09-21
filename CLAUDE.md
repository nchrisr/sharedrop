# shareDrop — context for Claude

Read this first, then `docs/STATUS.md` (where we are, what's next) and `docs/DECISIONS.md` (why things are the way they are).

## What this is

A local-network text/file sharing web app, in the spirit of AirDrop and Snapdrop. Devices open the app in a browser, discover each other via a Rust signaling server, and transfer data **directly between browsers over WebRTC DataChannels**. The server only brokers the connection; it never sees or stores the content.

Two deployment modes, both first-class:
1. **Self-hosted on a LAN** — a user runs the binary on their computer and opens `http://<that-machine's-ip>:3000` from their phone.
2. **Public instance** — a hosted URL anyone can visit, with the same privacy guarantees.

Source is (or will be) public on GitHub.

## How the owner works with Claude — read this

This is a **learning project** (Rust, WebRTC, network tooling). The owner writes the code.

- **Explain first, then let them implement.** Describe the concept, what to change, where, and why. Do not write the code into the repo unless explicitly asked.
- Short snippets are fine **when asked for**, or when an explanation genuinely can't be made without one. Keep them minimal.
- **Push back** on design choices when there's a real reason to. Give the alternative and the trade-off.
- Don't refactor or "clean up" beyond what was asked.
- **Never commit or push.** The owner does all git commits and pushes. Suggesting a commit message is fine.
- When a new Rust/networking/axum concept comes up, it's worth suggesting a short entry for `Concepts.md` (the owner's learning journal — see below).
- At the end of a working session, update `docs/STATUS.md` (session log + next steps). This is what makes the next session frictionless.

## Repository map

```
src/main.rs        server entry, router, ws_handler, subnet_key() + tests
src/peer.rs        Peer { id, subnet, sender: mpsc::Sender<SignalingMessage> }
src/messages.rs    SignalingMessage enum — the WebSocket wire format
static/            frontend, served via ServeDir fallback (placeholder for now)
docs/DECISIONS.md  design decisions with rationale
docs/STATUS.md     current state, next steps, session log
Concepts.md        owner's learning journal — gitignored, local only, do not commit
Cargo.toml         axum (ws), tokio (full), serde/serde_json, tower-http (fs), tracing, uuid
rustfmt.toml       edition 2024, width 100, crate-granular imports, std/external/crate grouping
```

## Commands

```bash
cargo run          # http://0.0.0.0:3000
cargo fmt
cargo clippy
cargo test         # unit tests live in-module under #[cfg(test)]
RUST_LOG=debug cargo run   # tracing level via EnvFilter, default "info"
```

## Conventions

- Rust edition 2024; format with `rustfmt.toml` as configured.
- Small commits, one concern each, imperative-mood messages (see `git log`).
- Pure functions get unit tests next to them (`subnet_key` is the pattern).
- Wire messages are JSON, tagged by `"type"` in `snake_case` (`#[serde(tag = "type", rename_all = "snake_case")]`).

## Hard constraints

- **Mostly Rust.** The backend is Rust; web tech only for the frontend.
- **Privacy: never store transferred content.** The server relays signaling only. Transfers are peer-to-peer.
- **Logging is welcome for diagnosing issues**, but: log connection events, peer IDs, subnet keys, message *types* — **never** SDP/ICE payloads, file names, or any user content.
