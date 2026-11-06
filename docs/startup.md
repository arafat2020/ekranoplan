# Startup

Both programs are WinForms apps. Both force TLS 1.2 in `Program.Main` before any window is created.

## Server

`Quasar.Server/Program.cs` enables visual styles and runs `FrmMain`.

`FrmMain` does four things at launch:

1. Loads the server certificate.
2. Constructs `QuasarServer` with that certificate.
3. Subscribes to server state, client connected, and client disconnected.
4. Starts listening if auto-listen is turned on in settings.

`StartConnectionListener` calls `ListenServer.Listen` with the configured port, an IPv6 flag, and an optional UPnP flag. The status bar shows whether the socket is listening and which port it bound.

`FrmMain` registers one long-lived handler, the client status handler, used to keep the list in sync. Feature handlers are not registered here. They appear when a feature window opens and disappear when that window closes.

Closing the main form disconnects the listener.

## Client

`Quasar.Client/Program.cs` installs two exception hooks, one for the UI thread and one for the AppDomain. Either hook writes a small restart batch file and exits the process. The intent is that a crash starts the executable again.

It then runs `QuasarApplication`.

`QuasarApplication` is a `Form`, but `OnLoad` sets `Visible` and `ShowInTaskbar` to false before calling `Run()`. The process exists so WinForms can pump messages. There is no main window.

`Run()` proceeds in order:

1. `Settings.Initialize()`. Debug builds set paths and return true. Release builds decrypt the fields the builder embedded. If initialization fails, the process exits with code 1.
2. A named mutex from `Settings.MUTEX`. If another instance already holds it, the process exits with code 2.
3. The zone identifier on the executable is removed.
4. If install is enabled and the running path is not the install path, the mutex is released, `ClientInstaller.Install()` runs, and the process exits with code 3. The installed copy is what connects.
5. Otherwise `ClientInstaller.ApplySettings()` runs, then the connect path starts.

The connect path:

1. Unless unattended mode is on, a notification icon is created. Its tooltip later switches between "No connection" and "Connection established".
2. If the logger setting is on, `KeyloggerService` starts in the background.
3. `HostsManager` is built from `Settings.HOSTS`.
4. `QuasarClient` is created with that host list and the pinned server certificate.
5. `InitializeMessageProcessors` constructs every feature handler and registers each one with `MessageHandler`.
6. `ActivityDetection` starts.
7. A background thread calls `ConnectLoop()`. When that loop returns, the process exits with code 0.

`ConnectLoop` is the whole lifetime of the agent. While the token is not cancelled it picks the next host, calls `Connect`, and sleeps while the socket stays up. When the socket drops it waits `Settings.RECONNECTDELAY` plus a random 250–750 ms, then tries again.

`Dispose` unregisters handlers, stops the logger and activity detection, releases the mutex, and disconnects.

## Debug and release configuration

`Quasar.Client/Config/Settings.cs` is compiled twice in spirit, behind `#if DEBUG`.

| | Debug | Release |
|---|---|---|
| Values | Constants in `Settings.cs`. Default host is `localhost:4782`. | Empty strings in source. The builder writes ciphertext into those fields. |
| `Initialize()` | Sets paths and returns true. | Decrypts tag, version, hosts, paths, and related fields. Returns false if version is empty. |
| Server identity check | `IdentifyClient` returns true without checking the signature. | The server verifies the signature with its certificate public key. |

Change `Settings.cs` before compiling a Debug client. A Release client is produced from the builder inside the server (`FrmBuilder`, `Build/ClientBuilder.cs`), which is what fills those fields.

## Single instance

`Utilities/SingleInstanceMutex.cs` wraps a system-wide mutex. The name comes from settings, so two builds with different mutex names will not see each other. Install releases the mutex first, otherwise the new copy could not start.
