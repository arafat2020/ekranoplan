# Phase 42 — Agent one-shot shell

One pull request. Rules: [README](README.md).

**Depends on:** [phase 41](41-shell-messages.md).

## Goal

The agent handler calls `Shell::exec` and sends `DoShellExecuteResponse`. Output longer than the documented cap is cut and the truncated flag is set. The test uses the fake, which does not spawn `sh`.

## Files this review may add

```text
bins/agent/src/handlers/shell.rs
crates/transport/tests/shell_exec.rs
```

## Review

- [ ] The test fails if stdout is not the command the fake echoed.
- [ ] A second test feeds an oversized fake stdout and expects `truncated = true`.
- [ ] `Command::new` does not appear in the agent crate.

## Not in this phase

The CLI. An interactive session.
