use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::{FALSE, UINT};
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::reason::{
    SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_MAJOR_OPERATINGSYSTEM, SHTDN_REASON_MINOR_UPGRADE,
};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winbase::LookupPrivilegeValueW;
use winapi::um::winnt::{
    HANDLE, LPWSTR, SE_PRIVILEGE_ENABLED, SE_SHUTDOWN_NAME, TOKEN_ADJUST_PRIVILEGES,
    TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use winapi::um::winuser::{
    ExitWindowsEx, EWX_FORCE, EWX_FORCEIFHUNG, EWX_LOGOFF, EWX_REBOOT, EWX_SHUTDOWN,
};

use ShutdownResult;

#[doc(hidden)]
#[macro_export]
macro_rules! last_os_error {
    () => {
        Err(Error::last_os_error())
    };
}

fn request_privileges() -> ShutdownResult {
    unsafe {
        let mut token: HANDLE = ptr::null_mut();
        let mut tkp: TOKEN_PRIVILEGES = mem::zeroed();
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        ) == FALSE
        {
            return last_os_error!();
        }
        let security_name: Vec<u16> = OsStr::new(SE_SHUTDOWN_NAME)
            .encode_wide()
            .chain(once(0))
            .collect();
        if LookupPrivilegeValueW(
            ptr::null(),
            security_name.as_ptr() as LPWSTR,
            &mut tkp.Privileges[0].Luid,
        ) == FALSE
        {
            return last_os_error!();
        }
        tkp.PrivilegeCount = 1;
        tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
        if AdjustTokenPrivileges(token, FALSE, &mut tkp, 0, ptr::null_mut(), ptr::null_mut())
            == FALSE
        {
            return last_os_error!();
        }
    }
    Ok(())
}

fn exit_windows(flag: UINT) -> ShutdownResult {
    unsafe {
        request_privileges()?;
        if ExitWindowsEx(
            flag | EWX_FORCEIFHUNG,
            SHTDN_REASON_MAJOR_OPERATINGSYSTEM
                | SHTDN_REASON_MINOR_UPGRADE
                | SHTDN_REASON_FLAG_PLANNED,
        ) == FALSE
        {
            return last_os_error!();
        }
    }
    Ok(())
}

/// Windows specific function to shut down the machine using the `ExitWindowsEx()` from `winuser` API.
pub fn shutdown() -> ShutdownResult {
    exit_windows(EWX_SHUTDOWN)
}

/// Windows specific function to shut down the machine instantly without confirmations using the `ExitWindowsEx()` from `winuser` API.
pub fn force_shutdown() -> ShutdownResult {
    exit_windows(EWX_SHUTDOWN | EWX_FORCE)
}

/// Windows specific function to reboot the machine using the `ExitWindowsEx()` from `winuser` API.
pub fn reboot() -> ShutdownResult {
    exit_windows(EWX_REBOOT)
}

/// Windows specific function to reboot the machine instantly without confirmations using the `ExitWindowsEx()` from `winuser` API.
pub fn force_reboot() -> ShutdownResult {
    exit_windows(EWX_REBOOT | EWX_FORCE)
}

/// Windows specific function to log out the user using the `ExitWindowsEx()` from `winuser` API.
pub fn logout() -> ShutdownResult {
    exit_windows(EWX_LOGOFF)
}

/// Windows specific function to log out the user instantly without confirmations using the `ExitWindowsEx()` from `winuser` API.
pub fn force_logout() -> ShutdownResult {
    exit_windows(EWX_LOGOFF | EWX_FORCE)
}
