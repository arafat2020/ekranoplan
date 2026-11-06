# Phase 47 — NetTable trait

One pull request. Rules: [README](README.md).

**Depends on:** [phase 46](46-cli-capabilities.md).

## Goal

Define a read-only TCP table and a fake with two rows. Closing a connection is not a method.

## Files this review may add

```text
crates/platform-api/src/net.rs
```

```rust
struct TcpConnection { protocol: String, local: String, remote: String, state: String }
trait NetTable {
    fn connections(&self) -> Vec<TcpConnection>;
}
```

## Review

- [ ] The trait has no `close` method.
- [ ] The words `kill` and `close` do not appear in the new file.
- [ ] The fake test expects two rows.

## Not in this phase

OS backends. Those are [48](48-linux-net.md) and [49](49-macos-net.md).
