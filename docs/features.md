# Features

Each feature is a vertical slice: one or more message types in Common, a client handler, and on the server a handler plus a form. The listen socket and the dispatcher stay shared.

Paths below are relative to the project root.

## Windows that follow the task-manager pattern

These open from the client list in `Quasar.Server/Forms/FrmMain.cs`. The form is cached per connection with `CreateNewOrGetExisting` or the same idea. The handler is registered in the form constructor and unregistered on close.

| UI | Server form | Server handler | Client handler | Request messages |
|---|---|---|---|---|
| Task Manager | `Forms/FrmTaskManager.cs` | `Messages/TaskManagerHandler.cs` | `Messages/TaskManagerHandler.cs` | `GetProcesses`, `DoProcessStart`, `DoProcessEnd` |
| File Manager | `Forms/FrmFileManager.cs` | `Messages/FileManagerHandler.cs` | `Messages/FileManagerHandler.cs` | `GetDrives`, `GetDirectory`, plus path and transfer messages |
| Remote Desktop | `Forms/FrmRemoteDesktop.cs` | `Messages/RemoteDesktopHandler.cs` | `Messages/RemoteDesktopHandler.cs` | `GetMonitors`, `GetDesktop`, `DoMouseEvent`, `DoKeyboardEvent` |
| Remote Shell | `Forms/FrmRemoteShell.cs` | `Messages/RemoteShellHandler.cs` | `Messages/RemoteShellHandler.cs` | `DoShellExecute` |
| Registry Editor | `Forms/FrmRegistryEditor.cs` | `Messages/RegistryHandler.cs` | `Messages/RegistryHandler.cs` | `DoLoadRegistryKey` and the create, rename, delete, and change messages |
| Startup Manager | `Forms/FrmStartupManager.cs` | `Messages/StartupManagerHandler.cs` | `Messages/StartupManagerHandler.cs` | `GetStartupItems`, `DoStartupItemAdd`, `DoStartupItemRemove` |
| TCP connections | `Forms/FrmConnections.cs` | `Messages/TcpConnectionsHandler.cs` | `Messages/TcpConnectionsHandler.cs` | `GetConnections`, `DoCloseConnection` |
| System information | `Forms/FrmSystemInformation.cs` | `Messages/SystemInformationHandler.cs` | `Messages/SystemInformationHandler.cs` | `GetSystemInfo` |
| Reverse proxy | `Forms/FrmReverseProxy.cs` | `Messages/ReverseProxyHandler.cs` | `Messages/ReverseProxyHandler.cs` | `ReverseProxyConnect`, `ReverseProxyData`, `ReverseProxyDisconnect` |

File transfer uses its own small family in addition to directory listing: `FileTransferRequest`, `FileTransferChunk`, `FileTransferCancel`, `FileTransferComplete`. Chunk layout is `Quasar.Common/Models/FileChunk.cs` and `Quasar.Common/IO/FileSplit.cs`. The server tracks an in-progress transfer in `Quasar.Server/Models/FileTransfer.cs`.

Remote desktop frames go through `Quasar.Common/Video`: `Resolution`, `JpgCompression`, and `UnsafeStreamCodec`. The picture box on the form is `Quasar.Server/Controls/RapidPictureBox.cs`.

Registry editing has extra modal forms for each value kind (string, multi-string, word, binary). The binary editor is the hex control under `Quasar.Server/Controls/HexEditor/`.

## Actions that use a dialog, then a message

These do not keep a per-client tool window. `FrmMain` opens a small dialog and sends one message to each selected client.

| Dialog | Message | Client handler |
|---|---|---|
| `Forms/FrmVisitWebsite.cs` | `DoVisitWebsite` | `WebsiteVisitorHandler` |
| `Forms/FrmShowMessagebox.cs` | `DoShowMessageBox` | `MessageBoxHandler` |
| `Forms/FrmRemoteExecution.cs` | process start on the task-manager handler | `TaskManagerHandler` |

Shutdown, reconnect, disconnect, uninstall, and elevate are also single messages from the main window: `DoShutdownAction`, `DoClientReconnect`, `DoClientDisconnect`, `DoClientUninstall`, `DoAskElevate`. `ShutdownHandler` and `ClientServicesHandler` on the client own those.

## Background work on the client

These are running before any feature window opens, if the matching setting is on.

| Piece | Where | How the console sees it |
|---|---|---|
| Keylogger | `Logging/KeyloggerService.cs`, started from `QuasarApplication` when the logger setting is on | `KeyloggerHandler` on both sides and `Forms/FrmKeylogger.cs`. The client stores logs. The console asks for the log directory. |
| Activity | `User/ActivityDetection.cs` | Status updates on the connection, handled with the client status path on the server. |
| Identification geo lookup | `IpGeoLocation/` | Country and flag index inside `ClientIdentification`. Not a separate window. |

## Reverse proxy, in more detail

The proxy is the one feature with sockets besides the main session.

- `Quasar.Server/ReverseProxy/ReverseProxyServer.cs` accepts a connection on the operator machine.
- `Quasar.Server/ReverseProxy/ReverseProxyClient.cs` is one of those local sockets.
- The server handler sends `ReverseProxyConnect` over the existing TLS session.
- `Quasar.Client/ReverseProxy/ReverseProxyClient.cs` opens the outbound socket on the remote machine.
- Bytes in both directions are carried as `ReverseProxyData`.
- `ReverseProxyDisconnect` tears the pair down.

The agent also keeps a list of proxy clients on its main `Client` object, so a dropped main session can close them.

## Console-only windows

These never send a feature message to a connected machine.

| Window | Role |
|---|---|
| `Forms/FrmMain.cs` | Client list, context menu, listen control. |
| `Forms/FrmSettings.cs` | Listen port, IPv6, UPnP, auto-listen, and related server settings. |
| `Forms/FrmCertificate.cs` | Server certificate used for TLS and for the identification signature. |
| `Forms/FrmBuilder.cs` | Produces a Release client with connection settings embedded. |
| `Forms/FrmAbout.cs` | About box. |

Server settings live in `Quasar.Server/Models/Settings.cs`. Builder input is `Models/BuildOptions.cs` and `Models/BuilderProfile.cs`. The write step is `Build/ClientBuilder.cs`.

## Message families in Common

Grouped by the handler that owns them. Names are the class names under `Quasar.Common/Messages/`.

**Identity and session.** `ClientIdentification`, `ClientIdentificationResult`, `SetStatus`, `SetUserStatus`, `DoClientReconnect`, `DoClientDisconnect`, `DoClientUninstall`, `DoAskElevate`.

**Processes.** `GetProcesses`, `GetProcessesResponse`, `DoProcessStart`, `DoProcessEnd`, `DoProcessResponse`.

**Files.** `GetDrives`, `GetDrivesResponse`, `GetDirectory`, `GetDirectoryResponse`, `DoPathDelete`, `DoPathRename`, `SetStatusFileManager`, `FileTransferRequest`, `FileTransferChunk`, `FileTransferCancel`, `FileTransferComplete`.

**Desktop.** `GetMonitors`, `GetMonitorsResponse`, `GetDesktop`, `GetDesktopResponse`, `DoMouseEvent`, `DoKeyboardEvent`.

**Shell.** `DoShellExecute`, `DoShellExecuteResponse`.

**Registry.** `DoLoadRegistryKey`, `GetRegistryKeysResponse`, and the create, rename, delete, and change pairs for keys and values (`DoCreateRegistryKey`, `GetCreateRegistryKeyResponse`, and the matching value, rename, and delete types).

**Startup items.** `GetStartupItems`, `GetStartupItemsResponse`, `DoStartupItemAdd`, `DoStartupItemRemove`.

**Connections.** `GetConnections`, `GetConnectionsResponse`, `DoCloseConnection`.

**System.** `GetSystemInfo`, `GetSystemInfoResponse`, `DoShutdownAction`.

**Other.** `DoShowMessageBox`, `DoVisitWebsite`, `GetPasswords`, `GetPasswordsResponse`, `GetKeyloggerLogsDirectory`, `GetKeyloggerLogsDirectoryResponse`.

**Proxy.** `ReverseProxyConnect`, `ReverseProxyConnectResponse`, `ReverseProxyData`, `ReverseProxyDisconnect`.

Models those messages embed are under `Quasar.Common/Models/`: `Process`, `Drive`, `FileSystemEntry`, `FileChunk`, `StartupItem`, `TcpConnection`, `RegSeekerMatch`, `RegValueData`, `RecoveredAccount`.
