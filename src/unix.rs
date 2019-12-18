use std::process::Command;

fn perform_shutdown(rebooting: bool, forced: bool) -> Option<i32> {
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
        Ok(output) => {
            if output.status.success() {
                None
            } else {
                Some(output.status.code().unwrap())
            }
        }
        Err(error) => error.raw_os_error(),
    }
}

/// Unix specific function to shut down the machine using the `shutdown` command.
/// When `forced` is `true`, it uses the `-P now` argument to shut down instantly.
pub fn shutdown(forced: bool) -> Option<i32> {
    perform_shutdown(false, forced)
}

/// Unix specific function to reboot the machine using the `shutdown` command.
/// When `forced` is `true`, it uses the `-P now` argument to reboot instantly.
pub fn reboot(forced: bool) -> Option<i32> {
    perform_shutdown(true, forced)
}

/// Unix specific function to log out the system using the `pkill` command.
/// When `forced` is `true`, it uses the `-KILL` argument to log out instantly.
pub fn logout(forced: bool) -> Option<i32> {
    match Command::new("whoami").output() {
        Ok(output) => {
            let username = String::from_utf8(output.stdout).unwrap();
            let mut cmd = Command::new("pkill");
            cmd.arg("-u");
            cmd.arg(username.trim());
            if forced {
                cmd.arg("-KILL");
            }
            match cmd.output() {
                Ok(output) => {
                    if output.status.success() {
                        None
                    } else {
                        Some(output.status.code().unwrap())
                    }
                }
                Err(error) => error.raw_os_error(),
            }
        }
        Err(error) => error.raw_os_error(),
    }
}
