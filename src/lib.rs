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
//!    match shutdown(true) {
//!        None => println!("Shutting down, bye!"),
//!        Some(code) => println!("Failed to shut down. (Os code: {})", code),
//!    }
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
pub fn shutdown(forced: bool) -> Option<i32> {
    os::shutdown(forced)
}

/// Calls the OS-specific function to reboot the machine.
///
/// # Arguments
///
/// * `[in] forced` - Forces the machine to reboot instantly without confirmations.
pub fn reboot(forced: bool) -> Option<i32> {
    os::reboot(forced)
}

/// Calls the OS-specific function to log out the system.
///
/// # Arguments
///
/// * `[in] forced` - Forces the machine to log out instantly without confirmations.
pub fn logout(forced: bool) -> Option<i32> {
    os::logout(forced)
}
