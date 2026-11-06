# Phase 33 — Linux directory list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 32](32-files-trait.md). May be reviewed in parallel with [phase 34](34-macos-files.md).

## Goal

Implement `Files` on Linux. `list_roots` returns one root, `/`. `list_dir` uses `std::fs`.

## Files this review may add

```text
crates/platform-linux/src/files.rs
```

## Behavior

A `#[cfg(target_os = "linux")]` test lists a temporary directory the test itself created, and sees a file the test wrote.

## Review

- [ ] `remove_file` and `rename` are absent.
- [ ] `platform-macos` is untouched.
- [ ] No message type is added.

## Not in this phase

Upload, download, delete.
