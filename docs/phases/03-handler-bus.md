# Phase 03 — Handler bus

One pull request. Rules: [README](README.md).

**Depends on:** [phase 02](02-identification-messages.md).

## Goal

Implement the `MessageHandler` behavior in memory. A processor has `can_execute`, `can_execute_from`, and `execute`. `process` copies the matching processors under a mutex, releases the lock, then runs them.

## Files this review may add

```text
crates/transport/src/bus.rs
crates/transport/tests/bus.rs
```

`transport` may depend on `protocol`. It may not depend on `tokio` yet.

## Behavior

The test registers two processors. A message for session A does not run the processor bound to session B. After `unregister`, that processor does not run.

## Review

- [ ] `bus.rs` does not import `std::net` or `rustls`.
- [ ] The sample message is `ClientIdentification` from phase 02. No new message type is added.
- [ ] The session filter test fails if `can_execute_from` is ignored.

## Not in this phase

TCP, TLS, a binary that listens.
