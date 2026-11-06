# Phases

Each file in this folder is one pull request for the Rust port. A reviewer should be able to read that diff in one sitting. The file lists the only paths the diff may touch.

The end-state tree and the crate dependency rules live in [port-phases.md](../port-phases.md). The reason for the split lives in [platform-agnostic-rust.md](../platform-agnostic-rust.md).

A phase starts only after the phase it depends on is merged. Two phases may be reviewed at the same time only when both say so.

Keylogger, password recovery, the registry editor, and Windows startup (`schtasks`, the Run key) are not phases. They stay in the C# client. Deferred work is listed in [later.md](later.md).

## Review rules

| Check | Pass looks like |
|---|---|
| Scope | The diff only contains the files listed in that phase. |
| Direction | `protocol` has no sockets. `platform-api` has no syscalls. The CLI does not depend on a platform crate. |
| Behavior | The test named in the phase fails if the new behavior is removed. |
| Non-goals | Nothing under "Not in this phase" was added early. |

## Order

| Phase | Review |
|---|---|
| [00](00-workspace.md) | Empty workspace compiles |
| [01](01-codec.md) | Length-prefix codec |
| [02](02-identification-messages.md) | Handshake messages only |
| [03](03-handler-bus.md) | In-memory handler bus |
| [04](04-plain-tcp.md) | Loopback TCP frames, no TLS |
| [05](05-tls.md) | Certificate pin on that loopback |
| [06](06-identification-gate.md) | No feature frame before identification |
| [07](07-host-info-trait.md) | `HostInfo` trait and a fake |
| [08](08-linux-host-info.md) | Linux `HostInfo` |
| [09](09-macos-host-info.md) | macOS `HostInfo` |
| [10](10-server-session.md) | Server records one identified session |
| [11](11-agent-identify.md) | Agent dials and identifies |
| [12](12-cli-sessions.md) | CLI lists sessions |
| [13](13-server-config.md) | Server TOML config |
| [14](14-agent-config.md) | Agent TOML config |
| [15](15-reconnect.md) | Reconnect one host |
| [16](16-process-trait.md) | `Processes::list` and a fake |
| [17](17-linux-process-list.md) | Linux process list |
| [18](18-macos-process-list.md) | macOS process list |
| [19](19-process-messages.md) | `GetProcesses` messages |
| [20](20-agent-process-list.md) | Agent answers `GetProcesses` |
| [21](21-cli-ps.md) | CLI prints the process list |
| [22](22-process-end-trait.md) | `Processes::end` on the fake |
| [23](23-linux-process-end.md) | Linux `end` |
| [24](24-macos-process-end.md) | macOS `end` |
| [25](25-process-end-message.md) | `DoProcessEnd` handler |
| [26](26-cli-process-end.md) | CLI asks to end a pid |
| [27](27-process-start-trait.md) | `Processes::start` on the fake |
| [28](28-linux-process-start.md) | Linux `start` |
| [29](29-macos-process-start.md) | macOS `start` |
| [30](30-process-start-message.md) | `DoProcessStart` handler |
| [31](31-cli-process-start.md) | CLI asks to start a process |
| [32](32-files-trait.md) | `Files` trait and a fake |
| [33](33-linux-files.md) | Linux directory list |
| [34](34-macos-files.md) | macOS directory list |
| [35](35-files-messages.md) | Directory messages |
| [36](36-agent-files.md) | Agent answers a directory list |
| [37](37-cli-ls.md) | CLI prints a directory |
| [38](38-shell-trait.md) | `Shell` trait and a fake |
| [39](39-linux-shell.md) | Linux one-shot shell |
| [40](40-macos-shell.md) | macOS one-shot shell |
| [41](41-shell-messages.md) | Shell messages |
| [42](42-agent-shell.md) | Agent runs one command |
| [43](43-cli-shell.md) | CLI prints command output |
| [44](44-capability-constants.md) | Capability bit constants |
| [45](45-agent-capabilities.md) | Agent sets its bits |
| [46](46-cli-capabilities.md) | CLI shows the bits |
| [47](47-net-trait.md) | `NetTable` trait and a fake |
| [48](48-linux-net.md) | Linux TCP table |
| [49](49-macos-net.md) | macOS TCP table |
| [50](50-connection-messages.md) | Connection list messages |
| [51](51-agent-connections.md) | Agent answers the connection list |
| [52](52-cli-connections.md) | CLI prints connections |
| [53](53-multi-host.md) | Agent dials the next host |
| [54](54-transfer-messages.md) | Download chunk messages |
| [55](55-agent-transfer.md) | Agent sends chunks |
| [56](56-server-transfer.md) | Server assembles chunks |
| [57](57-cli-get.md) | CLI writes the assembled file |

Phases 08 and 09 can be reviewed together after 07. The same is true of 17/18, 23/24, 28/29, 33/34, 39/40, and 48/49. Nothing else overlaps.
