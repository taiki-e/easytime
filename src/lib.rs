// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

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

## Examples

```
use easytime::{Duration, Instant};
use std::time::Duration as StdDuration;

fn foo(secs: u64, nanos: u32, instant: Instant) -> Option<StdDuration> {
    let now = Instant::now();

    let dur = Duration::new(secs, nanos);
    (now - instant - dur).into_inner()
}
```

If you use [`std::time`] directly, you need to write as follows:

```
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

<!-- tidy:sync-markdown-to-rustdoc:end -->
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
// docs.rs only (cfg is enabled by docs.rs, not build script)
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(doc)]
extern crate self as easytime;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
#[cfg(test)]
#[path = "gen/tests/assert_impl.rs"]
mod assert_impl;
#[cfg(test)]
#[path = "gen/tests/track_size.rs"]
mod track_size;

#[macro_use]
mod utils;

mod duration;
pub use self::duration::Duration;

#[cfg(feature = "std")]
mod instant;
#[cfg(feature = "std")]
pub use self::instant::Instant;

mod error;
pub use self::error::TryFromTimeError;
