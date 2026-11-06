# Phase 01 — Length-prefix codec

One pull request. Rules: [README](README.md).

**Depends on:** [phase 00](00-workspace.md).

## Goal

Frame an opaque byte payload the way `PayloadWriter` does: a 4-byte little-endian length, then that many bytes. Refuse a length above 5 MB. No protobuf.

## Files this review may add

```text
crates/protocol/src/codec.rs
crates/protocol/src/error.rs
crates/protocol/tests/codec.rs
```

`lib.rs` may export `codec` and `error`. It may not gain a socket type.

## Behavior

`crates/protocol/tests/codec.rs` covers four cases:

- A round trip of a short payload.
- A payload whose length is zero.
- A length that claims more bytes than the buffer holds.
- A length above 5 MB.

## Review

- [ ] Removing the length check makes the 5 MB test fail.
- [ ] `codec.rs` does not mention message types or `std::net`.
- [ ] The tests do not bind a port.

## Not in this phase

`prost`, TLS, reconnect, any `.proto` file.
