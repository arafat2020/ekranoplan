# Phase 09 — macOS HostInfo

One pull request. Rules: [README](README.md).

**Depends on:** [phase 07](07-host-info-trait.md). May be reviewed in parallel with [phase 08](08-linux-host-info.md).

## Goal

Implement `HostInfo` for macOS only.

## Files this review may add

```text
crates/platform-macos/src/host.rs
```

`lib.rs` may add `mod host`. `Cargo.toml` may add the one crate used to read the hostname and the platform UUID.

## Behavior

A test marked `#[cfg(target_os = "macos")]` asserts that hostname is non-empty and `host_id` is non-empty.

## Review

- [ ] This file does not parse `/etc/os-release`.
- [ ] No Linux `cfg` block is in this crate.
- [ ] `platform-api` is unchanged.

## Not in this phase

A crate that `cfg`-switches between Linux and macOS. The agent binary.
