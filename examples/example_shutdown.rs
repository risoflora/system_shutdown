//! How to shut down the machine.

use system_shutdown::shutdown;

fn main() {
    match shutdown() {
        Ok(_) => println!("Shutting down, bye!"),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    }
}
