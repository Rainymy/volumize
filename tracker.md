## Priority

- [ ] Add functionalities to system tray.
    - [x] Create menu + submenu
        - Button for showing hidden window.
        - Controlling Register service.
            - Turing off, on, and duration.
    - [ ] Integrate tray button into main loop.
        - Try reflect change to tray menus.
        - Need to look up how often I can update tray menus.
    - [ ] Save settings
        - I need to save the settings.

## Todo

- [ ] Redesign User interface.
    - Make it nicer on small screens like phone.
- [ ] Improve application icon.
    - Add some transparency.
    - Make it better when it is small.

## Done

- [x] Functionality get host IP address.
    - Server discovery depend on having static IP on startup.
- [x] Add system tray
    - [x] system tray icon & menu.
    - [x] minimize to tray functionality.
- [x] Implement server discovery/scanning
    - Implemented with mDNS & UDP as fallback.
- [x] Tauri uses Tokio under the hood.
    - Check if need to use directly or indirectly via Tauri.
    - Prefer to use the re-exported functionality for compatibility reasons.
        - Tauri & Tokio runtime may use different version.
- [x] Bug in UI sometimes the connection is stale.
    - Sometimes the device applications not changing.
    - Wireless connection has high latency.
- [x] Integrate WebSocket into frontend UI.
    - [x] Implement translation layer for frontend.
        - Can't make it work with serde.
    - [x] Integrate WebSocket into current implementation.
        - Try not to change how it is used in the UI.
- [x] Build WebSocket client-server communication
    - [x] handle WebSocket disconnections/reconnections
    - [x] send/receive WebSocket message protocol
- [x] Mobile build compiles
- [x] Bug: UI volume slider not updating between device selection.
    - Not optimal solution but works for now.
- [x] Fix desktop UI styling and layout
    - [x] fix the layout and improve styling
- [x] Basic desktop UI (displays volume levels)
- [x] Exposed Rust functions to frontend (JS)
- [x] Rust code to control per-application volume
