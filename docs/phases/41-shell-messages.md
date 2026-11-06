# Phase 41 — Shell messages

One pull request. Rules: [README](README.md).

**Depends on:** [phase 38](38-shell-trait.md).

## Goal

Add `DoShellExecute` and `DoShellExecuteResponse`. The response carries exit code, stdout, stderr, and a truncated flag. The size cap is a comment on the response message, and it is under the 5 MB frame cap. Nothing executes a command in this phase.

## Files this review may add

```text
crates/protocol/proto/shell.proto
crates/protocol/tests/shell.rs
```

## Behavior

The test frames a response whose stdout is `hi` and whose truncated flag is false, then decodes it.

## Review

- [ ] The proto file has two messages.
- [ ] Agent and CLI crates are unchanged.
- [ ] No PTY message is added.

## Not in this phase

The handler. That is [phase 42](42-agent-shell.md).
