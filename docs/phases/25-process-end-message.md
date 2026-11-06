# Phase 25 — DoProcessEnd message

One pull request. Rules: [README](README.md).

**Depends on:** [phase 22](22-process-end-trait.md). The OS backends are not required. The handler calls the trait.

## Goal

Add `DoProcessEnd` and `DoProcessResponse`. The agent handler calls `Processes::end` and sends the result. `Unsupported` becomes the error flag on the response. There is no new error channel.

## Files this review may add or edit

```text
crates/protocol/proto/processes.proto
bins/agent/src/handlers/processes.rs
crates/transport/tests/end_process.rs
```

The test uses the fake, which records the pid.

## Review

- [ ] `DoProcessStart` is not in the proto file.
- [ ] The test fails if a real signal is sent. The fake has no signal call. The test should not need a live pid.
- [ ] The CLI is unchanged.

## Not in this phase

The CLI subcommand. That is [phase 26](26-cli-process-end.md).
