// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- tidy:crate-doc:start -->
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

*Compiler support: requires rustc 1.58+*

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

<!-- tidy:crate-doc:end -->
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
    // Lints that may help when writing public library.
    missing_debug_implementations,
    missing_docs,
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    // clippy::missing_inline_in_public_items,
    // clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::must_use_candidate
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

#[macro_use]
mod utils;

mod duration;
pub use crate::duration::Duration;

#[cfg(feature = "std")]
mod instant;
#[cfg(feature = "std")]
pub use crate::instant::Instant;

mod error;
pub use crate::error::TryFromTimeError;
