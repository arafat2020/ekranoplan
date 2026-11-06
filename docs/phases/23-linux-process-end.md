# Phase 23 — Linux process end

One pull request. Rules: [README](README.md).

**Depends on:** [phase 22](22-process-end-trait.md). May be reviewed in parallel with [phase 24](24-macos-process-end.md).

## Goal

Implement `Processes::end` on Linux. Return the platform error when the call is refused. Do not add a test that kills an arbitrary pid.

## Files this review may edit

```text
crates/platform-linux/src/process.rs
```

## Review

- [ ] The only new test uses a pid that is already dead, or it checks the error path without signaling a live process. The pull request states which.
- [ ] `platform-macos` is untouched.
- [ ] No `.proto` file changes.

## Not in this phase

`DoProcessEnd` on the wire. The CLI.
