#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub use win32::*;

#[cfg(not(target_os = "windows"))]
mod generic;
#[cfg(not(target_os = "windows"))]
pub use generic::*;
