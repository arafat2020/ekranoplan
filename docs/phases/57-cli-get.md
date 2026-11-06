# Phase 57 — CLI download

One pull request. Rules: [README](README.md).

**Depends on:** [phase 56](56-server-transfer.md).

## Goal

`ekranoplan-cli get <session-id> <remote-path> <local-path>` asks for the file and writes the assembled bytes to `local-path`.

## Files this review may add or edit

```text
bins/cli/src/commands/get.rs
bins/server/src/admin.rs
```

## Review

- [ ] There is no `put` command.
- [ ] Unknown session id fails before a file is created.
- [ ] A cancel from the agent leaves no partial file, or the pull request documents that the partial file is removed and the test checks that.

## Not in this phase

Upload, remote delete, remote rename.

The scheduled phases end here. Work that was considered and left out is in [later.md](later.md).
