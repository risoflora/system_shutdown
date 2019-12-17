use std::process::Command;

fn perform_shutdown(rebooting: bool, forced: bool) -> bool {
    let mut cmd = Command::new("shutdown");
    if rebooting {
        cmd.arg("-r");
    } else {
        cmd.arg("-P");
    }
    if forced {
        cmd.arg("now");
    }
    match cmd.output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Unix specific function to shut down the machine using the `shutdown` command.
/// When `forced` is `true`, it uses the `-P now` argument to shut down instantly.
pub fn shutdown(forced: bool) -> bool {
    perform_shutdown(false, forced)
}

/// Unix specific function to reboot the machine using the `shutdown` command.
/// When `forced` is `true`, it uses the `-P now` argument to reboot instantly.
pub fn reboot(forced: bool) -> bool {
    perform_shutdown(true, forced)
}
