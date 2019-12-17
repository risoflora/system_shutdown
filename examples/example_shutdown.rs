//! How to shut down the machine.

extern crate system_shutdown;

use system_shutdown::shutdown;

fn main() {
    match shutdown(true) {
        None => println!("Shutting down, bye!"),
        Some(code) => println!("Failed to shut down. (Os code: {})", code),
    }
}
