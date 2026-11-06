# Phase 29 — macOS process start

One pull request. Rules: [README](README.md).

**Depends on:** [phase 27](27-process-start-trait.md). May be reviewed in parallel with [phase 28](28-linux-process-start.md).

## Goal

Implement `Processes::start` on macOS. Same split rules as phase 28, documented on the function.

## Files this review may edit

```text
crates/platform-macos/src/process.rs
```

## Review

- [ ] `Command::new("sh")` does not appear.
- [ ] `platform-linux` is untouched.
- [ ] No message type is added.

## Not in this phase

The wire message and the CLI.
