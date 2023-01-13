//! `system_shutdown` provides a cross platform way to shut down, reboot or log out operations.
//!
//! Supported platforms: Linux, Windows and macOS.
//!
//! # Example
//!
//! The example below shows how to shut down the machine:
//!
//! ```rust
//! use system_shutdown::shutdown;
//!
//! fn main() {
//!     match shutdown() {
//!         Ok(_) => println!("Shutting down, bye!"),
//!         Err(error) => eprintln!("Failed to shut down: {}", error),
//!     }
//! }
//! ```
//!
//! In most of the systems it does not requires the user to be root/admin.

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod os;

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod os;
#[cfg(target_os = "macos")]
pub use os::request_permission_dialog;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod os;
#[cfg(target_os = "windows")]
pub use os::{reboot_with_message, shutdown_with_message};

use std::io;

#[doc(hidden)]
#[macro_export]
macro_rules! not_implemented {
    () => {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "feature not implemented yet",
        ))
    };
}

/// A specialized `Result` type for shut down, reboot and log out operations.
pub type ShutdownResult = io::Result<()>;

/// Calls the OS-specific function to shut down the machine.
pub fn shutdown() -> ShutdownResult {
    os::shutdown()
}

/// Calls the OS-specific function to force to shut down the machine.
pub fn force_shutdown() -> ShutdownResult {
    os::force_shutdown()
}

/// Calls the OS-specific function to reboot the machine.
pub fn reboot() -> ShutdownResult {
    os::reboot()
}

/// Calls the OS-specific function to force to reboot the machine.
pub fn force_reboot() -> ShutdownResult {
    os::force_reboot()
}

/// Calls the OS-specific function to log out the user.
pub fn logout() -> ShutdownResult {
    os::logout()
}

/// Calls the OS-specific function to force to log out the user.
pub fn force_logout() -> ShutdownResult {
    os::force_logout()
}

/// Calls the OS-specific function to put the machine to sleep.
pub fn sleep() -> ShutdownResult {
    os::sleep()
}

/// Calls the OS-specific function to hibernate the machine.
pub fn hibernate() -> ShutdownResult {
    os::hibernate()
}
