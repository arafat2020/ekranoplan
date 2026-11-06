# Phase 20 — Agent answers GetProcesses

One pull request. Rules: [README](README.md).

**Depends on:** [phase 19](19-process-messages.md). Uses the fake `Processes` in the test. The Linux and macOS lists may already be merged. They are not required for this review.

## Goal

The agent handler receives `GetProcesses`, calls `Processes::list`, and sends `GetProcessesResponse`.

## Files this review may add

```text
bins/agent/src/handlers/processes.rs
crates/transport/tests/get_processes.rs
```

`main.rs` may register the handler. The test builds the transport with the fake backend, so it does not read `/proc`.

## Review

- [ ] The test fails if the handler returns an empty list while the fake has two rows.
- [ ] The CLI is unchanged.
- [ ] The handler does not call `std::process::Command`.

## Not in this phase

Ending or starting a process. The `ps` command. That is [phase 21](21-cli-ps.md).
