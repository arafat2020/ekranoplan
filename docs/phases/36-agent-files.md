# Phase 36 — Agent directory list

One pull request. Rules: [README](README.md).

**Depends on:** [phase 35](35-files-messages.md).

## Goal

The agent handler answers `GetDrives` and `GetDirectory` from the `Files` trait. The test uses the fake.

## Files this review may add

```text
bins/agent/src/handlers/files.rs
crates/transport/tests/list_dir.rs
```

## Review

- [ ] The test expects the fake's two entries.
- [ ] The handler does not open a file for reading bytes.
- [ ] The CLI is unchanged.

## Not in this phase

`ls` in the CLI. Chunked transfer.
