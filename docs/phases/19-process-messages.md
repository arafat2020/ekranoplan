# Phase 19 — Process list messages

One pull request. Rules: [README](README.md).

**Depends on:** [phase 16](16-process-trait.md). It does not depend on the OS backends. The messages carry `ProcessInfo` fields, not a trait object.

## Goal

Add `GetProcesses` and `GetProcessesResponse`. No handler sends them yet.

## Files this review may add

```text
crates/protocol/proto/processes.proto
crates/protocol/tests/processes.rs
```

## Behavior

The test builds a response with two processes, frames it, and decodes the same pids and names.

## Review

- [ ] The proto file has two messages.
- [ ] `DoProcessStart` and `DoProcessEnd` are absent.
- [ ] `bins/agent` and `bins/cli` are unchanged.

## Not in this phase

The agent handler. That is [phase 20](20-agent-process-list.md).
