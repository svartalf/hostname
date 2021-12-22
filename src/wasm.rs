use std::io;
#[cfg(feature = "set")]
use std::ffi::OsStr;
use std::ffi::OsString;
#[cfg(feature = "set")]
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;

use libc;

const _SC_HOST_NAME_MAX: libc::c_int = 180;

pub fn get() -> io::Result<OsString> {
    // According to the POSIX specification,
    // host names are limited to `HOST_NAME_MAX` bytes
    //
    // https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
    let size =
        unsafe { libc::sysconf(_SC_HOST_NAME_MAX) as libc::size_t };

    // "wasihost" string buffer
    let mut buffer = vec![0x77,0x61,0x73,0x69,0x68,0x6f,0x73,0x74,0x00; size];

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