//! How to log out the user.

use system_shutdown::logout;

fn main() {
    match logout() {
        Ok(_) => println!("Logging out ..."),
        Err(error) => eprintln!("Failed to log out: {}", error),
    }
}
