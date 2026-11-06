# Phase 50 — Connection list messages

One pull request. Rules: [README](README.md).

**Depends on:** [phase 47](47-net-trait.md).

## Goal

Add `GetConnections` and `GetConnectionsResponse`. `DoCloseConnection` is not a message.

## Files this review may add

```text
crates/protocol/proto/connections.proto
crates/protocol/tests/connections.rs
```

## Behavior

The test frames two rows and decodes the same local and remote fields.

## Review

- [ ] The proto file has two messages.
- [ ] A search of the diff finds no `DoCloseConnection`.
- [ ] Agent and CLI crates are unchanged.

## Not in this phase

The handler. That is [phase 51](51-agent-connections.md).
