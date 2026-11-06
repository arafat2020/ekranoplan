# Phase 48 — Linux TCP table

One pull request. Rules: [README](README.md).

**Depends on:** [phase 47](47-net-trait.md). May be reviewed in parallel with [phase 49](49-macos-net.md).

## Goal

Implement `NetTable::connections` by reading `/proc/net/tcp`.

## Files this review may add

```text
crates/platform-linux/src/net.rs
```

## Behavior

A `#[cfg(target_os = "linux")]` test asserts the returned list is readable. It does not assert a specific remote address, because that depends on the machine.

## Review

- [ ] The file does not open or close a socket except the read of `/proc/net/tcp`.
- [ ] `platform-macos` is untouched.
- [ ] No `.proto` change.

## Not in this phase

`DoCloseConnection`. The CLI.
