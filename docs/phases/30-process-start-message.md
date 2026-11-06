# Phase 30 — DoProcessStart message

One pull request. Rules: [README](README.md).

**Depends on:** [phase 27](27-process-start-trait.md).

## Goal

Add `DoProcessStart`. The agent handler calls `Processes::start` and returns the pid in `DoProcessResponse`. The test uses the fake, which stores the command and does not spawn.

## Files this review may add or edit

```text
crates/protocol/proto/processes.proto
bins/agent/src/handlers/processes.rs
crates/transport/tests/start_process.rs
```

## Review

- [ ] The test fails if the recorded command string differs from the request.
- [ ] `sh` is not invoked anywhere in the diff.
- [ ] The CLI is unchanged.

## Not in this phase

The CLI subcommand. That is [phase 31](31-cli-process-start.md). The shell trait.
