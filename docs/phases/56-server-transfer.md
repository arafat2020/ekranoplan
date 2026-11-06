# Phase 56 — Server assembles chunks

One pull request. Rules: [README](README.md).

**Depends on:** [phase 55](55-agent-transfer.md).

## Goal

The server stores an in-progress download keyed by session id and transfer id. Two chunks become one byte buffer. `FileTransferCancel` drops the buffer.

## Files this review may add

```text
bins/server/src/transfers.rs
bins/server/tests/assemble.rs
```

The test does not open a socket. It feeds two chunk values into `transfers.rs` and compares the bytes.

## Review

- [ ] The map key is session id plus transfer id, so two sessions do not share a buffer.
- [ ] Cancel leaves no bytes to read.
- [ ] The CLI is unchanged.
- [ ] The file is not written to disk in this phase.

## Not in this phase

The CLI command that chooses the output path. That is [phase 57](57-cli-get.md).
