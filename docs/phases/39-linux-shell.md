# Phase 39 — Linux one-shot shell

One pull request. Rules: [README](README.md).

**Depends on:** [phase 38](38-shell-trait.md). May be reviewed in parallel with [phase 40](40-macos-shell.md).

## Goal

Implement `Shell::exec` on Linux as `sh -c` with the command string. Capture stdout and stderr. This is one process, then it exits. It is not an interactive terminal.

## Files this review may add

```text
crates/platform-linux/src/shell.rs
```

## Behavior

A `#[cfg(target_os = "linux")]` test runs `printf hi` and expects stdout `hi`.

## Review

- [ ] This is the only file in the diff that contains `Command::new("sh")`.
- [ ] No PTY crate is added.
- [ ] `platform-macos` is untouched.

## Not in this phase

The wire message, the CLI, `cmd.exe`.
