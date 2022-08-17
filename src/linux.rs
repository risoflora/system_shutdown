use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::process::Command;

use super::not_implemented;
use super::ShutdownResult;

use zbus::export::serde::Serialize;
use zbus::zvariant::DynamicType;

fn name_has_owner(name: &str) -> bool {
    if let Ok(conn) = zbus::blocking::Connection::session() {
        let reply = conn.call_method(Some("org.freedesktop.DBus"), "/", Some("org.freedesktop.DBus"), "NameHasOwner", &(name));
        return reply.and_then(|r| r.body()).unwrap_or(false);
    }
    false
}

fn dbus_send<B: Serialize + DynamicType>(destination: &str, path: &str, interface: &str, method: &str, body: &B) -> bool {
    if name_has_owner(destination) {
        if let Ok(conn) = zbus::blocking::Connection::session() {
            let reply = conn.call_method(Some(destination), path, Some(interface), method, body);
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

fn get_session_id() -> String {
    let mut session = std::fs::read_to_string("/proc/self/sessionid").unwrap_or_default().trim().to_string();
    if session.is_empty() {
        session = std::env::var("XDG_SESSION_ID").unwrap_or_default();
    }
    session
}

/// Linux specific function to shut down the machine using the D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.freedesktop.login1.Manager.PowerOff(false)
/// - org.freedesktop.PowerManagement.Shutdown()
/// - org.freedesktop.SessionManagement.Shutdown()
/// - org.freedesktop.ConsoleKit.Manager.Stop()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Shutdown()
/// - org.gnome.SessionManager.Shutdown()
/// If nothing works up to this point, as a last resort this function calls `shutdown -h now`
pub fn shutdown() -> ShutdownResult {
    if dbus_send("org.freedesktop.login1",            "/org/freedesktop/login1",               "org.freedesktop.login1.Manager",                   "PowerOff", &(false)) { return Ok(()); } // interactive - false
    if dbus_send("org.freedesktop.PowerManagement",   "/org/freedesktop/PowerManagement",      "org.freedesktop.PowerManagement",                  "Shutdown",  &()) { return Ok(()); }
    if dbus_send("org.freedesktop.SessionManagement", "/org/freedesktop/SessionManagement",    "org.freedesktop.SessionManagement",                "Shutdown",  &()) { return Ok(()); }
    if dbus_send("org.freedesktop.ConsoleKit",        "/org/freedesktop/ConsoleKit/Manager",   "org.freedesktop.ConsoleKit.Manager",               "Stop",     &()) { return Ok(()); }
    if dbus_send("org.freedesktop.Hal",               "/org/freedesktop/Hal/devices/computer", "org.freedesktop.Hal.Device.SystemPowerManagement", "Shutdown", &()) { return Ok(()); }
    if dbus_send("org.gnome.SessionManager",          "/org/gnome/SessionManager",             "org.gnome.SessionManager",                         "Shutdown", &()) { return Ok(()); }

    // As a last resort
    run_command("shutdown", &["-h", "now"])
}

#[doc(hidden)]
pub fn force_shutdown() -> ShutdownResult {
    not_implemented!()
}

/// Linux specific function to reboot the machine using the D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.freedesktop.login1.Manager.Reboot(false)
/// - org.freedesktop.PowerManagement.Reboot()
/// - org.freedesktop.SessionManagement.Reboot()
/// - org.freedesktop.ConsoleKit.Manager.Restart()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Reboot()
/// - org.gnome.SessionManager.Reboot()
/// If nothing works up to this point, as a last resort this function calls `shutdown -r now`
pub fn reboot() -> ShutdownResult {
    if dbus_send("org.freedesktop.login1",            "/org/freedesktop/login1",               "org.freedesktop.login1.Manager",                   "Reboot",  &(false)) { return Ok(()); } // interactive - false
    if dbus_send("org.freedesktop.PowerManagement",   "/org/freedesktop/PowerManagement",      "org.freedesktop.PowerManagement",                  "Reboot",  &()) { return Ok(()); }
    if dbus_send("org.freedesktop.SessionManagement", "/org/freedesktop/SessionManagement",    "org.freedesktop.SessionManagement",                "Reboot",  &()) { return Ok(()); }
    if dbus_send("org.freedesktop.ConsoleKit",        "/org/freedesktop/ConsoleKit/Manager",   "org.freedesktop.ConsoleKit.Manager",               "Restart", &()) { return Ok(()); }
    if dbus_send("org.freedesktop.Hal",               "/org/freedesktop/Hal/devices/computer", "org.freedesktop.Hal.Device.SystemPowerManagement", "Reboot",  &()) { return Ok(()); }
    if dbus_send("org.gnome.SessionManager",          "/org/gnome/SessionManager",             "org.gnome.SessionManager",                         "Reboot",  &()) { return Ok(()); }

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

/// Linux specific function to log out the user using the D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.freedesktop.login1.Manager.TerminateSession(session_id)
/// - org.gnome.SessionManager.Logout(1)
/// - org.kde.KSMServerInterface.closeSession()
/// If nothing works up to this point, as a last resort this function calls `loginctl kill-session $XDG_SESSION_ID`
pub fn logout() -> ShutdownResult {
    let session_id = get_session_id();
    if !session_id.is_empty() {
        if dbus_send("org.freedesktop.login1",    "/org/freedesktop/login1",    "org.freedesktop.login1.Manager", "TerminateSession", &(session_id)) { return Ok(()); }
    }
    if dbus_send("org.gnome.SessionManager",  "/org/gnome/SessionManager",  "org.gnome.SessionManager",       "Logout",           &(1)) { return Ok(()); } // 1 - no confirmation dialog, 2 - force logout
    if dbus_send("org.kde.ksmserver",         "/KSMServer",                 "org.kde.KSMServerInterface",     "closeSession",     &())  { return Ok(()); }

    // As a last resort
    run_command("loginctl", &["kill-session", &session_id])
}

#[doc(hidden)]
pub fn force_logout() -> ShutdownResult {
    not_implemented!()
}

/// Linux specific function to log out the user using the D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.freedesktop.login1.Manager.Suspend(false)
/// - org.freedesktop.UPower.Suspend()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Suspend()
/// If nothing works up to this point, as a last resort this function calls `systemctl suspend`
pub fn sleep() -> ShutdownResult {
    if dbus_send("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager",                                 "Suspend", &(false)) { return Ok(()); } // interactive - false
    if dbus_send("org.freedesktop.UPower", "/org/freedesktop/UPower", "org.freedesktop.UPower",                                         "Suspend", &()) { return Ok(()); }
    if dbus_send("org.freedesktop.Hal",    "/org/freedesktop/Hal/devices/computer", "org.freedesktop.Hal.Device.SystemPowerManagement", "Suspend", &()) { return Ok(()); }

    // As a last resort
    run_command("systemctl", &["suspend"])
}

/// Linux specific function to log out the user using the D-BUS method call.
/// The following D-BUS calls are attempted:
/// - org.freedesktop.login1.Manager.Hibernate(false)
/// - org.freedesktop.UPower.Hibernate()
/// - org.freedesktop.Hal.Device.SystemPowerManagement.Hibernate()
/// If nothing works up to this point, as a last resort this function calls `systemctl hibernate`
pub fn hibernate() -> ShutdownResult {
    if dbus_send("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager",                                 "Hibernate", &(false)) { return Ok(()); } // interactive - false
    if dbus_send("org.freedesktop.UPower", "/org/freedesktop/UPower", "org.freedesktop.UPower",                                         "Hibernate", &()) { return Ok(()); }
    if dbus_send("org.freedesktop.Hal",    "/org/freedesktop/Hal/devices/computer", "org.freedesktop.Hal.Device.SystemPowerManagement", "Hibernate", &()) { return Ok(()); }

    // As a last resort
    run_command("systemctl", &["hibernate"])
}
