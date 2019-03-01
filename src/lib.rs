//! Providing wrapper types for safely performing panic-free checked arithmetic on instants and durations.
//!
//! This crate provides the following two data structures.
//!
//! * [`easytime::Instant`] -- A wrapper type for [`std::time::Instant`]
//!
//! * [`easytime::Duration`] -- A wrapper type for [`std::time::Duration`]
//!
//! [`easytime::Instant`]: struct.Instant.html
//! [`easytime::Duration`]: struct.Instant.html
//! [`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
//! [`std::time::Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
//!
//! ## Examples
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! use easytime::{Duration, Instant};
//! # #[cfg(feature = "std")]
//! use std::time::Duration as StdDuration;
//!
//! # #[cfg(feature = "std")]
//! fn foo(secs: u64, nanos: u32, instant: Instant) -> Option<StdDuration> {
//!     let now = Instant::now();
//!
//!     let dur = Duration::new(secs, nanos);
//!     (now - instant - dur).into_inner()
//! }
//! ```
//!
//! If you use `std::time` directly, you need to write as follows:
//!
//! ```rust
//! #![feature(checked_duration_since)]
//!
//! use std::time::{Duration, Instant};
//!
//! fn foo(secs: u64, nanos: u32, instant: Instant) -> Option<Duration> {
//!     let now = Instant::now();
//!
//!     let secs = Duration::from_secs(secs);
//!     let nanos = Duration::from_nanos(u64::from(nanos));
//!
//!     let dur = secs.checked_add(nanos)?;
//!     now.checked_duration_since(instant)?.checked_sub(dur)
//! }
//! ```
//!
//! ## Optional features
//!
//! * **`std`** *(enabled by default)*
//!   * Enable to use [`easytime::Instant`].
//!   * This requires Rust 1.34 or later.
//!   * If disabled this feature, easytime can compile with Rust 1.33.
//!

#![doc(html_root_url = "https://docs.rs/easytime/0.1.1")]
#![deny(missing_docs, missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![cfg_attr(not(feature = "std"), no_std)]

mod duration;
pub use duration::Duration;

#[cfg(feature = "std")]
mod instant;
#[cfg(feature = "std")]
pub use instant::Instant;

fn pair_and_then<A, B, C, F>(x: Option<A>, y: Option<B>, f: F) -> Option<C>
where
    F: FnOnce(A, B) -> Option<C>,
{
    match (x, y) {
        (Some(x), Some(y)) => f(x, y),
        _ => None,
    }
}
