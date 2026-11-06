# Phase 14 — Agent config

One pull request. Rules: [README](README.md).

**Depends on:** [phase 13](13-server-config.md).

## Goal

Remove the agent constants added in phase 11. Read them from a TOML file.

## Files this review may add

```text
bins/agent/src/config.rs
bins/agent/config.example.toml
```

`main.rs` may change only to call the loader.

Fields: one host, one port, pinned certificate path, reconnect delay in milliseconds. The delay is parsed and stored. Nothing sleeps in this phase.

## Review

- [ ] The example file has one host, `127.0.0.1`.
- [ ] `config.rs` does not dial.
- [ ] A missing `reconnect_delay` names that field in the error.

## Not in this phase

A host list, the reconnect loop, server config changes.
