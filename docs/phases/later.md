# Later

These are not scheduled. When one is written, it gets its own file in this folder and follows the same rules: one pull request, one behavior, an explicit file list.

| Name | Why it waits |
|---|---|
| Upload | The opposite direction of [phase 57](57-cli-get.md), and a separate review. |
| Interactive shell | A long-lived stream. [Phase 43](43-cli-shell.md) is one response and then it ends. |
| Close a TCP connection | A write on the table that [phase 52](52-cli-connections.md) only reads. |
| Reboot or power off | Easy to call by mistake. No standby on a server. |
| Reverse proxy | Extra sockets beside the TLS session. Its own module. |
| `platform-windows` | The same traits, after Linux and macOS are already dull. |
| Desktop capture | Stays off for a headless server. Not a branch inside `platform-linux`. |

Not scheduled at all: keylogger, password recovery, registry editor, and Windows startup (`schtasks`, the Run key). Those stay in the C# client.
