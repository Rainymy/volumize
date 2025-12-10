## ============== Priority ==============

- [ ] Implement the settings + including the settings page.
    - Add different settings functionality.

## ================ Todo ================

- [ ] Implement a authentication system.
    - Easies should be generate Random String.
    - Scan QR code.
- [ ] System tray menu update on timer.
    - tray menu is not reflected when choosing timer option.
- [ ] Improved UI design.
    - The new design is better than the old one.
- [ ] Refactor rust codebase.
    - Unify async and sync usages.
        - Async and sync code are a mess. Especially the thread management.

## ================ Done ================

- [x] Autostart.
    - [x] Implement autostart functionality.
    - [x] Add autostart settings to the tray menu.
- [x] Detect changes in volume.
    - [x] Implement volume detection functionality.
        - [x] Detect system audio changes.
        - [x] Detect application audio changes.
        - [x] Setup functionality to propagate changes to UI.
    - [x] Update Changes at UI.
        - [x] Send updates to all clients.
        - [x] Sync backend type with UI.
        - [x] Functionality to handle UI updates.
- [x] Implement a heart beat functionality.
    - Client may disconnect (device sleep).

### ============= v2.0 Demo =============

- [x] Update README.md
    - Add instructions for installation and usage.
- [x] Send application icon via API.
    - send icon as PNG.
- [x] Redesign the Rust backend architect.
    - Split up the 1 main API into multple calls.
    - [x] Integrate the UI with new backend implementation.
- [x] Redesign User interface.
    - Make it nicer on small screens like phone.
- [x] Improve application icon.
    - Add some transparency.
    - Make it better when it is small.
- [x] Add functionality to system tray.
    - [x] Create menu + submenu
        - Button for showing hidden window.
        - Controlling Register service.
            - Turing off, on, and duration.
    - [x] Integrate tray button into main loop.
        - [x] Menu selection for register service reflects.
            - Selection options: ["always on", "off", "on timer"]
            - Time options: [2m, 5m, 15m]
        - [x] Reflect settings changes to tray menus.
    - [x] Save settings
        - I need to save the settings.
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

### ========== v1.0 Prototype ===========

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
