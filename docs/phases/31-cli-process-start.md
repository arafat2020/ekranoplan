# Phase 31 — CLI start process

One pull request. Rules: [README](README.md).

**Depends on:** [phase 30](30-process-start-message.md).

## Goal

Add one CLI subcommand that sends `DoProcessStart` and prints the returned pid or the error flag.

## Files this review may edit

```text
bins/cli/src/commands/ps.rs
bins/server/src/admin.rs
```

## Review

- [ ] The command takes a session id and one command string.
- [ ] `admin.rs` gains one request variant and no other feature.
- [ ] The shell command does not appear.

## Not in this phase

`DoShellExecute`.
