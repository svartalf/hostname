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

#[cfg(unix)]
extern "C" {
    fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

#[cfg(windows)]
unsafe fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int {
    use winapi::um::sysinfoapi as nfo;

    let mut size = size as u32;

    // 0 == failure for this function
    if nfo::GetComputerNameExA(
        nfo::ComputerNameDnsHostname,
        name,
        &mut size
    ) != 0 {
        0
    } else {
        1
    }
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


    #[cfg(unix)]
    #[bench]
    fn bench_get_hostname(b: &mut test::Bencher) {
        b.iter(|| get_hostname().unwrap())
    }

    #[cfg(windows)]
    #[bench]
    fn bench_get_hostname(b: &mut test::Bencher) {
        b.iter(|| get_hostname().unwrap());
    }
}
