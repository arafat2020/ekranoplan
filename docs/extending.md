# Extending

A new feature does not need a new socket, a new opcode, or a change to `MessageHandler`. It needs four pieces that agree on the same message types.

## 1. Messages in Common

Add classes under `Quasar.Common/Messages/` that implement `IMessage`.

Use the existing names. A read is `GetSomething` and `GetSomethingResponse`. An action is `DoSomething` and, if the console needs a result, `DoSomethingResponse`. Put any structured payload in `Quasar.Common/Models/` rather than inventing a second copy on each side.

`TypeRegistry.GetPacketTypes` discovers concrete `IMessage` types and `AddTypeToSerializer` assigns each new type the next subtype index. A type that is added or removed changes those indexes. Both executables have to be built from the same Common, or they will disagree about which bytes mean which class.

Protobuf-net serializes public fields and the attributes already used on neighboring message classes. Match that style so the reader and writer stay symmetric.

## 2. Client handler

Add `Quasar.Client/Messages/SomethingHandler.cs` implementing `IMessageProcessor`.

- `CanExecute` returns true for the request types only. The client does not handle its own responses.
- `CanExecuteFrom` returns true.
- `Execute` switches on the message type and calls a private method per type. That method does the work and `sender.Send`s the response.

Register the instance in `QuasarApplication.InitializeMessageProcessors`, next to the existing `new ...Handler()` lines. A handler that is not in that list is never called. If the handler owns a socket, a process, or a timer, implement `IDisposable`. `CleanupMessageProcessors` disposes handlers that implement it.

## 3. Server handler

Add `Quasar.Server/Messages/SomethingHandler.cs` extending `MessageProcessorBase<T>`, where `T` is the progress value the form wants, such as an array of results.

- Construct with `base(true)` so progress posts to the UI synchronization context.
- Store the `Client` passed in.
- `CanExecute` returns true for the response types only.
- `CanExecuteFrom` returns `_client.Equals(sender)`.
- `Execute` turns the response into `OnReport(...)` or a dedicated event posted through `SynchronizationContext`.

Public methods on this handler are how the form sends requests: they call `_client.Send(new GetSomething())`. The form should not build frames itself.

## 4. Form and menu entry

Add `Quasar.Server/Forms/FrmSomething.cs` following `FrmTaskManager`.

- Static `CreateNewOrGetExisting(Client)` so one connection has one window.
- Constructor creates the handler, subscribes to `ProgressChanged` and to `ClientState`, then `MessageHandler.Register`.
- Close path unregisters, unsubscribes, and removes the dictionary entry.
- On disconnect, stop sending.

Add the menu item in `FrmMain` so the selected list row can open `FrmSomething.CreateNewOrGetExisting(client)`.

## What you should not need to touch

| Leave alone | Why |
|---|---|
| `PayloadReader`, `PayloadWriter` | Framing is length plus protobuf for every message. |
| `MessageHandler.Process` | Filtering by `CanExecute` and `CanExecuteFrom` already routes the new type. |
| `Server.Listen` and `Client.Connect` | The session is already up by the time a feature runs. |
| The identification messages | A feature message sent before identification is dropped on both sides. |

## Checklist

- [ ] Request and response classes implement `IMessage` and live in Common.
- [ ] Shared payload types live in `Quasar.Common/Models`.
- [ ] Client handler accepts requests, sends responses, and is constructed in `InitializeMessageProcessors`.
- [ ] Server handler accepts responses only from its own `Client`, and reports on the UI context.
- [ ] The form registers on open and unregisters on close.
- [ ] `FrmMain` can open the form for the selected client.
- [ ] Server and client are both rebuilt so the protobuf subtype indexes match.
