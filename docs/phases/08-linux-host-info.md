# Phase 08 — Linux HostInfo

One pull request. Rules: [README](README.md).

**Depends on:** [phase 07](07-host-info-trait.md). May be reviewed in parallel with [phase 09](09-macos-host-info.md).

## Goal

Implement `HostInfo` for Linux only.

## Files this review may add

```text
crates/platform-linux/src/host.rs
```

`lib.rs` may add `mod host`.

Hostname and OS name come from `uname` and `/etc/os-release`. `host_id` is `/etc/machine-id` when that file exists. The fallback, when the file is missing, is documented in a comment on the function and stays stable for the life of that install.

## Behavior

A test marked `#[cfg(target_os = "linux")]` asserts that hostname is non-empty.

## Review

- [ ] `process.rs`, `files.rs`, and `shell.rs` do not appear.
- [ ] No `#[cfg(target_os = "macos")]` block is in this crate.
- [ ] The test does not run on a macOS CI job. That is expected.

## Not in this phase

The macOS backend. A shared OS-release parser in `platform-api`. The agent binary.
