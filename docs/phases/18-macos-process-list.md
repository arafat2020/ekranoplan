# Phase 18 — macOS process list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 16](16-process-trait.md). May be reviewed in parallel with [phase 17](17-linux-process-list.md).

## Goal

Implement `Processes::list` for macOS.

## Files this review may add

```text
crates/platform-macos/src/process.rs
```

`Cargo.toml` may add the one crate used to enumerate processes, if phase 09 did not already add it.

## Behavior

A `#[cfg(target_os = "macos")]` test asserts that the current process pid is in the list.

## Review

- [ ] The file does not read `/proc`.
- [ ] The file does not spawn or kill a process.
- [ ] `platform-linux` is untouched.

## Not in this phase

The wire message and the CLI.
