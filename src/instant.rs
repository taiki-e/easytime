use core::{
    cmp::Ordering,
    convert::TryFrom,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use std::time;

use const_fn::const_fn;

use super::{pair_and_then, Duration, TryFromTimeError};

/// A measurement of a monotonically nondecreasing clock.
/// Opaque and useful only with `Duration`.
///
/// Instants are always guaranteed to be no less than any previously measured
/// instant when created, and are often useful for tasks such as measuring
/// benchmarks or timing how long an operation takes.
///
/// Note, however, that instants are not guaranteed to be **steady**. In other
/// words, each tick of the underlying clock may not be the same length (e.g.
/// some seconds may be longer than others). An instant may jump forwards or
/// experience time dilation (slow down or speed up), but it will never go
/// backwards.
///
/// Instants are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an instant. Instead, it only
/// allows measuring the duration between two instants (or comparing two
/// instants).
///
/// The size of an `Instant` struct may vary depending on the target operating
/// system.
///
/// # OS-specific behaviors
///
/// An `Instant` is a wrapper around system-specific types and it may behave
/// differently depending on the underlying operating system. For example,
/// the following snippet is fine on Linux but fails and returns `None` on macOS:
///
/// ```
/// use easytime::{Duration, Instant};
///
/// let now = Instant::now();
/// let max_nanoseconds = u64::MAX / 1_000_000_000;
/// let duration = Duration::new(max_nanoseconds, 0);
/// // Unlike `std::time::Instant`, it won't panic!
/// let result = now + duration;
///
/// #[cfg(target_os = "linux")]
/// assert!(result.is_some());
/// #[cfg(target_os = "macos")]
/// assert!(result.is_none());
/// ```
///
/// # Underlying System calls
///
/// See the [standard library documentation](std::time::Instant#underlying-system-calls)
/// for the system calls used to get the current time using `now()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Instant(Option<time::Instant>);

impl Instant {
    /// Returns a "none" value
    pub const NONE: Self = Self(None);

    /// Returns an instant corresponding to "now".
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Instant;
    ///
    /// let now = Instant::now();
    /// ```
    pub fn now() -> Self {
        Self(Some(time::Instant::now()))
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{thread::sleep, time};
    ///
    /// use easytime::Instant;
    ///
    /// let now = Instant::now();
    /// sleep(time::Duration::new(1, 0));
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.duration_since(now));
    /// ```
    #[cfg(not(stable_lt_1_39))]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        Duration(pair_and_then(self.0.as_ref(), earlier.0, time::Instant::checked_duration_since))
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{thread::sleep, time};
    ///
    /// use easytime::Instant;
    ///
    /// let now = Instant::now();
    /// sleep(time::Duration::new(1, 0));
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.duration_since(now));
    /// println!("{:?}", now.duration_since(new_now)); // None
    /// ```
    #[cfg(stable_lt_1_39)]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        Duration(pair_and_then(self.0.as_ref(), earlier.0, |this, earlier| {
            // https://github.com/rust-lang/rust/pull/58395
            if *this >= earlier {
                Some(this.duration_since(earlier))
            } else {
                None
            }
        }))
    }

    /// Returns the amount of time elapsed since this instant was created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{thread::sleep, time};
    ///
    /// use easytime::Instant;
    ///
    /// let instant = Instant::now();
    /// let three_secs = time::Duration::from_secs(3);
    /// sleep(three_secs);
    /// assert!(instant.elapsed() >= three_secs);
    /// ```
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    // =============================================================================
    // Option based method implementations

    /// Returns `true` if [`into_inner`](Self::into_inner) returns `Some`.
    ///
    /// This is `const fn` on Rust 1.46+.
    #[inline]
    #[const_fn("1.46")]
    pub const fn is_some(&self) -> bool {
        match &self.0 {
            Some(_) => true,
            None => false,
        }
    }

    /// Returns `true` if [`into_inner`](Self::into_inner) returns `None`.
    ///
    /// This is `const fn` on Rust 1.46+.
    #[inline]
    #[const_fn("1.46")]
    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Returns the contained [`std::time::Instant`] or `None`.
    #[inline]
    pub const fn into_inner(self) -> Option<time::Instant> {
        self.0
    }

    /// Returns the contained [`std::time::Instant`] or a default.
    ///
    /// `instant.unwrap_or(default)` is equivalent to `instant.into_inner().unwrap_or(default)`.
    ///
    /// This is `const fn` on Rust 1.46+.
    #[inline]
    #[const_fn("1.46")]
    pub const fn unwrap_or(self, default: time::Instant) -> time::Instant {
        match self.0 {
            Some(d) => d,
            None => default,
        }
    }

    /// Returns the contained [`std::time::Instant`] or computes it from a closure.
    ///
    /// `instant.unwrap_or_else(default)` is equivalent to `instant.into_inner().unwrap_or_else(default)`.
    #[inline]
    pub fn unwrap_or_else<F>(self, default: F) -> time::Instant
    where
        F: FnOnce() -> time::Instant,
    {
        self.0.unwrap_or_else(default)
    }
}

// =============================================================================
// Trait implementations

impl PartialEq<time::Instant> for Instant {
    fn eq(&self, other: &time::Instant) -> bool {
        self.0.map_or(false, |this| this == *other)
    }
}

impl PartialEq<Instant> for time::Instant {
    fn eq(&self, other: &Instant) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<time::Instant> for Instant {
    fn partial_cmp(&self, other: &time::Instant) -> Option<Ordering> {
        self.0.as_ref().and_then(|this| this.partial_cmp(other))
    }
}

impl PartialOrd<Instant> for time::Instant {
    fn partial_cmp(&self, other: &Instant) -> Option<Ordering> {
        other.0.as_ref().and_then(|other| self.partial_cmp(other))
    }
}

impl From<time::Instant> for Instant {
    fn from(instant: time::Instant) -> Self {
        Self(Some(instant))
    }
}

impl From<Option<time::Instant>> for Instant {
    fn from(dur: Option<time::Instant>) -> Self {
        Self(dur)
    }
}

impl TryFrom<Instant> for time::Instant {
    type Error = TryFromTimeError;

    fn try_from(instant: Instant) -> Result<Self, Self::Error> {
        instant.into_inner().ok_or(TryFromTimeError(()))
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, other: Duration) -> Self::Output {
        Self(pair_and_then(self.0.as_ref(), other.0, time::Instant::checked_add))
    }
}

impl Add<time::Duration> for Instant {
    type Output = Self;

    fn add(self, other: time::Duration) -> Self::Output {
        Self(self.0.and_then(|this| this.checked_add(other)))
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl AddAssign<time::Duration> for Instant {
    fn add_assign(&mut self, other: time::Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, other: Duration) -> Self::Output {
        Self(pair_and_then(self.0.as_ref(), other.0, time::Instant::checked_sub))
    }
}

impl Sub<time::Duration> for Instant {
    type Output = Self;

    fn sub(self, other: time::Duration) -> Self::Output {
        Self(self.0.and_then(|this| this.checked_sub(other)))
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl SubAssign<time::Duration> for Instant {
    fn sub_assign(&mut self, other: time::Duration) {
        *self = *self - other;
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        self.duration_since(other)
    }
}

impl Sub<time::Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: time::Instant) -> Self::Output {
        self.duration_since(Self::from(other))
    }
}
