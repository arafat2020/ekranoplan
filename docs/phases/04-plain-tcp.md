# Phase 04 — Plain TCP frames

One pull request. Rules: [README](README.md).

**Depends on:** [phase 03](03-handler-bus.md).

## Goal

Move one framed message across a loopback TCP socket. TLS stays out so a framing bug is not confused with a certificate bug.

## Files this review may add

```text
crates/transport/src/tcp.rs
crates/transport/tests/tcp_loopback.rs
```

`Cargo.toml` may add `tokio` with the features this file uses.

## Behavior

The test binds `127.0.0.1` on port 0, dials it, writes one framed `ClientIdentification`, and reads the same struct on the other side. A second case writes a length above 5 MB and expects the codec error, then a stopped read.

The listener is dropped at the end of the test. It is not a running server.

## Review

- [ ] `transport/Cargo.toml` does not list `rustls`.
- [ ] `bus.rs` is unchanged.
- [ ] The test binds port 0, not a fixed port.

## Not in this phase

Certificate pinning, reconnect, the agent binary, the server binary.
