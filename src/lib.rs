//! Providing wrapper types for safely performing panic-free checked arithmetic
//! on instants and durations.
//!
//! This crate provides the following two data structures.
//!
//! * [`easytime::Instant`] -- A wrapper type for [`std::time::Instant`]
//!
//! * [`easytime::Duration`] -- A wrapper type for [`std::time::Duration`]
//!
//! [`easytime::Instant`]: Instant
//! [`easytime::Duration`]: Duration
//!
//! # Examples
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
//! If you use [`std::time`] directly, you need to write as follows:
//!
//! ```rust
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
//! # Optional features
//!
//! * **`std`** *(enabled by default)*
//!   * Enable to use [`easytime::Instant`].
//!   * If disabled this feature, `easytime` can be used in `no_std` environments.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(missing_debug_implementations, missing_docs)]
#![warn(clippy::all, clippy::default_trait_access)]

mod duration;
pub use crate::duration::Duration;

#[cfg(feature = "std")]
mod instant;
#[cfg(feature = "std")]
pub use crate::instant::Instant;

use core::fmt;

// =============================================================================
// TryFromTimeError

/// The error type returned when a conversion from `easytime` types to `std::time` types fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromTimeError(());

impl fmt::Display for TryFromTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid arithmetic attempted on instants or durations")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromTimeError {}

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
