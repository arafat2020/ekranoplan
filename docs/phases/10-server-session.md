# Phase 10 — Server records a session

One pull request. Rules: [README](README.md).

**Depends on:** [phase 06](06-identification-gate.md), [phase 08](08-linux-host-info.md), and [phase 09](09-macos-host-info.md). The host phases are required only so the workspace still builds. This phase does not call `HostInfo`.

## Goal

The server binary accepts one TLS connection, requires identification, and stores the peer in memory. It logs os, hostname, and username. It does not dial anything.

## Files this review may add or rewrite

```text
bins/server/src/main.rs
bins/server/src/sessions.rs
```

`sessions.rs` is a map from connection id to the identification fields. Bind address, port, and certificate paths are constants at the top of `main.rs`, with a comment that phase 13 replaces them.

## Behavior

A test, or a short manual step written in the pull request, connects with the phase 05 test certificate, sends `ClientIdentification`, and observes one log line. Closing the socket removes the map entry.

## Review

- [ ] The agent binary is unchanged.
- [ ] The CLI binary is unchanged.
- [ ] A first frame that is not identification does not create a map entry.

## Not in this phase

Config files, reconnect, a local admin socket, any feature handler.
