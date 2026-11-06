# Phase 15 — Reconnect

One pull request. Rules: [README](README.md).

**Depends on:** [phase 14](14-agent-config.md).

## Goal

When the TLS session ends, wait, dial the same host again, and identify again. The server treats the new connection as a new session. The identified flag is cleared before the next dial.

## Files this review may add

```text
bins/agent/src/reconnect.rs
crates/transport/tests/reconnect.rs
```

`main.rs` may call `reconnect.rs`. It may not contain the delay arithmetic.

## Behavior

`reconnect.rs` exposes the wait duration: at least the configured delay, and at most that delay plus 750 ms. That unit test does not sleep. A loopback test accepts once, drops, and accepts again. An agent-level test that needs a clock is `#[ignore]`.

## Review

- [ ] The jitter math is not copied into `main.rs`.
- [ ] The identified flag from phase 06 is false at the start of the second dial.
- [ ] Config still has one host.

## Not in this phase

A list of hosts. That is [phase 53](53-multi-host.md).
