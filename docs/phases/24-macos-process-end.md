# Phase 24 — macOS process end

One pull request. Rules: [README](README.md).

**Depends on:** [phase 22](22-process-end-trait.md). May be reviewed in parallel with [phase 23](23-linux-process-end.md).

## Goal

Implement `Processes::end` on macOS. Same limit as phase 23: no test kills an arbitrary live pid.

## Files this review may edit

```text
crates/platform-macos/src/process.rs
```

## Review

- [ ] The file does not call Linux-only APIs.
- [ ] `platform-linux` is untouched.
- [ ] No `.proto` file changes.

## Not in this phase

The wire message and the CLI.
