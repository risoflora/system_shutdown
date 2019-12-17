//! How to reboot the machine.

extern crate system_shutdown;

use system_shutdown::reboot;

fn main() {
    match reboot(true) {
        None => println!("Rebooting ..."),
        Some(code) => println!("Failed to reboot. (Os code: {})", code),
    }
}
