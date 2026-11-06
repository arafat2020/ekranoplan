# Phase 53 — Next host

One pull request. Rules: [README](README.md).

**Depends on:** [phase 15](15-reconnect.md). It can be reviewed any time after reconnect. It is placed here so it does not land in the middle of a feature.

## Goal

The agent config `hosts` becomes a list. When a dial fails, try the next host. Phase 15 left this out on purpose.

## Files this review may edit

```text
bins/agent/src/config.rs
bins/agent/src/reconnect.rs
bins/agent/config.example.toml
```

## Behavior

The example file has two `127.0.0.1` entries with different ports. A unit test of host selection returns the second host after the first dial returns an error.

## Review

- [ ] No new crate is added unless the pull request shows why `std` cannot parse the host.
- [ ] No new message type is added.
- [ ] Server config is unchanged.

## Not in this phase

A feature handler.
