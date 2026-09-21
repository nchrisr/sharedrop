# Status — resume here

Keep this file short and current. Update the **Current state**, **Next up**, and **Session log** sections at the end of every working session.

## End goal

A web app where anyone can paste text or drop files and share them between devices on the same network. Runnable by anyone on their own machine (open from a phone via the machine's LAN IP), and also hosted at a public URL with the same privacy guarantees. Source on GitHub.

## Milestones

- [x] **M0 — Skeleton.** axum server, `/ws` route, static file serving, tracing.
- [x] **M1 — Peer registry.** Subnet grouping, per-subnet peer map, `Peer` struct, per-peer channel, `SignalingMessage` enum.
- [ ] **M2 — Signaling works.** Server parses incoming messages, routes offer/answer/ICE to the target peer, broadcasts join/leave within a subnet. Testable with two browser tabs / `websocat`.
- [ ] **M3 — Frontend discovers peers.** Minimal page: connects to `/ws`, shows a list of peers on your subnet updating live.
- [ ] **M4 — WebRTC DataChannel between two browsers.** Offer/answer/ICE exchange through the server; a DataChannel opens; send a "hello".
- [ ] **M5 — Text sharing.** Paste text on one device, it appears on the other.
- [ ] **M6 — File sharing.** Chunked file send over the DataChannel with progress; receiver saves it.
- [ ] **M7 — Public-hosting readiness.** Trusted-proxy handling for client IP (`X-Forwarded-For`), TLS via proxy, IPv6 grouping, basic abuse limits.
- [ ] **M8 — Polish & release.** UI, device names/icons, README for self-hosters, deploy the public instance.

## Current state (as of 2026-09-21)

**Server** — `src/main.rs`
- Accepts WebSocket upgrades on `/ws`, derives a subnet key from the client IPv4, registers the peer in `PeerMap`, and removes it on disconnect (dropping the subnet entry when empty).
- The receive loop (`while let Some(msg) = socket.recv().await`) only breaks on error — **it does not parse or act on incoming messages yet**.
- The `rx` half of the per-peer channel is created and then dropped unused — **nothing forwards channel messages to the socket yet**.
- Nothing broadcasts `PeerJoined` / `PeerLeft`.
- `subnet_key` has two unit tests. Nothing else is tested.

**Frontend** — `static/index.html` is a placeholder `<h1>`.

**Tooling** — `cargo build`, `cargo test` pass on the last commit (`3f1e6a0`).

## Next up (in order)

These are M2. Each is small enough for one session and one commit.

1. **Split the socket and spawn a writer task.** `socket.split()` → `(sink, stream)`. Spawn a task that loops on `rx.recv()`, serializes each `SignalingMessage` to JSON, and sends it on `sink`. Concept to learn: why only one task should own the sink; `tokio::spawn` + `select!` or `abort()` for teardown.
2. **Send a welcome message.** On connect, tell the new peer its own ID and the IDs of peers already on its subnet. Needs a new `SignalingMessage` variant (e.g. `Welcome { peer_id, peers: Vec<Uuid> }`).
3. **Broadcast join/leave.** After registering, send `PeerJoined` to every *other* peer on the subnet via their `sender`; on disconnect, send `PeerLeft`. Decide how to handle a full or closed channel (see DECISIONS D7).
4. **Parse and route.** In the receive loop, `serde_json::from_str::<SignalingMessage>` each text frame. For `Offer` / `Answer` / `IceCandidate`, look up `to` in the same subnet and forward. Overwrite `from` with the sender's real ID so peers can't spoof. Ignore/log unknown or malformed messages — never crash the connection task on bad input.
5. **Verify with two clients** (two browser tabs with a few lines of JS in the console, or `websocat`). Then commit and tick M2.

After M2, M3 is the first frontend work — a good moment to decide whether to stay with vanilla JS or pick a framework (no decision made yet; vanilla is fine for M3).

## Open questions / deferred

- **Client IP behind a proxy** (public instance). `ConnectInfo` will give the proxy's IP. Need `X-Forwarded-For` handling that is only trusted when a "behind proxy" flag/env var is set. Deferred to M7. See DECISIONS D3.
- **`/24` vs full-IP grouping** for the public instance. See DECISIONS D4. Revisit at M7.
- **IPv6.** Currently rejected. Revisit at M7. See DECISIONS D5.
- **Secure context on LAN.** WebRTC APIs generally require `https://` or `localhost`. Whether `http://192.168.x.x` works for `RTCPeerConnection` in current browsers needs checking before M4. If not: self-signed cert, or `mkcert`, or document a workaround.
- **STUN server.** For LAN-only, host candidates may suffice and no STUN is needed. For the public instance, peers behind the same NAT still usually need STUN to discover they can talk directly. Decide at M4; a public STUN (e.g. Google's) is fine to start.
- **Frontend framework** — undecided, not urgent.

## Session log

One line per session. Newest first.

- **2026-09-21** — Wrote `CLAUDE.md`, `docs/DECISIONS.md`, `docs/STATUS.md` to make the project resumable across Claude sessions. No code changes.
- **2026-04-19** — Added `Peer` struct, `SignalingMessage` enum, per-peer `mpsc` channel. (`3f1e6a0`)
- **2026-04-18** — Peer map is now nested per subnet; peers removed on disconnect. (`b8e810a`)
- **2026-04-09** — Added `subnet_key()` with tests; connections grouped by /24. (`f77b50b`)
- **2026-04-05** — Initial axum skeleton. (`4f146b2`)
