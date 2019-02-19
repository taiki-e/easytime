# easytime

[![Build Status](https://travis-ci.com/taiki-e/easytime.svg?branch=master)](https://travis-ci.com/taiki-e/easytime)
[![version](https://img.shields.io/crates/v/easytime.svg)](https://crates.io/crates/easytime/)
[![documentation](https://docs.rs/easytime/badge.svg)](https://docs.rs/easytime/)
[![license](https://img.shields.io/crates/l/easytime.svg)](https://crates.io/crates/easytime/)

Providing wrapper types for safely performing panic-free checked arithmetic on instants and durations.

This crate provides the following two data structures.

* `easytime::Instant` -- A wrapper type for [`std::time::Instant`]

* `easytime::Duration` -- A wrapper type for [`std::time::Duration`]

[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easytime = "0.1"
```

The current version of easytime requires Rust 1.34 or later.

[**Documentation**](https://docs.rs/easytime/)

## Optional features

* **`std`** *(enabled by default)*
  * Enable to use `easytime::Instant`.
  * This requires Rust 1.34 or later.
  * If disabled this feature, easytime can compile with Rust 1.33.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
