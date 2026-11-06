# Phase 28 — Linux process start

One pull request. Rules: [README](README.md).

**Depends on:** [phase 27](27-process-start-trait.md). May be reviewed in parallel with [phase 29](29-macos-process-start.md).

## Goal

Implement `Processes::start` on Linux with `std::process::Command`. Document, in a comment on the function, how the single command string is split into an argv. The protocol crate does not split it.

## Files this review may edit

```text
crates/platform-linux/src/process.rs
```

## Review

- [ ] `Command::new("sh")` does not appear. This is not the shell feature.
- [ ] `platform-macos` is untouched.
- [ ] No message type is added.

## Not in this phase

`DoProcessStart` on the wire. `DoShellExecute`.
