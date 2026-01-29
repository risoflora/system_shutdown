use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::process::Command;

use super::not_implemented;
use super::ShutdownResult;

use zbus::export::serde::Serialize;
use zbus::zvariant::DynamicType;

fn name_has_owner(name: &str) -> bool {
    if let Ok(conn) = zbus::blocking::Connection::session() {
        let reply = conn.call_method(
            Some("org.freedesktop.DBus"),
            "/",
            Some("org.freedesktop.DBus"),
            "NameHasOwner",
            &(name),
        );
        return reply.and_then(|r| r.body().deserialize()).unwrap_or(false);
    }
    false
}

fn dbus_send<B: Serialize + DynamicType>(
    destination: &str,
    path: &str,
    interface: &str,
    method: &str,
    body: &B,
) -> bool {
    if name_has_owner(destination) {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            let reply = conn.call_method(Some(destination), path, Some(interface), method, body);
            if let Err(e) = &reply {
                if let zbus::Error::MethodError(name, _, _) = e {
                    let error_str = name.as_str();
                    if error_str.contains("org.gtk.GDBus.UnmappedGError.Quark")
                        && error_str.contains(".Code19")
                    {
                        // Code 19 is G_IO_ERROR_CANCELLED
                        return true;
                    }
                }
            }
            return reply.is_ok();
        }
    }
    false
}

fn run_command(command: &str, args: &[&str]) -> ShutdownResult {
    let mut cmd = Command::new(command);
    cmd.args(args);
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                return Ok(());
            }
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ))
        }
        Err(error) => Err(error),
    }
}

fn get_session_id() -> String {
    let mut session = std::env::var("XDG_SESSION_ID").unwrap_or_default();
    if session.is_empty() {
        session = std::fs::read_to_string("/proc/self/sessionid")
            .unwrap_or_default()
            .trim()
            .to_string();
    }
    session
}

/// Linux specific function to shut down the machine using D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.gnome.SessionManager.Shutdown()
/// - org.kde.KSMServerInterface.logout(-1, 2, 2)
/// - org.xfce.SessionManager.Shutdown(true)
/// - org.freedesktop.login1.Manager.PowerOff(true)
/// - org.freedesktop.PowerManagement.Shutdown()
/// - org.freedesktop.SessionManagement.Shutdown()
/// - org.freedesktop.ConsoleKit.Manager.Stop()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Shutdown()
/// - org.freedesktop.systemd1.Manager.PowerOff()
/// If nothing works up to this point, as a last resort this function calls `shutdown -h now`
pub fn shutdown() -> ShutdownResult {
    if dbus_send(
        "org.gnome.SessionManager",
        "/org/gnome/SessionManager",
        "org.gnome.SessionManager",
        "Shutdown",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.kde.ksmserver",
        "/KSMServer",
        "org.kde.KSMServerInterface",
        "logout",
        &(-1, 2, 2),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.xfce.SessionManager",
        "/org/xfce/SessionManager",
        "org.xfce.SessionManager",
        "Shutdown",
        &(true),
    ) {
        return Ok(());
    } // allow_save - true
    if dbus_send(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
        "PowerOff",
        &(true),
    ) {
        return Ok(());
    } // interactive - true
    if dbus_send(
        "org.freedesktop.PowerManagement",
        "/org/freedesktop/PowerManagement",
        "org.freedesktop.PowerManagement",
        "Shutdown",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.SessionManagement",
        "/org/freedesktop/SessionManagement",
        "org.freedesktop.SessionManagement",
        "Shutdown",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.ConsoleKit",
        "/org/freedesktop/ConsoleKit/Manager",
        "org.freedesktop.ConsoleKit.Manager",
        "Stop",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.Hal",
        "/org/freedesktop/Hal/devices/computer",
        "org.freedesktop.Hal.Device.SystemPowerManagement",
        "Shutdown",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        "org.freedesktop.systemd1.Manager",
        "PowerOff",
        &(),
    ) {
        return Ok(());
    }

    // As a last resort
    run_command("shutdown", &["-h", "now"])
}

#[doc(hidden)]
pub fn force_shutdown() -> ShutdownResult {
    not_implemented!()
}

/// Linux specific function to reboot the machine using D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.gnome.SessionManager.Reboot()
/// - org.kde.KSMServerInterface.logout(-1, 1, 2)
/// - org.xfce.SessionManager.Restart(true)
/// - org.freedesktop.login1.Manager.Reboot(true)
/// - org.freedesktop.PowerManagement.Reboot()
/// - org.freedesktop.SessionManagement.Reboot()
/// - org.freedesktop.ConsoleKit.Manager.Restart()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Reboot()
/// - org.freedesktop.systemd1.Manager.Reboot()
/// If nothing works up to this point, as a last resort this function calls `shutdown -r now`
pub fn reboot() -> ShutdownResult {
    if dbus_send(
        "org.gnome.SessionManager",
        "/org/gnome/SessionManager",
        "org.gnome.SessionManager",
        "Reboot",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.kde.ksmserver",
        "/KSMServer",
        "org.kde.KSMServerInterface",
        "logout",
        &(-1, 1, 2),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.xfce.SessionManager",
        "/org/xfce/SessionManager",
        "org.xfce.SessionManager",
        "Restart",
        &(true),
    ) {
        return Ok(());
    } // allow_save - true
    if dbus_send(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
        "Reboot",
        &(true),
    ) {
        return Ok(());
    } // interactive - true
    if dbus_send(
        "org.freedesktop.PowerManagement",
        "/org/freedesktop/PowerManagement",
        "org.freedesktop.PowerManagement",
        "Reboot",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.SessionManagement",
        "/org/freedesktop/SessionManagement",
        "org.freedesktop.SessionManagement",
        "Reboot",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.ConsoleKit",
        "/org/freedesktop/ConsoleKit/Manager",
        "org.freedesktop.ConsoleKit.Manager",
        "Restart",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.Hal",
        "/org/freedesktop/Hal/devices/computer",
        "org.freedesktop.Hal.Device.SystemPowerManagement",
        "Reboot",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        "org.freedesktop.systemd1.Manager",
        "Reboot",
        &(),
    ) {
        return Ok(());
    }

    // As a last resort
    run_command("shutdown", &["-r", "now"])
}

/// Linux specific function to force reboot the machine using the magic SysRq key.
/// Reference: https://www.kernel.org/doc/html/latest/admin-guide/sysrq.html
pub fn force_reboot() -> ShutdownResult {
    let mut file = File::create("/proc/sys/kernel/sysrq")?;
    file.write_all(b"128")?;
    file = File::create("/proc/sysrq-trigger")?;
    file.write_all(b"b")?;
    Ok(())
}

/// Linux specific function to log out the user using D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.gnome.SessionManager.Logout(1)
/// - org.kde.KSMServerInterface.logout(-1, 0, 2)
/// - org.kde.KSMServerInterface.closeSession()
/// - org.xfce.SessionManager.Logout(true, true)
/// - org.freedesktop.login1.Manager.TerminateSession(session_id)
/// If nothing works up to this point, as a last resort this function calls `loginctl kill-session $XDG_SESSION_ID`
pub fn logout() -> ShutdownResult {
    if dbus_send(
        "org.gnome.SessionManager",
        "/org/gnome/SessionManager",
        "org.gnome.SessionManager",
        "Logout",
        &(1),
    ) {
        return Ok(());
    } // 1 - no confirmation dialog, 2 - force logout
    if dbus_send(
        "org.kde.ksmserver",
        "/KSMServer",
        "org.kde.KSMServerInterface",
        "logout",
        &(-1, 0, 2),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.kde.ksmserver",
        "/KSMServer",
        "org.kde.KSMServerInterface",
        "closeSession",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.xfce.SessionManager",
        "/org/xfce/SessionManager",
        "org.xfce.SessionManager",
        "Logout",
        &(true, true),
    ) {
        return Ok(());
    } // show_dialog - true, allow_save - true

    let session_id = get_session_id();
    if session_id.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            "could not determine session ID for logout",
        ));
    }

    if dbus_send(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
        "TerminateSession",
        &session_id,
    ) {
        return Ok(());
    }

    // As a last resort
    run_command("loginctl", &["kill-session", &session_id])
}

#[doc(hidden)]
pub fn force_logout() -> ShutdownResult {
    not_implemented!()
}

/// Linux specific function to put the machine to sleep using D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.xfce.SessionManager.Suspend()
/// - org.freedesktop.login1.Manager.Suspend(true)
/// - org.freedesktop.UPower.Suspend()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Suspend()
/// If nothing works up to this point, as a last resort this function calls `systemctl suspend`
pub fn sleep() -> ShutdownResult {
    if dbus_send(
        "org.xfce.SessionManager",
        "/org/xfce/SessionManager",
        "org.xfce.SessionManager",
        "Suspend",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
        "Suspend",
        &(true),
    ) {
        return Ok(());
    } // interactive - true
    if dbus_send(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        "org.freedesktop.UPower",
        "Suspend",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.Hal",
        "/org/freedesktop/Hal/devices/computer",
        "org.freedesktop.Hal.Device.SystemPowerManagement",
        "Suspend",
        &(),
    ) {
        return Ok(());
    }

    // As a last resort
    run_command("systemctl", &["suspend"])
}

/// Linux specific function to hibernate the machine using D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.xfce.SessionManager.Hibernate()
/// - org.freedesktop.login1.Manager.Hibernate(true)
/// - org.freedesktop.UPower.Hibernate()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Hibernate()
/// If nothing works up to this point, as a last resort this function calls `systemctl hibernate`
pub fn hibernate() -> ShutdownResult {
    if dbus_send(
        "org.xfce.SessionManager",
        "/org/xfce/SessionManager",
        "org.xfce.SessionManager",
        "Hibernate",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
        "Hibernate",
        &(true),
    ) {
        return Ok(());
    } // interactive - true
    if dbus_send(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        "org.freedesktop.UPower",
        "Hibernate",
        &(),
    ) {
        return Ok(());
    }
    if dbus_send(
        "org.freedesktop.Hal",
        "/org/freedesktop/Hal/devices/computer",
        "org.freedesktop.Hal.Device.SystemPowerManagement",
        "Hibernate",
        &(),
    ) {
        return Ok(());
    }

    // As a last resort
    run_command("systemctl", &["hibernate"])
}
