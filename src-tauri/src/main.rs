// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;

#[cfg(target_os = "windows")]
#[path = "win32/windows.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux/linux.rs"]
mod platform;

fn main() {
    match platform::make_controller() {
        Ok(controller) => {
            let _device = controller.get_all_applications();

            // _device.unwrap().iter().for_each(|val| {
            //     dbg!(val);
            // });

            // dbg!(_device.ok());
        }
        Err(err) => {
            dbg!(err);
        }
    }

    volumize_lib::run();
}
