use std::io;
#[cfg(feature = "set")]
use std::ffi::OsStr;
use std::ffi::OsString;
#[cfg(feature = "set")]
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;



use windows::Win32::System::SystemInformation::ComputerNamePhysicalDnsHostname;



pub fn get() -> io::Result<OsString> {
    use windows::core::PWSTR;
    use windows::Win32::System::SystemInformation::GetComputerNameExW;

    let mut size = 0;
    unsafe {
        // Don't care much about the result here,
        // it is guaranteed to return an error,
        // since we passed the NULL pointer as a buffer
        let result = GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PWSTR::null(),
            &mut size,
        );
        debug_assert_eq!(result.0, 0);
    };

    let mut buffer = Vec::with_capacity(size as usize);

    let result = unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PWSTR::from_raw(buffer.as_mut_ptr()),
            &mut size,
        )
    };

    if !result.as_bool() {
        Err(io::Error::last_os_error())
    } else {
        unsafe {
            buffer.set_len(size as usize);
        }

        Ok(OsString::from_wide(&buffer))
    }
}

#[cfg(feature = "set")]
pub fn set(hostname: &OsStr) -> io::Result<()> {
    use windows::core::PCWSTR;
    use windows::Win32::System::SystemInformation::SetComputerNameExW;

    let buffer = hostname.encode_wide().collect::<Vec<_>>();

    let result = unsafe {
        SetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PCWSTR::from_raw(buffer.as_ptr()),
        )
    };

    if !result.as_bool() {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
