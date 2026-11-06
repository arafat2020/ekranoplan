# Ekranoplan

Architecture notes for the Quasar solution in `Arafat/Quasar`.

Quasar is a Windows remote-administration application written in C# on .NET Framework. One program listens. The other connects out to it. Every feature is a message type on a shared bus.

These notes describe how the code is organized and how a session runs. They are a map of the source tree, not a build or operator guide.

## Contents

| Document | What it covers |
|---|---|
| [Overview](overview.md) | What the system is, and the shape of a session |
| [Solution layout](solution-layout.md) | The three projects and what each folder is for |
| [Startup](startup.md) | How the server console and the client agent boot |
| [Networking](networking.md) | Who connects to whom, TLS, framing, and the handshake |
| [Message bus](message-bus.md) | `IMessage`, handlers, and how a request becomes a reply |
| [Features](features.md) | Which window, handler, and message family belong together |
| [Extending](extending.md) | The four pieces a new feature has to add |
| [Platform-agnostic Rust](platform-agnostic-rust.md) | Why the current code is Windows-only, and a Rust split that runs on macOS, Linux, and a headless Linux server |
| [Port phases](phases/README.md) | One review per file for the Rust port. [Target tree](port-phases.md) |

## Source of truth

The running code is under `/Users/betopia/Desktop/Arafat/Quasar`. When these notes and the code disagree, the code wins.
