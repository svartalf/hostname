# hostname

[![Latest Version](https://img.shields.io/crates/v/hostname.svg)](https://crates.io/crates/hostname)
[![Latest Version](https://docs.rs/hostname/badge.svg)](https://docs.rs/hostname)
[![Build Status](https://github.com/svartalf/hostname/workflows/Continuous%20integration/badge.svg)](https://github.com/svartalf/hostname/actions)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.13+-green.svg)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

> Cross-platform hostname functions, compatible with Windows and *nix systems.

## [Document](https://docs.rs/hostname)

## Usage

Add dependency to Cargo.toml

```toml
[dependencies]
hostname = "^0.1"
```

In your `main.rs` or `lib.rs`:

```rust
extern crate hostname;
```

## Examples

```rust
use hostname::get_hostname;

assert!(get_hostname().is_some());
```

## License

hostname is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details.
