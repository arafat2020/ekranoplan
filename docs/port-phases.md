# Port phases

Each review is one file under [phases](phases/README.md). Start at [phase 00](phases/00-workspace.md). This page only keeps the tree those phases fill, so the phase files do not repeat it.

Early phases create the crates. Later phases add files inside them. They do not rename crates.

## Target tree

```text
ekranoplan/
├── Cargo.toml
├── rust-toolchain.toml
├── crates/
│   ├── protocol/
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── proto/
│   │   ├── src/
│   │   └── tests/
│   ├── transport/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── tests/
│   ├── platform-api/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── platform-linux/
│   │   ├── Cargo.toml
│   │   └── src/
│   └── platform-macos/
│       ├── Cargo.toml
│       └── src/
└── bins/
    ├── agent/
    ├── server/
    └── cli/
```

## Dependency direction

```text
cli  →  server admin API only
server  →  transport  →  protocol
agent   →  transport  →  protocol
agent   →  platform-api  ←  platform-linux | platform-macos
protocol has no tokio, no rustls, no libc
platform-api has no tokio and no syscall crates
```

The phase index and the review rules are in [phases/README.md](phases/README.md).
