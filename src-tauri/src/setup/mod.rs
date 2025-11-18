mod desktop;
mod system_tray;

mod menu_event;
mod setup;

pub use desktop::create_tauri_app;
pub use menu_event::menu_event;
#[allow(unused_imports)]
pub use setup::*;
