# Phase 22 — Process end on the fake

One pull request. Rules: [README](README.md).

**Depends on:** [phase 16](16-process-trait.md). Can be written after phase 21 so the list path is already reviewed. Depends on 21 only for order, not for types.

## Goal

Add `end` to the trait. The fake records the pid and does not signal a real process.

## Files this review may edit

```text
crates/platform-api/src/process.rs
crates/platform-api/src/fake.rs
```

```rust
fn end(&self, pid: u32) -> Result<(), Unsupported>;
```

`Unsupported` is an error type in `platform-api`, used when a backend cannot do the operation. This phase's fake returns `Ok(())` after recording the pid.

## Review

- [ ] The test reads the recorded pid back.
- [ ] The test does not send a signal.
- [ ] Linux and macOS files are untouched.
- [ ] `start` is not added.

## Not in this phase

The wire message. OS backends. Those are phases 23, 24, and 25.
