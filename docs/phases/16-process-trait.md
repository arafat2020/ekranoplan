# Phase 16 — Processes trait

One pull request. Rules: [README](README.md).

**Depends on:** [phase 15](15-reconnect.md).

## Goal

Define a read-only process list and a fake that returns two rows. Starting and ending a process are later phases.

## Files this review may add or edit

```text
crates/platform-api/src/process.rs
crates/platform-api/src/fake.rs
```

```rust
struct ProcessInfo { pid: u32, name: String }
trait Processes {
    fn list(&self) -> Vec<ProcessInfo>;
}
```

## Review

- [ ] `ProcessInfo` has no window title and no Windows session id.
- [ ] `start` and `end` are not methods on the trait.
- [ ] The fake test expects exactly two rows.
- [ ] No `/proc` parsing, no `.proto` file, no CLI command.

## Not in this phase

Linux and macOS backends. Those are [17](17-linux-process-list.md) and [18](18-macos-process-list.md), and they may be reviewed in parallel after this merges.
