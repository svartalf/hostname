use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use libc;

pub fn get() -> io::Result<OsString> {
    // According to the POSIX specification,
    // host names are limited to `HOST_NAME_MAX` bytes
    //
    // https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
    let size =
        unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) as libc::size_t };

    let mut buffer = vec![0u8; size];

    let result = unsafe {
        libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, size)
    };

    if result != 0 {
        return Err(io::Error::last_os_error());
    }

    // Returned name might be truncated if it does not fit
    // and `buffer` will not contain the trailing \0 in that case.
    // Manually capping the buffer length here.
    let end = buffer
        .iter()
        .position(|&byte| byte == 0x00)
        .unwrap_or_else(|| buffer.len());
    buffer.resize(end, 0x00);

    Ok(OsString::from_vec(buffer))
}

pub fn set(hostname: &OsStr) -> io::Result<()> {
    #[cfg(not(any(target_os = "macos")))]
    let size = hostname.len() as libc::size_t;

    #[cfg(any(target_os = "macos"))]
    let size = hostname.len() as libc::c_int;

    let result = unsafe {
        libc::sethostname(
            hostname.as_bytes().as_ptr() as *const libc::c_char,
            size,
        )
    };

    if result != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
