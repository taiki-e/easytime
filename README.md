# easytime

[![crates.io](https://img.shields.io/crates/v/easytime.svg?style=flat-square&logo=rust)](https://crates.io/crates/easytime)
[![docs.rs](https://img.shields.io/badge/docs.rs-easytime-blue?style=flat-square)](https://docs.rs/easytime)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.34+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/easytime/CI/master?style=flat-square)](https://github.com/taiki-e/easytime/actions?query=workflow%3ACI+branch%3Amaster)

Providing wrapper types for safely performing panic-free checked arithmetic
on instants and durations.

This crate provides the following two data structures.

- [`easytime::Instant`] -- A wrapper type for [`std::time::Instant`]

- [`easytime::Duration`] -- A wrapper type for [`std::time::Duration`]

[`easytime::Instant`]: https://docs.rs/easytime/0.1/easytime/struct.Instant.html
[`easytime::Duration`]: https://docs.rs/easytime/0.1/easytime/struct.Duration.html
[`std::time`]: https://doc.rust-lang.org/std/time/index.html
[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easytime = "0.1"
```

*Compiler support: requires rustc 1.34+*

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

If you use [`std::time`] directly, you need to write as follows:

```rust
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

- **`std`** *(enabled by default)*
  - Enable to use [`easytime::Instant`].
  - If disabled this feature, `easytime` can be used in `no_std` environments.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
