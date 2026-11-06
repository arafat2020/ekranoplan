# Phase 44 — Capability constants

One pull request. Rules: [README](README.md).

**Depends on:** [phase 43](43-cli-shell.md).

## Goal

Name the capability bits in one module. No binary sets them yet. Registry, startup, and desktop exist as constants whose value is defined and unused, so later code does not invent a second numbering.

## Files this review may add

```text
crates/protocol/src/caps.rs
crates/protocol/tests/caps.rs
```

Bits: host, process, files, shell, connections. Constants that stay unset in the agent: registry, startup, desktop.

## Behavior

The test builds a set with host and process, and asserts registry is not in it.

## Review

- [ ] No handler is added.
- [ ] The agent binary is unchanged.
- [ ] Bit numbers are constants, not derived from type discovery.

## Not in this phase

Setting the bits from a linked platform crate. That is [phase 45](45-agent-capabilities.md).
