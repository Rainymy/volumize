## Priority

- [ ] Add system tray
    - [ ] system tray icon and menu
    - [ ] minimize to tray functionality

## Todo

- [ ] Implement server discovery/scanning
    - I do not know how to. Will look into it.

## Done

- [x] Rust code to control per-application volume
- [x] Exposed Rust functions to frontend (JS)
- [x] Basic desktop UI (displays volume levels)
- [x] Fix desktop UI styling and layout
    - [x] fix the layout and improve styling
- [x] Bug: UI volume slider not updating between device selection.
    - Not optimal solution but works for now.
- [x] Mobile build compiles
- [x] Build WebSocket client-server communication
    - [x] handle WebSocket disconnections/reconnections
    - [x] send/receive WebSocket message protocol
- [x] Integrate WebSocket into frontend UI.
    - [x] Implement translation layer for frontend.
        - Can't make it work with serde.
    - [x] Integrate WebSocket into current implementation.
        - Try not to change how it is used in the UI.
- [x] Bug in UI sometimes the connection is stale.
    - Sometimes the device applications not changing.
    - Wireless connection has high latency.
- [x] Tauri uses Tokio under the hood.
    - Check if need to use directly or indirectly via Tauri.
    - Prefer to use the re-exported functionality for compatibility reasons.
        - Tauri & Tokio runtime may use different version.
