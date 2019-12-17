use std::mem;
use std::ptr;
use winapi::shared::minwindef::{FALSE, TRUE};
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

/// Windows specific function to shut down/restart the machine using the `ExitWindowsEx()` from `winuser` API.
/// When `forced` is `true`, it uses the `EWX_FORCEIFHUNG` flag to process the user request instantly without confirmations.
pub fn sys_shutdown(rebooting: bool, forced: bool) -> bool {
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
            let mut flags = if rebooting { EWX_REBOOT } else { EWX_SHUTDOWN };
            if forced {
                flags |= EWX_FORCEIFHUNG;
            }
            return GetLastError() == ERROR_SUCCESS
                && ExitWindowsEx(
                    flags,
                    SHTDN_REASON_MAJOR_OPERATINGSYSTEM
                        | SHTDN_REASON_MINOR_UPGRADE
                        | SHTDN_REASON_FLAG_PLANNED,
                ) == TRUE;
        }
    }
    false
}
