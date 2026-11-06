# Phase 45 — Agent sets capability bits

One pull request. Rules: [README](README.md).

**Depends on:** [phase 44](44-capability-constants.md).

## Goal

Move identification construction out of `main.rs`. The agent sets host, process, files, and shell because those handlers are linked. It does not set registry, startup, or desktop. The bits are not a config flag.

## Files this review may add or edit

```text
bins/agent/src/identify.rs
bins/agent/src/main.rs
```

## Review

- [ ] A test of `identify.rs` expects the four bits on and registry off.
- [ ] `config.rs` does not grow a capabilities field.
- [ ] No registry handler file is created.

## Not in this phase

Printing the bits in the CLI. That is [phase 46](46-cli-capabilities.md).
