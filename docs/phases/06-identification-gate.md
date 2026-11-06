# Phase 06 — Identification gate

One pull request. Rules: [README](README.md).

**Depends on:** [phase 05](05-tls.md).

## Goal

Do not deliver frames to the bus until identification succeeds. The listening side drops a connection whose first frame is not `ClientIdentification`. The dialing side stays unidentified when the result is false.

## Files this review may add

```text
crates/transport/src/session.rs
crates/transport/tests/identify.rs
```

## Behavior

Three loopback cases, using the phase 05 TLS helpers:

- Handshake succeeds, and a following frame reaches the bus.
- The first frame is not `ClientIdentification`. The socket closes. Use a length-prefixed buffer that does not decode as that message. Do not add a new feature message to make this case.
- `ClientIdentificationResult.result` is false. The dialing side does not mark itself identified.

## Review

- [ ] The gate is in `session.rs`. The same checks are not copied into `tcp.rs`.
- [ ] Hostname and OS in the test are constants. No `HostInfo` trait exists yet.
- [ ] The false-result test fails if the flag is set before the result arrives.

## Not in this phase

Real host facts, the agent binary, the server binary.
