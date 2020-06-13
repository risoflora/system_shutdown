use std::fs::File;
use std::io::{Error, ErrorKind, Write};
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
            if output.status.success() && output.stderr.is_empty() {
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

/// Linux specific function to force reboot the machine using the magic SysRq key.
pub fn force_reboot() -> ShutdownResult {
    // Reference: https://www.kernel.org/doc/html/latest/admin-guide/sysrq.html
    let mut file = File::create("/proc/sys/kernel/sysrq")?;
    file.write_all(b"128")?;
    file = File::create("/proc/sysrq-trigger")?;
    file.write_all(b"b")?;
    Ok(())
}

#[doc(hidden)]
pub fn logout() -> ShutdownResult {
    not_implemented!()
}

#[doc(hidden)]
pub fn force_logout() -> ShutdownResult {
    not_implemented!()
}
