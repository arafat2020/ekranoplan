# Phase 12 — CLI lists sessions

One pull request. Rules: [README](README.md).

**Depends on:** [phase 11](11-agent-identify.md).

## Goal

The server listens on a local socket. The CLI's only command asks that socket for the session rows.

## Files this review may add

```text
bins/server/src/admin.rs
bins/cli/src/main.rs
bins/cli/src/commands/sessions.rs
```

On Linux and macOS the socket path is an argument. The default for this phase is `/tmp/ekranoplan-admin.sock`, mode `0600`. The request is a short length-prefixed list command. It is not TLS, and it does not forward an agent frame.

## Behavior

Two agents produce two rows. A CLI pointed at a missing socket prints a local error and exits non-zero.

## Review

- [ ] `bins/cli/Cargo.toml` does not depend on `platform-linux` or `platform-macos`.
- [ ] `admin.rs` does not import the TLS session type except to read the already-stored rows.
- [ ] There is no `ps`, `ls`, or `shell` command.

## Not in this phase

Process list, file list, shell, a desktop UI.
