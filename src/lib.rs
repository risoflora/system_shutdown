//! `system_shutdown` provides a cross platform way to shut down or reboot the machine.
//!
//! Supported platforms: Linux, Windows and MacOS.
//!
//! # Example
//!
//! The example below shows how to shut down the machine:
//!
//! ```rust
//! extern crate system_shutdown;
//!
//! use system_shutdown::shutdown;
//!
//! fn main() {
//!     if shutdown(true) {
//!         println!("Shutting down, bye!");
//!     } else {
//!         println!("Failed to shut down.");
//!     }
//! }
//! ```
//!
//! In most of the systems it does not require the user to be root/admin.

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "unix.rs"]
mod os;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod os;

/// Calls the OS-specific function to shut down the machine.
///
/// # Arguments
///
/// * `[in] forced` - Forces the machine to shut down instantly without confirmations.
pub fn shutdown(forced: bool) -> bool {
    os::sys_shutdown(false, forced)
}

/// Calls the OS-specific function to reboot the machine.
///
/// # Arguments
///
/// * `[in] forced` - Forces the machine to reboot instantly without confirmations.
pub fn reboot(forced: bool) -> bool {
    os::sys_shutdown(true, forced)
}
