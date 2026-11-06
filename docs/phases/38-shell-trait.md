# Phase 38 — Shell trait

One pull request. Rules: [README](README.md).

**Depends on:** [phase 37](37-cli-ls.md).

## Goal

Define a one-shot command runner. The fake copies the command into stdout and does not spawn a process.

## Files this review may add

```text
crates/platform-api/src/shell.rs
```

```rust
struct ShellOutput { exit_code: i32, stdout: Vec<u8>, stderr: Vec<u8> }
trait Shell {
    fn exec(&self, command: &str) -> Result<ShellOutput, Unsupported>;
}
```

## Review

- [ ] The fake test compares stdout to the command bytes.
- [ ] `Command` is not imported in `platform-api`.
- [ ] No PTY type is declared.

## Not in this phase

OS backends. Those are [39](39-linux-shell.md) and [40](40-macos-shell.md).
