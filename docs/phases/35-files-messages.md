# Phase 35 — Directory messages

One pull request. Rules: [README](README.md).

**Depends on:** [phase 32](32-files-trait.md).

## Goal

Add `GetDrives`, `GetDrivesResponse`, `GetDirectory`, and `GetDirectoryResponse`. Nothing sends them yet.

## Files this review may add

```text
crates/protocol/proto/files.proto
crates/protocol/tests/files.rs
```

## Behavior

The test frames a directory response with two entries and decodes the same names and sizes.

## Review

- [ ] `FileTransferChunk`, `DoPathDelete`, and `DoPathRename` are absent.
- [ ] Agent and CLI crates are unchanged.

## Not in this phase

The handler. That is [phase 36](36-agent-files.md).
