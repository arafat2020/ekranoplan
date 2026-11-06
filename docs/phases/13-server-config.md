# Phase 13 — Server config

One pull request. Rules: [README](README.md).

**Depends on:** [phase 12](12-cli-sessions.md).

## Goal

Remove the server constants added in phase 10. Read them from a TOML file.

## Files this review may add

```text
bins/server/src/config.rs
bins/server/config.example.toml
```

`main.rs` may change only to call the loader.

Fields: bind address, port, certificate path, private key path, admin socket path. A missing file or a missing field exits with a message that names the field.

## Review

- [ ] `config.example.toml` uses `127.0.0.1` and a relative fixture path.
- [ ] `config.rs` does not bind a socket. `main.rs` still does that.
- [ ] The agent binary is unchanged.

## Not in this phase

Agent config, embedding settings in the binary, the C# builder's ciphertext.
