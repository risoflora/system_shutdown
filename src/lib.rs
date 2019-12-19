//! `system_shutdown` provides a cross platform way to shut down, reboot or log out operations.
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
//!     match shutdown() {
//!         Ok(_) => println!("Shutting down, bye!"),
//!         Err(error) => eprintln!("Failed to shut down: {}", error),
//!     }
//! }
//! ```
//!
//! In most of the systems it does not requires the user to be root/admin.

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "linux.rs"]
mod os;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod os;

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
