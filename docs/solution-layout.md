# Solution layout

`Quasar.sln` contains four projects.

```text
Quasar/
├── Quasar.sln
├── Quasar.Common/          shared protocol and models
├── Quasar.Common.Tests/    tests for the shared library
├── Quasar.Server/          operator console
└── Quasar.Client/          connecting agent
```

## Quasar.Common

Both executables reference this project. It has no dependency on Server or Client.

| Folder | Contents |
|---|---|
| `Messages/` | Every protocol type. Each class implements the empty `IMessage` interface. Requests are named `Get*` or `Do*`. Replies are named `*Response`. |
| `Messages/ReverseProxy/` | Connect, data, and disconnect messages for the proxy path. |
| `Networking/` | `PayloadReader`, `PayloadWriter`, and `ISender`. This is the length-prefix framing around protobuf. |
| `Models/` | Data carried inside messages: process, drive, file entry, startup item, recovered account, registry value, TCP connection. |
| `Enums/` | Small shared enums: connection state, account type, mouse action, shutdown action, file type, user status. |
| `Cryptography/` | AES-256 helper, SHA-256 helper, and a constant-time comparison helper. |
| `Video/` | Frame size and the desktop stream codec used by remote desktop. |
| `DNS/` | Host list parsing. A client can be given more than one `host:port`. |
| `IO/` | Splitting a file into chunks for transfer. |
| `Helpers/`, `Extensions/`, `Utilities/` | Small shared functions used by both sides. |

`Messages/MessageHandler.cs` is the static dispatcher. `Messages/IMessageProcessor.cs` is the handler interface. `Messages/MessageProcessorBase.cs` is the server-side base class that marshals progress back to the UI thread. `Messages/TypeRegistry.cs` registers concrete message types as protobuf subtypes of `IMessage`.

## Quasar.Server

WinForms console. Entry point is `Program.cs`, which runs `Forms/FrmMain`.

| Folder | Contents |
|---|---|
| `Forms/` | One form per feature, plus main, settings, certificate, builder, and about. |
| `Messages/` | Server-side handlers. Each one is paired with a form and with a client handler of the same name. |
| `Networking/` | `Server` (listen, accept, TLS), `QuasarServer` (identification and dispatch), `Client` (one connected machine), `BufferPool`, `UPnPService`. |
| `Build/` | Client builder and the renamer used when producing a release client. |
| `Models/` | Server-only state: settings, build options, builder profile, an in-progress file transfer. |
| `Controls/` | Custom WinForms controls, including the hex editor used by the registry editor. |
| `ReverseProxy/` | Local listener that pairs an incoming socket with a remote connection on the client. |
| `Registry/` | Helpers for showing registry values in the editor. |
| `Helper/`, `Utilities/`, `Extensions/` | UI and system helpers used only by the console. |

`FrmMain` owns the listen socket and the client list. It does not own feature protocol. Opening a feature constructs a form, and that form registers its handler.

## Quasar.Client

WinForms agent. The form never stays visible. Entry point is `Program.cs`, which runs `QuasarApplication`.

| Folder | Contents |
|---|---|
| `QuasarApplication.cs` | Boot: settings, single instance, handler registration, connect loop. |
| `Config/Settings.cs` | Connection and install settings. Debug builds read the constants in this file. Release builds decrypt values written by the builder. |
| `Networking/` | `Client` (socket, TLS, framing) and `QuasarClient` (reconnect loop and identification). |
| `Messages/` | Client-side handlers. These implement `IMessageProcessor` directly. They run the work and send a response. |
| `Setup/` | Install, startup entry, update, and uninstall. Used during boot when those settings are on. |
| `Logging/` | Background keylogger service. |
| `ReverseProxy/` | Outbound half of a proxied connection. |
| `Registry/` | Registry read and edit used by `RegistryHandler`. |
| `Recovery/` | Browser and FTP account readers used by `PasswordRecoveryHandler`. |
| `User/` | Account info and activity detection. |
| `IpGeoLocation/` | Country lookup included in the identification message. |
| `IO/` | Shell helper, hardware id, batch file used to restart after an unhandled exception. |

## How the projects point at each other

```text
Quasar.Server ──► Quasar.Common
Quasar.Client ──► Quasar.Common
```

Server and Client never reference each other. A new message has to live in Common or the other side cannot deserialize it.
