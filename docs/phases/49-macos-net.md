# Phase 49 — macOS TCP table

One pull request. Rules: [README](README.md).

**Depends on:** [phase 47](47-net-trait.md). May be reviewed in parallel with [phase 48](48-linux-net.md).

## Goal

Implement `NetTable::connections` with the same process-listing crate already used on macOS, or the platform API that crate wraps.

## Files this review may add

```text
crates/platform-macos/src/net.rs
```

## Review

- [ ] The file does not read `/proc/net/tcp`.
- [ ] It does not close a connection.
- [ ] `platform-linux` is untouched.

## Not in this phase

The wire message and the CLI.
