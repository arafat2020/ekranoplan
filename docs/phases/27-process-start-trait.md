# Phase 27 — Process start on the fake

One pull request. Rules: [README](README.md).

**Depends on:** [phase 22](22-process-end-trait.md).

## Goal

Add `start` to the trait. The fake stores the command string and returns a fixed pid. It does not spawn anything.

## Files this review may edit

```text
crates/platform-api/src/process.rs
crates/platform-api/src/fake.rs
```

```rust
fn start(&self, command: &str) -> Result<u32, Unsupported>;
```

## Review

- [ ] The test reads the stored command string back.
- [ ] `std::process::Command` does not appear in `platform-api`.
- [ ] OS crates are untouched.

## Not in this phase

`sh -c`. A shell is a different trait. OS backends are phases 28 and 29.
