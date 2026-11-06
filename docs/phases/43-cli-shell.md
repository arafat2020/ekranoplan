# Phase 43 — CLI shell output

One pull request. Rules: [README](README.md).

**Depends on:** [phase 42](42-agent-shell.md).

## Goal

`ekranoplan-cli shell <session-id> <command>` prints stdout, stderr, and the exit code.

## Files this review may add or edit

```text
bins/cli/src/commands/shell.rs
bins/server/src/admin.rs
```

## Review

- [ ] One command, one response. The CLI does not keep a stream open.
- [ ] Unknown session id fails locally.
- [ ] `admin.rs` adds one request variant.

## Not in this phase

A PTY or a persistent remote shell.
