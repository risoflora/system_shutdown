//! How to log out the system.

extern crate system_shutdown;

use system_shutdown::logout;

fn main() {
    match logout(true) {
        None => println!("Logging out ..."),
        Some(code) => println!("Failed to log out. (Os code: {})", code),
    }
}
