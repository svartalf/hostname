//! Get hostname. Compatible with windows and linux.
//!
//! # Examples
//! ```
//! extern crate hostname;
//!
//! assert!(hostname::get_hostname().is_some());
//! ```
//!
#![cfg_attr(all(feature = "unstable", test), feature(test))]

extern crate libc;
#[cfg(windows)]
extern crate winapi;

use std::ffi::CStr;
#[cfg(windows)]
use std::mem;

#[cfg(unix)]
extern "C" {
    fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

#[cfg(windows)]
unsafe fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int {
    let mut wsa_data = mem::uninitialized();

    // 514 == 2.2, which is ~20 years old at this point, so should be safe ;)
    let startup_res = winapi::um::winsock2::WSAStartup(514, &mut wsa_data);
    if startup_res != 0 {
        return startup_res;
    }

    let res = winapi::um::winsock2::gethostname(name, size as i32);

    winapi::um::winsock2::WSACleanup();

    res
}

/// Get hostname.
pub fn get_hostname() -> Option<String> {
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr() as *mut libc::c_char;

    unsafe {
        if gethostname(ptr, len as libc::size_t) != 0 {
            return None;
        }

        Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
}

#[test]
fn test_get_hostname() {
    assert!(get_hostname().is_some());
    assert!(!get_hostname().unwrap().is_empty());
}

#[cfg(all(feature = "unstable", test))]
mod benches {
    extern crate test;
    use super::get_hostname;

    #[bench]
    fn bench_get_hostname(b: &mut test::Bencher) {
        b.iter(|| get_hostname().unwrap())
    }
}
