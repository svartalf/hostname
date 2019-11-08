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

    Ok(wrap_buffer(buffer))
}

fn wrap_buffer(mut bytes: Vec<u8>) -> OsString {
    // Returned name might be truncated if it does not fit
    // and `buffer` will not contain the trailing \0 in that case.
    // Manually capping the buffer length here.
    let end = bytes
        .iter()
        .position(|&byte| byte == 0x00)
        .unwrap_or_else(|| bytes.len());
    bytes.resize(end, 0x00);

    OsString::from_vec(bytes)
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

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::wrap_buffer;

    // Happy path case: there is a correct null terminated C string in a buffer
    // and a bunch of NULL characters from the pre-allocated buffer
    #[test]
    fn test_non_overflowed_buffer() {
        let buf = b"potato\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new("potato"));
    }

    #[test]
    fn test_empty_buffer() {
        let buf = b"".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new(""));
    }

    #[test]
    fn test_filled_with_null_buffer() {
        let buf = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new(""));
    }

    // Hostname value had overflowed the buffer, so it was truncated
    // and according to the POSIX documentation of the `gethostname`:
    //
    // > it is unspecified whether the returned name is null-terminated.
    #[test]
    fn test_overflowed_buffer() {
        let buf = b"potat".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new("potat"));
    }
}
