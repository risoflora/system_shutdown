use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::process::Command;

use super::not_implemented;
use super::ShutdownResult;

fn invoke_script(script: &str) -> ShutdownResult {
    let mut cmd = Command::new("osascript");
    cmd.args(&["-e", script]);
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

/// MacOS requires to explicitly allow the application to call "System Events". If you want to use this crate in an unattended way (automation etc.),
/// you may want to ask for permission beforehand to allow this app to call "System Events".
/// This function requests the "System Events" to press the Shift key, which should be a fairly safe operation.
pub fn request_permission_dialog() -> ShutdownResult {
    invoke_script("tell application \"System Events\" to keystroke key code 16")
}

/// MacOS specific function to reboot without showing a confirmation dialog using AppleScript and "System Events" call "shut down"
/// First time you use this, MacOS will ask for a permission. If you want to ask for a permission beforehand, use [`request_permission_dialog`]
pub fn shutdown() -> ShutdownResult {
    invoke_script("tell application \"System Events\" to shut down")
}

#[doc(hidden)]
pub fn force_shutdown() -> ShutdownResult {
    not_implemented!()
}

/// MacOS specific function to reboot without showing a confirmation dialog using AppleScript and "System Events" call "restart"
/// First time you use this, MacOS will ask for a permission. If you want to ask for a permission beforehand, use [`request_permission_dialog`]
pub fn reboot() -> ShutdownResult {
    invoke_script("tell application \"System Events\" to restart")
}

/// Unix specific function to force reboot the machine using the magic SysRq key.
pub fn force_reboot() -> ShutdownResult {
    // Reference: https://www.kernel.org/doc/html/latest/admin-guide/sysrq.html
    let mut file = File::create("/proc/sys/kernel/sysrq")?;
    file.write_all(b"128")?;
    file = File::create("/proc/sysrq-trigger")?;
    file.write_all(b"b")?;
    Ok(())
}

/// MacOS specific function to logout with a confirmation dialog using AppleScript and "System Events" call "log out".
/// First time you use this, MacOS will ask for a permission. If you want to ask for a permission beforehand, use [`request_permission_dialog`]
pub fn logout() -> ShutdownResult {
    invoke_script("tell application \"System Events\" to log out")
}

/// MacOS specific function to force logout without showing a confirmation dialog using AppleScript and "loginwindow" call "«event aevtrlgo»"
pub fn force_logout() -> ShutdownResult {
    invoke_script("tell application \"loginwindow\" to «event aevtrlgo»")
}

/// MacOS specific function to put the machine to sleep using AppleScript and "System Events" call "sleep"
/// First time you use this, MacOS will ask for a permission. If you want to ask for a permission beforehand, use [`request_permission_dialog`]
pub fn sleep() -> ShutdownResult {
    invoke_script("tell application \"System Events\" to sleep")
}

#[doc(hidden)]
pub fn hibernate() -> ShutdownResult {
    // It's possible but not generally a good idea https://superuser.com/a/630985
    not_implemented!()
}
