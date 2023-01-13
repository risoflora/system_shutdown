use std::io::Error;
use std::mem;
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{BOOLEAN, HANDLE},
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED,
            TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
        },
        System::{
            Power::SetSuspendState,
            Shutdown::{
                ExitWindowsEx, InitiateSystemShutdownW, EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN,
                EXIT_WINDOWS_FLAGS, SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_MAJOR_OPERATINGSYSTEM,
                SHTDN_REASON_MINOR_UPGRADE,
            },
            SystemServices::SE_SHUTDOWN_NAME,
            Threading::{GetCurrentProcess, OpenProcessToken},
        },
        UI::WindowsAndMessaging::{EWX_FORCE, EWX_FORCEIFHUNG},
    },
};

use super::ShutdownResult;

#[doc(hidden)]
#[macro_export]
macro_rules! last_os_error {
    () => {
        Err(Error::last_os_error())
    };
}

fn request_privileges() -> ShutdownResult {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        let mut tkp: TOKEN_PRIVILEGES = mem::zeroed();
        if !OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )
        .as_bool()
        {
            return last_os_error!();
        }
        if !LookupPrivilegeValueW(
            PCWSTR::null(),
            SE_SHUTDOWN_NAME,
            &mut tkp.Privileges[0].Luid,
        )
        .as_bool()
        {
            return last_os_error!();
        }
        tkp.PrivilegeCount = 1;
        tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
        if !AdjustTokenPrivileges(token, false, Some(&tkp), 0, None, None).as_bool() {
            return last_os_error!();
        }
    }
    Ok(())
}

fn exit_windows(flag: u32) -> ShutdownResult {
    unsafe {
        request_privileges()?;
        if !ExitWindowsEx(
            EXIT_WINDOWS_FLAGS(flag | EWX_FORCEIFHUNG),
            SHTDN_REASON_MAJOR_OPERATINGSYSTEM
                | SHTDN_REASON_MINOR_UPGRADE
                | SHTDN_REASON_FLAG_PLANNED,
        )
        .as_bool()
        {
            return last_os_error!();
        }
    }
    Ok(())
}
fn initiate_system_shutdown(
    message: &str,
    timeout: u32,
    force_close_apps: bool,
    restart: bool,
) -> ShutdownResult {
    unsafe {
        request_privileges()?;
        if !InitiateSystemShutdownW(
            PCWSTR::null(),
            PCWSTR(HSTRING::from(message).as_ptr()),
            timeout,
            force_close_apps,
            restart,
        )
        .as_bool()
        {
            return last_os_error!();
        }
    }
    Ok(())
}
fn set_suspend_state(hibernate: bool) -> ShutdownResult {
    unsafe {
        request_privileges()?;
        if SetSuspendState(BOOLEAN::from(hibernate), BOOLEAN(0), BOOLEAN(0)).0 == 0 {
            return last_os_error!();
        }
    }
    Ok(())
}

/// Windows specific function to gracefully request system shutdown, providing a way to show a message,
/// set a timeout and specify if apps should be force-closed
pub fn shutdown_with_message(
    message: &str,
    timeout: u32,
    force_close_apps: bool,
) -> ShutdownResult {
    initiate_system_shutdown(message, timeout, force_close_apps, false)
}

/// Windows specific function to gracefully request system reboot, providing a way to show a message,
/// set a timeout and specify if apps should be force-closed
pub fn reboot_with_message(message: &str, timeout: u32, force_close_apps: bool) -> ShutdownResult {
    initiate_system_shutdown(message, timeout, force_close_apps, true)
}

/// Windows specific function to shut down the machine using the `ExitWindowsEx()` from `winuser` API.
pub fn shutdown() -> ShutdownResult {
    exit_windows(EWX_SHUTDOWN.0)
}

/// Windows specific function to shut down the machine instantly without confirmations using the `ExitWindowsEx()` from `winuser` API.
pub fn force_shutdown() -> ShutdownResult {
    exit_windows(EWX_SHUTDOWN.0 | EWX_FORCE)
}

/// Windows specific function to reboot the machine using the `ExitWindowsEx()` from `winuser` API.
pub fn reboot() -> ShutdownResult {
    exit_windows(EWX_REBOOT.0)
}

/// Windows specific function to reboot the machine instantly without confirmations using the `ExitWindowsEx()` from `winuser` API.
pub fn force_reboot() -> ShutdownResult {
    exit_windows(EWX_REBOOT.0 | EWX_FORCE)
}

/// Windows specific function to log out the user using the `ExitWindowsEx()` from `winuser` API.
pub fn logout() -> ShutdownResult {
    exit_windows(EWX_LOGOFF.0)
}

/// Windows specific function to log out the user instantly without confirmations using the `ExitWindowsEx()` from `winuser` API.
pub fn force_logout() -> ShutdownResult {
    exit_windows(EWX_LOGOFF.0 | EWX_FORCE)
}

/// Windows specific function to put the machine to sleep using `SetSuspendState()` API call.
pub fn sleep() -> ShutdownResult {
    set_suspend_state(false)
}

/// Windows specific function to hibernate the machine using `SetSuspendState()` API call.
pub fn hibernate() -> ShutdownResult {
    set_suspend_state(true)
}
