# Phase 07 — HostInfo trait

One pull request. Rules: [README](README.md).

**Depends on:** [phase 06](06-identification-gate.md).

## Goal

Define how both operating systems report host identity. Ship a fake implementation the later tests can use. Do not call the operating system.

## Files this review may add

```text
crates/platform-api/src/host.rs
crates/platform-api/src/fake.rs
```

```rust
trait HostInfo {
    fn os_name(&self) -> String;
    fn hostname(&self) -> String;
    fn username(&self) -> String;
    fn host_id(&self) -> String;
}
```

`fake.rs` returns fixed strings. `lib.rs` exports the module.

## Review

- [ ] `platform-api/Cargo.toml` does not list `libc`, `sysinfo`, `nix`, or `objc`.
- [ ] The trait has no method named for Windows (`is_vista`, registry hive, drive letter).
- [ ] A unit test reads the four fake strings.

## Not in this phase

`/proc`, `libproc`, the agent calling this trait.

This phase blocks [08](08-linux-host-info.md) and [09](09-macos-host-info.md). Those two may be reviewed in parallel.
