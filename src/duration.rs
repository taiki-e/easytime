use const_fn::const_fn;
use core::{
    convert::TryFrom,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    time,
};

use super::{pair_and_then, TryFromTimeError};

/// A `Duration` type to represent a span of time, typically used for system
/// timeouts.
///
/// Each `Duration` is composed of a whole number of seconds and a fractional part
/// represented in nanoseconds.  If the underlying system does not support
/// nanosecond-level precision, APIs binding a system timeout will typically round up
/// the number of nanoseconds.
///
/// `Duration`s implement many common traits, including [`Add`], [`Sub`], and other
/// [`ops`] traits.
///
/// [`Add`]: std::ops::Add
/// [`Sub`]: std::ops::Sub
/// [`ops`]: std::ops
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub(crate) Option<time::Duration>);

impl Duration {
    /// Creates a new `Duration` from the specified number of whole seconds and
    /// additional nanoseconds.
    ///
    /// If the number of nanoseconds is greater than 1 billion (the number of
    /// nanoseconds in a second), then it will carry over into the seconds provided.
    #[inline]
    pub fn new(secs: u64, nanos: u32) -> Self {
        let secs = time::Duration::from_secs(secs);
        let nanos = time::Duration::from_nanos(u64::from(nanos));
        Self(secs.checked_add(nanos))
    }

    /// Creates a new `Duration` from the specified number of whole seconds.
    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        Self(Some(time::Duration::from_secs(secs)))
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        Self(Some(time::Duration::from_millis(millis)))
    }

    /// Creates a new `Duration` from the specified number of microseconds.
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        Self(Some(time::Duration::from_micros(micros)))
    }

    /// Creates a new `Duration` from the specified number of nanoseconds.
    #[inline]
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(Some(time::Duration::from_nanos(nanos)))
    }

    /// Returns the number of _whole_ seconds contained by this `Duration`.
    ///
    /// The returned value does not include the fractional (nanosecond) part of the
    /// duration, which can be obtained using [`subsec_nanos`].
    ///
    /// [`subsec_nanos`]: Self::subsec_nanos
    #[inline]
    #[const_fn("1.46")]
    pub const fn as_secs(&self) -> Option<u64> {
        match &self.0 {
            Some(d) => Some(d.as_secs()),
            None => None,
        }
    }

    /// Returns the fractional part of this `Duration`, in whole milliseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by milliseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one thousand).
    #[inline]
    #[const_fn("1.46")]
    pub const fn subsec_millis(&self) -> Option<u32> {
        match &self.0 {
            Some(d) => Some(d.subsec_millis()),
            None => None,
        }
    }

    /// Returns the fractional part of this `Duration`, in whole microseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by microseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one million).
    #[inline]
    #[const_fn("1.46")]
    pub const fn subsec_micros(&self) -> Option<u32> {
        match &self.0 {
            Some(d) => Some(d.subsec_micros()),
            None => None,
        }
    }

    /// Returns the fractional part of this `Duration`, in nanoseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by nanoseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one billion).
    #[inline]
    #[const_fn("1.46")]
    pub const fn subsec_nanos(&self) -> Option<u32> {
        match &self.0 {
            Some(d) => Some(d.subsec_nanos()),
            None => None,
        }
    }

    /// Returns the total number of whole milliseconds contained by this `Duration`.
    #[inline]
    #[const_fn("1.46")]
    pub const fn as_millis(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_millis()),
            None => None,
        }
    }

    /// Returns the total number of whole microseconds contained by this `Duration`.
    #[inline]
    #[const_fn("1.46")]
    pub const fn as_micros(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_micros()),
            None => None,
        }
    }

    /// Returns the total number of nanoseconds contained by this `Duration`.
    #[inline]
    #[const_fn("1.46")]
    pub const fn as_nanos(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_nanos()),
            None => None,
        }
    }

    // TODO: duration_float https://github.com/rust-lang/rust/issues/54361
    // TODO: div_duration https://github.com/rust-lang/rust/issues/63139

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

    /// Returns the contained [`std::time::Duration`] or `None`.
    #[inline]
    pub const fn into_inner(self) -> Option<time::Duration> {
        self.0
    }

    /// Returns the contained [`std::time::Duration`] or a default.
    ///
    /// `dur.unwrap_or(default)` is equivalent to `dur.into_inner().unwrap_or(default)`.
    #[inline]
    #[const_fn("1.46")]
    pub const fn unwrap_or(self, default: time::Duration) -> time::Duration {
        match self.0 {
            Some(d) => d,
            None => default,
        }
    }

    /// Returns the contained [`std::time::Duration`] or computes it from a closure.
    ///
    /// `dur.unwrap_or_else(default)` is equivalent to `dur.into_inner().unwrap_or_else(default)`.
    #[inline]
    pub fn unwrap_or_else<F>(self, default: F) -> time::Duration
    where
        F: FnOnce() -> time::Duration,
    {
        self.0.unwrap_or_else(default)
    }
}

// =============================================================================
// Trait implementations

impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self(Some(time::Duration::default()))
    }
}

impl From<time::Duration> for Duration {
    fn from(dur: time::Duration) -> Self {
        Self(Some(dur))
    }
}

impl TryFrom<Duration> for time::Duration {
    type Error = TryFromTimeError;

    fn try_from(dur: Duration) -> Result<Self, Self::Error> {
        dur.into_inner().ok_or_else(|| TryFromTimeError(()))
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(pair_and_then(self.0, rhs.0, time::Duration::checked_add))
    }
}

impl Add<time::Duration> for Duration {
    type Output = Self;

    fn add(self, rhs: time::Duration) -> Self::Output {
        Self(self.0.and_then(|lhs| lhs.checked_add(rhs)))
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<time::Duration> for Duration {
    fn add_assign(&mut self, rhs: time::Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(pair_and_then(self.0, rhs.0, time::Duration::checked_sub))
    }
}

impl Sub<time::Duration> for Duration {
    type Output = Self;

    fn sub(self, rhs: time::Duration) -> Self::Output {
        Self(self.0.and_then(|lhs| lhs.checked_sub(rhs)))
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<time::Duration> for Duration {
    fn sub_assign(&mut self, rhs: time::Duration) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0.and_then(|lhs| lhs.checked_mul(rhs)))
    }
}

impl Mul<Duration> for u32 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<u32> for Duration {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Duration {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self(self.0.and_then(|lhs| lhs.checked_div(rhs)))
    }
}

impl DivAssign<u32> for Duration {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

// TODO: duration_sum
// impl Sum for Duration
// impl<'a> Sum<&'a Duration> for Duration
