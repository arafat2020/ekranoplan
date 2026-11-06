# Phase 21 — CLI process list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 20](20-agent-process-list.md).

## Goal

`ekranoplan-cli ps <session-id>` prints pid and name for that session.

## Files this review may add

```text
bins/cli/src/commands/ps.rs
```

`main.rs` may add the subcommand. The server admin socket may gain one request variant that sends `GetProcesses` on the chosen session and returns the response rows. That variant lives in `bins/server/src/admin.rs` and is the only server edit allowed.

## Review

- [ ] The command prints two columns, pid and name.
- [ ] An unknown session id fails locally and does not open a new agent connection.
- [ ] No kill or start flag is accepted.

## Not in this phase

`DoProcessEnd` and `DoProcessStart`.
