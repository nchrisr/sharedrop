# Roadmap

Where the project is going, in order. [`STATUS.md`](STATUS.md) is where it is *right now* — current state, the next few commits, session log. Tick items here as they land; move the detail to `STATUS.md` when a milestone becomes the active work.

Items marked **must** are commitments, not ideas.

## Now — rename to CrabShare (must)

The project was called shareDrop. That name belongs to an existing WebRTC file-transfer project — [sharedrop.io](https://github.com/sharedropio/sharedrop), ~10.7k stars, same premise, acquired by LimeWire in 2025. Rename while the repo has no visibility and the cost is six files.

Casing rule: `CrabShare` everywhere a human reads it, `crabshare` everywhere a machine reads it (crate, binary, directory, repo slug, domain).

- [ ] `Cargo.toml` — package `name` to `crabshare`; this renames the binary too
- [ ] `cargo check` — regenerates the `Cargo.lock` entry; don't hand-edit it
- [ ] `static/index.html` — `<title>` and `<h1>`
- [ ] `README.md` — heading, tagline, and repoint the `snapdrop.net` link at the GitHub repo (the domain is now a LimeWire ad page)
- [ ] `CLAUDE.md` — heading
- [ ] `Concepts.md` — heading (local, gitignored, no rush)
- [ ] GitHub — rename `nchrisr/sharedrop` to `nchrisr/crabshare`, then `git remote set-url origin`
- [ ] Rename the local working directory, last, after the commit

## Now — claim the names (must)

Both were unregistered as of 2026-09-23. This is the only item on the list that someone else can take from us.

- [ ] Register `crabshare.com`
- [ ] Register `crabshare.app`
- [ ] Optional: reserve `crabshare` on crates.io with a stub publish

## Milestones

- [x] **M0 — Skeleton.** axum server, `/ws` route, static file serving, tracing.
- [x] **M1 — Peer registry.** Subnet grouping, per-subnet peer map, `Peer` struct, per-peer channel, `SignalingMessage` enum.
- [ ] **M2 — Signaling works.** Server parses incoming messages, routes offer/answer/ICE to the target peer, broadcasts join/leave within a subnet. Testable with two browser tabs / `websocat`.
- [ ] **M3 — Frontend discovers peers.** Minimal page: connects to `/ws`, shows a list of peers on your subnet updating live.
- [ ] **M4 — WebRTC DataChannel between two browsers.** Offer/answer/ICE exchange through the server; a DataChannel opens; send a "hello".
- [ ] **M5 — Text sharing (must).** Paste or copy text on one device, it appears on another device on the same network. This is the first milestone where the tool is actually useful to someone, and the shortest path to a demo — prioritise it over file sharing.
- [ ] **M6 — File sharing.** Chunked file send over the DataChannel with progress; receiver saves it.
- [ ] **M7 — Public-hosting readiness (must).** Trusted-proxy handling for client IP (`X-Forwarded-For`), TLS via proxy, IPv6 grouping, basic abuse limits. Blocks M8.
- [ ] **M8 — Polish & release (must).** UI, device names/icons, README for self-hosters, and the public instance live at `crabshare.com` and `crabshare.app` — same privacy guarantees as self-hosting, no content ever touching the server.

## Deferred

Tracked here so they don't get silently dropped. Rationale lives in [`DECISIONS.md`](DECISIONS.md).

- TURN relay for peers that can't connect directly (symmetric NAT). Out of scope — D1.
- Full-IP vs `/24` peer grouping on the public instance. Revisit at M7 — D4.
- IPv6 support. Currently rejected outright. Revisit at M7 — D5.
- Optional room codes for sharing across networks.
- Frontend framework choice. Vanilla is fine through M3.
