# Phase 32 — Files trait

One pull request. Rules: [README](README.md).

**Depends on:** [phase 31](31-cli-process-start.md).

## Goal

Define a read-only directory listing and a fake. No delete, rename, or transfer.

## Files this review may add

```text
crates/platform-api/src/files.rs
```

`fake.rs` may grow a `Files` impl with one root and two entries.

```rust
trait Files {
    fn list_roots(&self) -> Vec<Root>;
    fn list_dir(&self, path: &Path) -> Result<Vec<DirEntry>, Unsupported>;
}
```

`DirEntry` is name, file or directory, and size.

## Review

- [ ] `remove_file` and `rename` do not appear.
- [ ] The fake test lists the two fixed entries.
- [ ] No `.proto` file is added.

## Not in this phase

OS backends. Those are [33](33-linux-files.md) and [34](34-macos-files.md).
