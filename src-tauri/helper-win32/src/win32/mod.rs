mod apply_rule;
mod firewall;
mod windows;

#[cfg(windows)]
pub use apply_rule::*;
#[cfg(windows)]
pub use windows::*;

#[allow(unused_imports)]
use super::CustomExitCode;
use super::formatter::*;
