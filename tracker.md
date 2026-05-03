## ================ Todo ================

- [ ] Client detect if there is multiple server instances running.
    - Dropdown menu to select the server instance.
- [ ] Implement the settings + including the settings page.
    - Add different settings functionality.
- [ ] Implement a authentication system.
    - Easies should be generate Random String.
    - Scan QR code.
- [ ] System tray menu update on timer.
    - tray menu is not reflected when choosing timer option.
    - [ ] Remove the timer options. Easier to remove than implement
- [ ] Refactor rust codebase.
    - Unify async and sync usages.
        - Async and sync code are a mess. Especially the thread management.

## ================ Done ================

- [x] Allow only one instance of the application to run.
- [x] Setup firewall rule for windows (allow "public" to "private" network)
- [x] Exit to tray option is missing from tray menu option.
- [x] There is "TrayIcon" setting in "tauri.conf.json".
    - Use it and refactor the tray handling code.
- [x] Autostart.
    - [x] Implement autostart functionality.
    - [x] Add autostart settings to the tray menu.
