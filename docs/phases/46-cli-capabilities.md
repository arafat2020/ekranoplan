# Phase 46 — CLI shows capability bits

One pull request. Rules: [README](README.md).

**Depends on:** [phase 45](45-agent-capabilities.md).

## Goal

The session list prints the capability names. `ps`, `ls`, and `shell` are omitted from the help line for a session that lacks the matching bit. The server still forwards a message if something sends it. The agent handler returns the error response when the trait returns `Unsupported`.

## Files this review may edit

```text
bins/cli/src/commands/sessions.rs
```

## Review

- [ ] No new feature handler is added.
- [ ] A session fixture with an empty bit set prints no feature names.
- [ ] Registry is not offered as a command.

## Not in this phase

Remote desktop, registry, startup manager.
