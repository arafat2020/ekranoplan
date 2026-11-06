# Phase 02 — Identification messages

One pull request. Rules: [README](README.md).

**Depends on:** [phase 01](01-codec.md).

## Goal

Add the two handshake messages, and only those. Field numbers are written in the `.proto` file. They are not assigned by scanning types the way `TypeRegistry` does.

## Files this review may add

```text
crates/protocol/proto/identification.proto
crates/protocol/build.rs
crates/protocol/tests/identification.rs
```

`identification.proto` defines `ClientIdentification` and `ClientIdentificationResult`. `ClientIdentification` includes an `os` enum (`WINDOWS`, `MACOS`, `LINUX`) and a `capabilities` field. `build.rs` runs prost. `Cargo.toml` may add `prost` and `prost-build`.

## Behavior

The test builds a `ClientIdentification`, frames it with the phase 01 codec, and decodes the same values.

## Review

- [ ] `protocol` does not depend on `tokio` or `rustls`.
- [ ] The proto file contains two messages. A third message fails the review.
- [ ] The test fails if the codec is bypassed and raw structs are compared without a frame.

## Not in this phase

`GetProcesses` or any other feature message. Sockets.
