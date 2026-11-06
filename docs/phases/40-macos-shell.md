# Phase 40 — macOS one-shot shell

One pull request. Rules: [README](README.md).

**Depends on:** [phase 38](38-shell-trait.md). May be reviewed in parallel with [phase 39](39-linux-shell.md).

## Goal

Implement `Shell::exec` on macOS as `sh -c`. One process, then it exits.

## Files this review may add

```text
crates/platform-macos/src/shell.rs
```

## Behavior

A `#[cfg(target_os = "macos")]` test runs `printf hi` and expects stdout `hi`.

## Review

- [ ] No PTY crate is added.
- [ ] `platform-linux` is untouched.
- [ ] No message type is added.

## Not in this phase

The wire message and the CLI.
