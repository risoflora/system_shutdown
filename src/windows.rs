use std::mem;
use std::ptr;
use winapi::shared::minwindef::{FALSE, TRUE, UINT};
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::reason::{
    SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_MAJOR_OPERATINGSYSTEM, SHTDN_REASON_MINOR_UPGRADE,
};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winbase::LookupPrivilegeValueA;
use winapi::um::winnt::{
    HANDLE, LPCSTR, SE_PRIVILEGE_ENABLED, SE_SHUTDOWN_NAME, TOKEN_ADJUST_PRIVILEGES,
    TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use winapi::um::winuser::{ExitWindowsEx, EWX_FORCEIFHUNG, EWX_REBOOT, EWX_SHUTDOWN};

fn exit_windows(mut flags: UINT, forced: bool) -> Option<i32> {
    unsafe {
        let mut token: HANDLE = ptr::null_mut();
        let mut tkp: TOKEN_PRIVILEGES = mem::zeroed();
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        ) == TRUE
        {
            LookupPrivilegeValueA(
                ptr::null(),
                SE_SHUTDOWN_NAME.as_ptr() as LPCSTR,
                &mut tkp.Privileges[0].Luid,
            );
            tkp.PrivilegeCount = 1;
            tkp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
            AdjustTokenPrivileges(token, FALSE, &mut tkp, 0, ptr::null_mut(), ptr::null_mut());
            if forced {
                flags |= EWX_FORCEIFHUNG;
            }
            let err = GetLastError();
            if err != ERROR_SUCCESS {
                return Some(err as i32);
            }
            if ExitWindowsEx(
                flags,
                SHTDN_REASON_MAJOR_OPERATINGSYSTEM
                    | SHTDN_REASON_MINOR_UPGRADE
                    | SHTDN_REASON_FLAG_PLANNED,
            ) == TRUE
            {
                return None;
            }
        }
        Some(GetLastError() as i32)
    }
}

/// Windows specific function to shut down the machine using the `ExitWindowsEx()` from `winuser` API.
/// When `forced` is `true`, it uses the `EWX_FORCEIFHUNG` flag to shut down instantly without confirmations.
pub fn shutdown(forced: bool) -> Option<i32> {
    exit_windows(EWX_SHUTDOWN, forced)
}

/// Windows specific function to reboot the machine using the `ExitWindowsEx()` from `winuser` API.
/// When `forced` is `true`, it uses the `EWX_FORCEIFHUNG` flag to reboot instantly without confirmations.
pub fn reboot(forced: bool) -> Option<i32> {
    exit_windows(EWX_REBOOT, forced)
}
