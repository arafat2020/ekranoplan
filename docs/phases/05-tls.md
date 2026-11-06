# Phase 05 — TLS on the loopback

One pull request. Rules: [README](README.md).

**Depends on:** [phase 04](04-plain-tcp.md).

## Goal

Wrap the phase 04 streams in rustls. The dialing side pins the server certificate. A different certificate closes the connection before a frame is delivered.

## Files this review may add

```text
crates/transport/src/tls.rs
crates/transport/tests/tls_loopback.rs
crates/transport/tests/fixtures/server.crt
crates/transport/tests/fixtures/server.key
crates/transport/tests/fixtures/wrong.crt
crates/transport/tests/fixtures/README.md
```

`Cargo.toml` may add `rustls`. The fixture key is a test key. `fixtures/README.md` says how it was generated and that it is not a production key.

## Behavior

Two cases. The pin matches and one identification frame arrives. The pin does not match and zero frames arrive.

## Review

- [ ] `tcp.rs` does not gain certificate types. TLS lives in `tls.rs` only.
- [ ] The mismatch test fails if the pin check is removed.
- [ ] The fixture PEM is marked as a test key.

## Not in this phase

The identification gate. A bad certificate must fail inside rustls, not because a message field was checked.
