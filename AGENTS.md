# AGENTS.md

> **Living document.** Kept short on purpose and grown as the project does. If something here is wrong, fix it here.

## Part 1: Orientation

A local-network text and file sharing web app, in the spirit of AirDrop and Snapdrop. A Rust server does WebRTC **signaling only**: it introduces peers and relays their connection setup. Content moves directly between browsers over WebRTC DataChannels.

The project is being renamed from shareDrop to CrabShare (see `docs/ROADMAP.md`). Write `CrabShare` where a human reads it and `crabshare` where a machine reads it (crate, binary, repo, domain).

### Where things live

```
src/main.rs        entry point, router, /ws handler, subnet_key() and its tests
src/peer.rs        Peer: id, subnet key, outbound channel sender
src/messages.rs    SignalingMessage, the WebSocket wire format
static/            frontend (plain HTML/CSS/JS), served as the router's fallback
docs/ROADMAP.md    milestones, in order
docs/STATUS.md     maintainer's local notes on current state; gitignored, may not exist
docs/DECISIONS.md  design decisions with rationale (D1, D2, …)
```

### Build and run

Rust edition 2024, stable toolchain.

```bash
cargo build
cargo run                  # listens on 0.0.0.0:3000; run from the repo root
RUST_LOG=debug cargo run   # default log level is info
```

- `static/` is resolved relative to the current directory, so `cargo run` from anywhere else serves 404s.
- Host and port are constants in `src/main.rs`. There is no config file or env var for them yet.
- From another device on the LAN, open `http://<host's LAN IP>:3000`.
- Only IPv4 clients are accepted (D5).

### Test

```bash
cargo test          # unit tests, in-module under #[cfg(test)]
cargo clippy
cargo fmt --check
```

There is no CI and there are no integration tests. Check `/ws` by hand with two clients, either two browser tabs or `websocat ws://127.0.0.1:3000/ws`.

### Hard constraints

- **The server never stores or inspects transferred content.** It relays signaling only (D1).
- **Logging.** Allowed: connection events, peer IDs, subnet keys, message *types*, counts, errors. Never logged: SDP or ICE payloads, file names or sizes, text content, anything from a DataChannel (D9).
- **The backend is Rust.** Web technologies are used in `static/` only (D2).
- **Both deployment modes are first-class:** self-hosted on a LAN, and a public instance behind a reverse proxy (D3). Behind a proxy, `ConnectInfo` holds the proxy's IP, not the client's. Trusted-proxy handling is not built yet (M7).
- Decisions marked **settled** in `docs/DECISIONS.md` change only together with an update to their entry. Record new design choices there.

## Part 2: Coding conventions

**Principle:** prefer readability and explicitness over cleverness or terseness, and accept some verbosity for it, as long as performance doesn't suffer. The rules below cover only places where this project departs from default Rust practice or pins down something that is ambiguous by default.

- **Wire format.** `SignalingMessage` is JSON, internally tagged with `#[serde(tag = "type", rename_all = "snake_case")]`. New variants follow the same scheme (D8).
- **Never trust client-supplied identity.** When relaying, overwrite `from` with the sender's real peer ID, and route to `to` only within the sender's subnet.
- **Client input must never panic the server.** Log malformed or unknown messages by type only, then skip them. The connection task keeps running.
- **Use `expect` instead of `unwrap` outside tests.** Write the message as the assumption that broke, for example `.expect("bind listener")`.
- **Peer map lock.** Take the lock inside an explicit `{ }` block and never `.await` while holding it. To message other peers, clone their `Sender`s inside the block and send after the block ends.
- **Name the target type of a conversion.** Write `Some(x)` or `Ok(x)` rather than `.into()` when wrapping in `Option` or `Result`. In general, avoid `.into()` when the target type isn't visible on the same line.
- **Imports: one `use` per module path, no nested braces.** For example, `use axum::extract::ws::{WebSocket, WebSocketUpgrade};` rather than one merged `use axum::{...}` tree. Put them in three groups separated by blank lines: `std`, external crates, then `crate`. Stable rustfmt can't enforce either rule, so keep both by hand.
- **Tests.** Pure functions get unit tests next to them, in the same file's `#[cfg(test)] mod tests`. `subnet_key` is the pattern.
- Before committing, run `cargo fmt` and make sure there are no new `cargo clippy` warnings.
