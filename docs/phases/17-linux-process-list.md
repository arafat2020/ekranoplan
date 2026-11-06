# Phase 17 — Linux process list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 16](16-process-trait.md). May be reviewed in parallel with [phase 18](18-macos-process-list.md).

## Goal

Implement `Processes::list` by reading `/proc/<pid>/comm`.

## Files this review may add

```text
crates/platform-linux/src/process.rs
```

## Behavior

A `#[cfg(target_os = "linux")]` test asserts that the current process pid is in the list.

## Review

- [ ] The file does not spawn or kill a process.
- [ ] `start` and `end` are not implemented.
- [ ] `platform-macos` is untouched.

## Not in this phase

The wire message and the CLI.
