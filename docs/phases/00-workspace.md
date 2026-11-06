# Phase 00 — Workspace skeleton

One pull request. Rules: [README](README.md).

**Depends on:** nothing. This is the first phase.

## Goal

Create the cargo workspace and the empty crates. Every crate compiles. None of them do anything.

## Files this review may add

```text
ekranoplan/Cargo.toml
ekranoplan/rust-toolchain.toml
crates/protocol/Cargo.toml
crates/protocol/src/lib.rs
crates/transport/Cargo.toml
crates/transport/src/lib.rs
crates/platform-api/Cargo.toml
crates/platform-api/src/lib.rs
crates/platform-linux/Cargo.toml
crates/platform-linux/src/lib.rs
crates/platform-macos/Cargo.toml
crates/platform-macos/src/lib.rs
bins/agent/Cargo.toml
bins/agent/src/main.rs
bins/server/Cargo.toml
bins/server/src/main.rs
bins/cli/Cargo.toml
bins/cli/src/main.rs
```

Each `lib.rs` and `main.rs` is a short module comment and an empty body. `main` returns `Ok(())`.

`platform-linux` and `platform-macos` depend on `platform-api` only. The agent depends on `transport` and `platform-api`. The server and the CLI depend on `std` only.

## Review

- [ ] `cargo build --workspace` succeeds.
- [ ] `protocol/Cargo.toml` does not list `tokio`, `rustls`, `prost`, or `libc`.
- [ ] No `src` file outside the list above was added.

## Not in this phase

Messages, tests other than the build, config files, traits, sockets.
