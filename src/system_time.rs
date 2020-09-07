use core::{
    convert::TryFrom,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use std::time;

use const_fn::const_fn;

use super::{pair_and_then, Duration, TryFromTimeError};

/// A measurement of the system clock, useful for talking to
/// external entities like the file system or other processes.
///
/// Distinct from the [`Instant`] type, this time measurement **is not
/// monotonic**. This means that you can save a file to the file system, then
/// save another file to the file system, **and the second file has a
/// `SystemTime` measurement earlier than the first**. In other words, an
/// operation that happens after another operation in real time may have an
/// earlier `SystemTime`!
///
/// Consequently, comparing two `SystemTime` instances to learn about the
/// duration between them returns a [`Result`] instead of an infallible [`Duration`]
/// to indicate that this sort of time drift may happen and needs to be handled.
///
/// Although a `SystemTime` cannot be directly inspected, the [`UNIX_EPOCH`]
/// constant is provided in this module as an anchor in time to learn
/// information about a `SystemTime`. By calculating the duration from this
/// fixed point in time, a `SystemTime` can be converted to a human-readable time,
/// or perhaps some other string representation.
///
/// The size of a `SystemTime` struct may vary depending on the target operating
/// system.
///
/// [`Instant`]: super::Instant
/// [`Duration`]: super::Duration
/// [`UNIX_EPOCH`]: Self::UNIX_EPOCH
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime(Option<time::SystemTime>);

impl SystemTime {
    /// An anchor in time which can be used to create new `SystemTime` instances or
    /// learn about where in time a `SystemTime` lies.
    ///
    /// This constant is defined to be "1970-01-01 00:00:00 UTC" on all systems with
    /// respect to the system clock. Using `duration_since` on an existing
    /// `SystemTime` instance can tell how far away from this point in time a
    /// measurement lies, and using `UNIX_EPOCH + duration` can be used to create a
    /// `SystemTime` instance to represent another fixed point in time.
    pub const UNIX_EPOCH: Self = Self(Some(time::SystemTime::UNIX_EPOCH));

    /// Returns the system time corresponding to "now".
    pub fn now() -> Self {
        Self(Some(time::SystemTime::now()))
    }

    /// Returns the amount of time elapsed from an earlier point in time.
    ///
    /// This function may fail because measurements taken earlier are not
    /// guaranteed to always be before later measurements (due to anomalies such
    /// as the system clock being adjusted either forwards or backwards).
    pub fn duration_since(&self, earlier: Self) -> Duration {
        Duration(pair_and_then(self.0.as_ref(), earlier.0, |this, earlier| {
            this.duration_since(earlier).ok()
        }))
    }

    /// Returns the amount of time elapsed since this system time was created.
    ///
    /// This function may fail as the underlying system clock is susceptible to
    /// drift and updates (e.g., the system clock could go backwards), so this
    /// function may not always succeed.
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    // =============================================================================
    // Option based method implementations

    /// Returns `true` if [`into_inner`] returns `Some`.
    ///
    /// [`into_inner`]: Self::into_inner
    #[inline]
    #[const_fn("1.46")]
    pub const fn is_some(&self) -> bool {
        match &self.0 {
            Some(_) => true,
            None => false,
        }
    }

    /// Returns `true` if [`into_inner`] returns `None`.
    ///
    /// [`into_inner`]: Self::into_inner
    #[inline]
    #[const_fn("1.46")]
    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Returns the contained [`std::time::SystemTime`] or `None`.
    #[inline]
    pub const fn into_inner(self) -> Option<time::SystemTime> {
        self.0
    }

    /// Returns the contained [`std::time::SystemTime`] or a default.
    ///
    /// `system_time.unwrap_or(default)` is equivalent to `system_time.into_inner().unwrap_or(default)`.
    #[inline]
    #[const_fn("1.46")]
    pub const fn unwrap_or(self, default: time::SystemTime) -> time::SystemTime {
        match self.0 {
            Some(d) => d,
            None => default,
        }
    }

    /// Returns the contained [`std::time::SystemTime`] or computes it from a closure.
    ///
    /// `system_time.unwrap_or_else(default)` is equivalent to `system_time.into_inner().unwrap_or_else(default)`.
    #[inline]
    pub fn unwrap_or_else<F>(self, default: F) -> time::SystemTime
    where
        F: FnOnce() -> time::SystemTime,
    {
        self.0.unwrap_or_else(default)
    }
}

// =============================================================================
// Trait implementations

impl From<time::SystemTime> for SystemTime {
    fn from(system_time: time::SystemTime) -> Self {
        Self(Some(system_time))
    }
}

impl TryFrom<SystemTime> for time::SystemTime {
    type Error = TryFromTimeError;

    fn try_from(system_time: SystemTime) -> Result<Self, Self::Error> {
        system_time.into_inner().ok_or(TryFromTimeError(()))
    }
}

impl Add<Duration> for SystemTime {
    type Output = Self;

    fn add(self, other: Duration) -> Self::Output {
        Self(pair_and_then(self.0.as_ref(), other.0, time::SystemTime::checked_add))
    }
}

impl Add<time::Duration> for SystemTime {
    type Output = Self;

    fn add(self, other: time::Duration) -> Self::Output {
        Self(self.0.and_then(|this| this.checked_add(other)))
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl AddAssign<time::Duration> for SystemTime {
    fn add_assign(&mut self, other: time::Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, other: Duration) -> Self::Output {
        Self(pair_and_then(self.0.as_ref(), other.0, time::SystemTime::checked_sub))
    }
}

impl Sub<time::Duration> for SystemTime {
    type Output = Self;

    fn sub(self, other: time::Duration) -> Self::Output {
        Self(self.0.and_then(|this| this.checked_sub(other)))
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl SubAssign<time::Duration> for SystemTime {
    fn sub_assign(&mut self, other: time::Duration) {
        *self = *self - other;
    }
}

impl Sub for SystemTime {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        self.duration_since(other)
    }
}

impl Sub<time::SystemTime> for SystemTime {
    type Output = Duration;

    fn sub(self, other: time::SystemTime) -> Self::Output {
        self.duration_since(Self::from(other))
    }
}
