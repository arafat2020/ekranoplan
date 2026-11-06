# Phase 11 — Agent identifies

One pull request. Rules: [README](README.md).

**Depends on:** [phase 10](10-server-session.md).

## Goal

The agent dials the server, fills `ClientIdentification` from `HostInfo`, and completes the gate. One build links one platform crate.

## Files this review may rewrite

```text
bins/agent/src/main.rs
bins/agent/Cargo.toml
```

Feature flags:

```text
--no-default-features --features platform-linux
--no-default-features --features platform-macos
```

Host, port, and the pinned certificate path are constants at the top of `main.rs`, with a comment that phase 14 replaces them.

## Behavior

Run the phase 10 server and this agent on loopback with the test certificate. The server prints one line and stays up. Stopping the agent removes the session.

## Review

- [ ] The server crate is unchanged.
- [ ] Both feature flags are not enabled in one build.
- [ ] No feature handler module exists under `bins/agent/src/handlers/`.

## Not in this phase

TOML, reconnect, the CLI.
