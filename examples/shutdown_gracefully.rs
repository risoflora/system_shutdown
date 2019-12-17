//! How to shut down the machine.

extern crate system_shutdown;

use system_shutdown::shutdown;

fn main() {
    if shutdown(true) {
        println!("Shutting down, bye!");
    } else {
        println!("Failed to shut down.");
    }
}
