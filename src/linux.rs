use std::io::{Error, ErrorKind};
use std::process::Command;

use not_implemented;
use ShutdownResult;

fn perform_shutdown(rebooting: bool) -> ShutdownResult {
    let mut cmd = Command::new("shutdown");
    if rebooting {
        cmd.arg("-r");
    } else {
        cmd.arg("-h");
    }
    cmd.arg("now");
    match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.len() == 0 {
                return Ok(());
            }
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    }
}

/// Linux specific function to shut down the machine using the `shutdown` command.
pub fn shutdown() -> ShutdownResult {
    perform_shutdown(false)
}

#[doc(hidden)]
pub fn force_shutdown() -> ShutdownResult {
    not_implemented!()
}

/// Linux specific function to reboot the machine using the `shutdown` command.
pub fn reboot() -> ShutdownResult {
    perform_shutdown(true)
}

#[doc(hidden)]
pub fn force_reboot() -> ShutdownResult {
    not_implemented!()
}

#[doc(hidden)]
pub fn logout() -> ShutdownResult {
    not_implemented!()
}

#[doc(hidden)]
pub fn force_logout() -> ShutdownResult {
    not_implemented!()
}
