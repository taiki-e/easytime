# easytime

[![Build Status](https://travis-ci.com/taiki-e/easytime.svg?branch=master)](https://travis-ci.com/taiki-e/easytime)
[![version](https://img.shields.io/crates/v/easytime.svg)](https://crates.io/crates/easytime/)
[![documentation](https://docs.rs/easytime/badge.svg)](https://docs.rs/easytime/)
[![license](https://img.shields.io/crates/l/easytime.svg)](https://crates.io/crates/easytime/)

Providing wrapper types for safely performing panic-free checked arithmetic on instants and durations.

This crate provides the following two data structures.

* [`easytime::Instant`] -- A wrapper type for [`std::time::Instant`]

* [`easytime::Duration`] -- A wrapper type for [`std::time::Duration`]

[`easytime::Instant`]: https://docs.rs/easytime/0.1/easytime/struct.Instant.html
[`easytime::Duration`]: https://docs.rs/easytime/0.1/easytime/struct.Duration.html
[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html

[**Documentation**](https://docs.rs/easytime/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easytime = "0.1"
```

The current easytime requires Rust 1.34 or later.

## Examples

```rust
use easytime::{Duration, Instant};
use std::time::Duration as StdDuration;

fn foo(secs: u64, nanos: u32, instant: Instant) -> Option<StdDuration> {
    let now = Instant::now();

    let dur = Duration::new(secs, nanos);
    (now - instant - dur).into_inner()
}
```

If you use `std::time` directly, you need to write as follows:

```rust
#![feature(checked_duration_since)]

use std::time::{Duration, Instant};

fn foo(secs: u64, nanos: u32, instant: Instant) -> Option<Duration> {
    let now = Instant::now();

    let secs = Duration::from_secs(secs);
    let nanos = Duration::from_nanos(u64::from(nanos));

    let dur = secs.checked_add(nanos)?;
    now.checked_duration_since(instant)?.checked_sub(dur)
}
```

## Optional features

* **`std`** *(enabled by default)*
  * Enable to use [`easytime::Instant`].
  * If disabled this feature, `easytime` can be used in `no_std` environments.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
