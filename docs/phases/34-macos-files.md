# Phase 34 — macOS directory list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 32](32-files-trait.md). May be reviewed in parallel with [phase 33](33-linux-files.md).

## Goal

Implement `Files` on macOS. `list_roots` returns `/`. `list_dir` uses `std::fs`.

## Files this review may add

```text
crates/platform-macos/src/files.rs
```

## Behavior

Same temporary-directory test as phase 33, gated on `target_os = "macos"`.

## Review

- [ ] `remove_file` and `rename` are absent.
- [ ] `platform-linux` is untouched.

## Not in this phase

The wire message and the CLI.
