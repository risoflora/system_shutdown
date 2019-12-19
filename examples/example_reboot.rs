//! How to reboot the machine.

extern crate system_shutdown;

use system_shutdown::reboot;

fn main() {
    match reboot() {
        Ok(_) => println!("Rebooting ..."),
        Err(error) => eprintln!("Failed to reboot: {}", error),
    }
}
