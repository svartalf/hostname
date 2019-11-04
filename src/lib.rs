//! A crate with utilities to get and set the system host name.
//!
//! ## Examples
//!
//! Set and get the host name:
//!
//! ```rust,no_run
//! # use std::io;
//! # use std::ffi::OsStr;
//! # fn try_main() -> io::Result<()> {
//! hostname::set("potato")?;
//! let new_name = hostname::get()?;
//! assert_eq!(new_name, OsStr::new("potato"));
//! # Ok(())
//! # }
//! # fn main() {
//! #    try_main().unwrap();
//! # }
//! ```
#![doc(html_root_url = "https://docs.rs/hostname/0.1.5")]
#![deny(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated,
    rust_2018_idioms,
    trivial_casts,
    unused_import_braces,
    unused_results
)]
#![allow(unknown_lints, unused_extern_crates)]

#[macro_use]
extern crate match_cfg;

use std::ffi::{OsStr, OsString};
use std::io;

match_cfg! {
    #[cfg(any(unix, target_os = "redox"))] => {
        extern crate libc;

        mod nix;
        use ::nix as sys;
    }
    #[cfg(target_os = "windows")] => {
        extern crate winapi;

        mod windows;
        use ::windows as sys;
    }
    _ => {
        compile_error!("Unsupported target OS! Create an issue: https://github.com/svartalf/hostname/issues/new");
    }
}

/// Return the system hostname.
///
/// ## Example
///
/// ```rust
/// # use std::io;
/// # fn try_main() -> io::Result<()> {
/// let name = hostname::get()?;
/// # Ok(())
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// # }
/// ```
pub fn get() -> io::Result<OsString> {
    sys::get()
}

/// Set the system hostname.
///
/// ## Example
///
/// ```rust,no_run
/// # use std::io;
/// # fn try_main() -> io::Result<()> {
/// hostname::set("potato")?;
/// # Ok(())
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// # }
/// ```
pub fn set<T>(hostname: T) -> io::Result<()>
where
    T: AsRef<OsStr>,
{
    sys::set(hostname.as_ref())
}

/// Get hostname.
///
/// ## Deprecation
///
/// This function is deprecated and will be removed in the `0.3.0` version.
/// Consider using [get](fn.get.html) instead.
#[deprecated(since = "0.2.0", note = "Use hostname::get() instead")]
pub fn get_hostname() -> Option<String> {
    get()
        .ok()
        .map(|os_string| os_string.to_string_lossy().into_owned())
}
