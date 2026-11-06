# Phase 52 — CLI connection list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 51](51-agent-connections.md).

## Goal

`ekranoplan-cli connections <session-id>` prints protocol, local, remote, and state.

## Files this review may add or edit

```text
bins/cli/src/commands/connections.rs
bins/server/src/admin.rs
```

## Review

- [ ] The command is read-only. No flag closes a row.
- [ ] Unknown session id fails locally.
- [ ] `admin.rs` adds one request variant.

## Not in this phase

`DoCloseConnection`. Reverse proxy.
