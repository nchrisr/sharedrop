# Design decisions

Each entry: the context, what was decided, why, and whether it's settled. Add a new entry when a choice is made that a future reader (or a future Claude session) might otherwise second-guess or accidentally undo.

Status legend: **settled** — deliberate, change only with a good reason. **first-pass** — it works, chosen without much deliberation, open to revisiting.

---

## D1. Signaling server + browser-to-browser WebRTC; server never touches content — settled

**Context.** Need to move text and files between devices without accounts or cloud storage. Privacy is a core requirement.

**Decision.** A Rust server does only *signaling*: it tells peers about each other and relays the SDP offer/answer and ICE candidates needed to set up a WebRTC connection. The actual transfer goes over a WebRTC DataChannel directly between browsers.

**Why.** The server never sees the data, so "we don't store your data" is true by construction, not by policy. This is also how Snapdrop works, so the model is proven for this use case.

**Consequences.** No transfer history, no resumable uploads via the server, no server-side scanning. If two peers can't reach each other directly (symmetric NAT), a TURN relay would be needed — deliberately out of scope for now.

---

## D2. Mostly Rust — settled

**Context.** The project exists partly to learn Rust and network programming.

**Decision.** Backend in Rust (axum + tokio). Frontend uses web technologies as needed; no constraint on framework choice there.

**Why.** Learning goal. Also a single static binary is easy for self-hosters to run.

---

## D3. Two deployment modes: self-hosted LAN and public instance — settled

**Context.** The owner wants (a) anyone to be able to run it on their own machine and reach it from their phone via the machine's LAN IP, and (b) a public URL that offers the same privacy.

**Decision.** Both are supported targets. The server binds `0.0.0.0` so LAN devices can reach it.

**Consequences.**
- Peer discovery must work when clients connect directly (LAN, `ConnectInfo` gives the real client IP) **and** when they connect through a reverse proxy (public hosting, `ConnectInfo` gives the proxy's IP for everyone). The second case needs `X-Forwarded-For` / `Forwarded` header handling, gated so it's only trusted when actually behind a proxy. **Not yet implemented — see STATUS.md.**
- TLS: browsers require a secure context for some WebRTC features. On a public host TLS comes from the proxy; on a LAN `http://192.168.x.x` counts as insecure — this needs investigating before the WebRTC work. **Open question.**

---

## D4. Peers grouped by `/24` subnet of the client IP — first-pass

**Context.** Devices should only see other devices "near" them, not every user of a public instance.

**Decision.** `subnet_key()` takes the first three octets of the client's IPv4 address (`192.168.1.5` → `192_168_1`). Peers are stored per subnet key and will only be announced to each other within the same key.

**Why (first-pass).** Simple, no user input needed, and matches Snapdrop's model. On a public instance, devices behind the same home router share one public IP and therefore one key, which is the desired grouping.

**Alternatives worth revisiting.**
- Group by *full* IP rather than /24. On a public instance the /24 of a public IP groups unrelated households at the same ISP together — a privacy/UX smell. On a LAN, /24 vs full IP behaves the same for typical home networks. Full-IP grouping may be strictly better for the public case.
- An optional user-entered room code for cross-network sharing.

---

## D5. IPv6 connections are rejected — first-pass

**Decision.** `subnet_key()` returns `None` for IPv6 and the WebSocket handler closes the connection with a warning.

**Why.** Kept the first version simple.

**Should revisit.** Many mobile networks and some ISPs are IPv6-first. A key derived from the IPv6 prefix (e.g. /64) would be the equivalent of the /24 grouping. Note that `[::1]` (IPv6 localhost) is also rejected, which can surprise you when testing locally.

---

## D6. `Arc<tokio::sync::Mutex<HashMap<subnet, HashMap<Uuid, Peer>>>>` for the peer registry — first-pass

**Decision.** One shared map, nested by subnet then by peer ID, guarded by tokio's async Mutex, shared via `Arc` through axum `State`.

**Why.** Straightforward and easy to reason about while learning. The nesting makes "all peers on my subnet" a single lookup.

**Alternatives.** `std::sync::Mutex` is cheaper and fine as long as the lock is never held across an `.await` (currently true). `DashMap` or `RwLock` if contention ever matters. It won't at this project's scale — don't change this without a reason.

---

## D7. Per-peer `mpsc` channel (capacity 32) as the outbound path — first-pass

**Decision.** Each connection gets an `mpsc::channel::<SignalingMessage>(32)`. The `Sender` lives in `Peer` so other tasks can push messages to that peer; a task owned by the connection will drain the `Receiver` into the WebSocket.

**Why.** Standard actor-ish pattern: only the connection's own task writes to its socket, everyone else goes through the channel. Avoids sharing the socket sink between tasks.

**Notes.** 32 is arbitrary. If a peer's channel is full, `send().await` will apply backpressure to whoever is relaying to it; `try_send` and drop is the alternative. Decide when implementing the relay.

---

## D8. Wire format: JSON, `#[serde(tag = "type", rename_all = "snake_case")]` — settled

**Decision.** `SignalingMessage` serializes as `{"type": "offer", "from": ..., "to": ..., "sdp": ...}` etc.

**Why.** Internally tagged enums are the most natural shape for a JS client to switch on. `snake_case` keeps the JSON conventional.

**Current variants.** `peer_joined`, `peer_left`, `offer`, `answer`, `ice_candidate`. Expect to add something like `welcome` (your own ID + current peer list) and maybe `ping`/`pong`.

---

## D9. Privacy and logging policy — settled

**Decision.** Never persist transferred content on the server. Logging is encouraged for diagnosing problems, with a hard line on what's logged:

- **OK:** connection/disconnection events, peer IDs, subnet keys, message *types*, counts, errors.
- **Never:** SDP or ICE candidate payloads, file names or sizes, text content, anything from a DataChannel.

**Why.** SDP and ICE contain local IPs and network topology; content is the user's. The public instance must be trustworthy without the user having to take anyone's word for it.

---

## D10. `Concepts.md` is a local learning journal, gitignored — settled

**Decision.** `Concepts.md` in the repo root holds explanations of concepts encountered while building. It is in `.gitignore` and not committed.

**Why.** It's personal study notes, not project documentation. Claude should suggest entries for it when new concepts come up, but never `git add` it.
