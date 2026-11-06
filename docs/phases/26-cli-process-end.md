# Phase 26 — CLI end process

One pull request. Rules: [README](README.md).

**Depends on:** [phase 25](25-process-end-message.md).

## Goal

Add one CLI subcommand that sends `DoProcessEnd` for a session id and a pid, and prints the result flag.

## Files this review may edit

```text
bins/cli/src/commands/ps.rs
bins/server/src/admin.rs
```

`admin.rs` may add one request variant. It may not grow a second feature.

## Review

- [ ] The command requires both a session id and a pid.
- [ ] Unknown session id fails locally.
- [ ] `DoProcessStart` is still not accepted.

## Not in this phase

Starting a process.
