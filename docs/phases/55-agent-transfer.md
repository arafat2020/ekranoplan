# Phase 55 — Agent sends chunks

One pull request. Rules: [README](README.md).

**Depends on:** [phase 54](54-transfer-messages.md).

## Goal

On `FileTransferRequest`, the agent reads the file through a new `Files::read_chunks` method and sends `FileTransferChunk` messages, then `FileTransferComplete`. The fake yields two fixed chunks and does not touch the disk.

Chunk size is a named constant. One chunk plus the protobuf wrapper stays under 5 MB.

## Files this review may add or edit

```text
crates/platform-api/src/files.rs
crates/platform-api/src/fake.rs
bins/agent/src/handlers/transfer.rs
crates/transport/tests/transfer_send.rs
```

Linux and macOS `files.rs` may add `read_chunks` as a thin wrapper over `std::fs::File`. If that makes the diff too large, land the wrappers as a follow-up comment in the pull request and keep this review on the fake only. The OS wrappers are then not part of this phase.

## Review

- [ ] The test assembles the two fake chunks in order.
- [ ] `remove_file` does not appear.
- [ ] The server crate is unchanged.

## Not in this phase

Writing the file on the server. Upload. Delete.
