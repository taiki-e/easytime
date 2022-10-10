/*!
Providing wrapper types for safely performing panic-free checked arithmetic
on instants and durations.

This crate provides the following two data structures.

- [`easytime::Instant`] -- A wrapper type for [`std::time::Instant`]

- [`easytime::Duration`] -- A wrapper type for [`std::time::Duration`]

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easytime = "0.2"
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
    let nanos = Duration::from_nanos(nanos as u64);

    let dur = secs.checked_add(nanos)?;
    now.checked_duration_since(instant)?.checked_sub(dur)
}
```

## Optional features

- **`std`** *(enabled by default)*
  - Enable to use [`easytime::Instant`].
  - If disabled this feature, `easytime` can be used in `no_std` environments.
*/

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    single_use_lifetimes,
    unreachable_pub
)]
#![warn(
    clippy::pedantic,
    // lints for public library
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    // clippy::std_instead_of_alloc, // alloc requires Rust 1.36
    clippy::std_instead_of_core,
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::manual_map, // Option::map is not const
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pattern_matching, // const Option::is_some requires Rust 1.48
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(doc)]
extern crate self as easytime;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
#[cfg(test)]
#[path = "gen/assert_impl.rs"]
mod assert_impl;

mod duration;
#[allow(unreachable_pub)] // false positive: https://github.com/rust-lang/rust/issues/102352
pub use crate::duration::Duration;

#[cfg(feature = "std")]
mod instant;
#[cfg(feature = "std")]
pub use crate::instant::Instant;

mod error;
#[allow(unreachable_pub)] // false positive: https://github.com/rust-lang/rust/issues/102352
pub use crate::error::TryFromTimeError;

// =============================================================================
// Utilities

#[inline]
fn pair_and_then<A, B, C, F>(x: Option<A>, y: Option<B>, f: F) -> Option<C>
where
    F: FnOnce(A, B) -> Option<C>,
{
    match (x, y) {
        (Some(x), Some(y)) => f(x, y),
        _ => None,
    }
}
