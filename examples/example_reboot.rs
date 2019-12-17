//! How to reboot the machine.

extern crate system_shutdown;

use system_shutdown::reboot;

fn main() {
    if reboot(true) {
        println!("Rebooting ...");
    } else {
        println!("Failed to reboot.");
    }
}
