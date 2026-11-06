# Phase 51 — Agent connection list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 50](50-connection-messages.md).

## Goal

The agent handler calls `NetTable::connections` and sends `GetConnectionsResponse`. The test uses the fake's two rows.

## Files this review may add

```text
bins/agent/src/handlers/connections.rs
crates/transport/tests/connections.rs
```

`identify.rs` may set the connections capability bit. That is the only edit allowed in that file.

## Review

- [ ] The handler has no method that closes a socket.
- [ ] The test fails if a row is dropped.
- [ ] The CLI is unchanged.

## Not in this phase

The CLI command. Closing a remote connection.
