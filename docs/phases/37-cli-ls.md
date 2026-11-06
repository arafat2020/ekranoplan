# Phase 37 — CLI directory list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 36](36-agent-files.md).

## Goal

`ekranoplan-cli ls <session-id> <path>` prints the directory entries. A roots command prints the root list.

## Files this review may add or edit

```text
bins/cli/src/commands/ls.rs
bins/server/src/admin.rs
```

## Review

- [ ] Two subcommands at most: roots and ls. No get, put, or rm.
- [ ] Unknown session id fails locally.
- [ ] `admin.rs` adds the list request only.

## Not in this phase

File download. That starts at [phase 54](54-transfer-messages.md).
