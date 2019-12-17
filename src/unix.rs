use std::process::Command;

/// Unix specific function to shut down/restart the machine using the `shutdown` command.
/// When `forced` is `true`, it uses the `-P now` to process the user request instantly.
pub fn sys_shutdown(rebooting: bool, forced: bool) -> bool {
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
