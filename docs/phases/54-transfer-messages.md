# Phase 54 — Download messages

One pull request. Rules: [README](README.md).

**Depends on:** [phase 35](35-files-messages.md). Placed after the connection list so each feature finishes before the next starts.

## Goal

Add the four download messages. Nothing reads a file yet.

## Files this review may add

```text
crates/protocol/proto/transfer.proto
crates/protocol/tests/transfer.rs
```

Messages: `FileTransferRequest`, `FileTransferChunk`, `FileTransferComplete`, `FileTransferCancel`.

## Behavior

The test frames two chunks and decodes them in order.

## Review

- [ ] There is no upload message.
- [ ] Agent, server, and CLI are unchanged.
- [ ] Chunk payload in the test is a few bytes, not a 5 MB fixture.

## Not in this phase

The agent sender. That is [phase 55](55-agent-transfer.md).
