// SPDX-License-Identifier: Apache-2.0 OR MIT

// Refs: https://github.com/rust-lang/rust/blob/254b59607d4417e9dffbc307138ae5c86280fe4c/library/core/src/time.rs

#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use core::{
    cmp, fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    time,
};

use crate::{TryFromTimeError, utils::pair_and_then};

const NANOS_PER_SEC: u32 = 1_000_000_000;

/// A `Duration` type to represent a span of time, typically used for system
/// timeouts.
///
/// Each `Duration` is composed of a whole number of seconds and a fractional part
/// represented in nanoseconds. If the underlying system does not support
/// nanosecond-level precision, APIs binding a system timeout will typically round up
/// the number of nanoseconds.
///
/// [`Duration`]s implement many common traits, including [`Add`], [`Sub`], and other
/// [`ops`] traits. It implements [`Default`] by returning a zero-length `Duration`.
///
/// [`ops`]: core::ops
///
/// # Examples
///
/// ```
/// use easytime::Duration;
///
/// let five_seconds = Duration::new(5, 0);
/// let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);
///
/// assert_eq!(five_seconds_and_five_nanos.as_secs(), Some(5));
/// assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), Some(5));
///
/// let ten_millis = Duration::from_millis(10);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub(crate) Option<time::Duration>);

impl Duration {
    /// Returns a "none" value
    pub const NONE: Self = Self(None);

    /// A duration of zero time.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::ZERO;
    /// assert!(duration.is_zero());
    /// assert_eq!(duration.as_nanos(), Some(0));
    /// ```
    pub const ZERO: Self = Self::from_nanos(0);

    /// The maximum duration.
    ///
    /// May vary by platform as necessary. Must be able to contain the difference between
    /// two instances of `Instant` or two instances of `SystemTime`.
    /// This constraint gives it a value of about 584,942,417,355 years in practice,
    /// which is currently used on all platforms.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// assert_eq!(Duration::MAX, Duration::new(u64::MAX, 1_000_000_000 - 1));
    /// ```
    pub const MAX: Self = Self(Some(time::Duration::MAX));

    /// Creates a new `Duration` from the specified number of whole seconds and
    /// additional nanoseconds.
    ///
    /// If the number of nanoseconds is greater than 1 billion (the number of
    /// nanoseconds in a second), then it will carry over into the seconds provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let five_seconds = Duration::new(5, 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(secs: u64, nanos: u32) -> Self {
        let secs = time::Duration::from_secs(secs);
        let nanos = time::Duration::from_nanos(nanos as u64);
        Self(secs.checked_add(nanos))
    }

    /// Creates a new `Duration` from the specified number of whole seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_secs(5);
    ///
    /// assert_eq!(Some(5), duration.as_secs());
    /// assert_eq!(Some(0), duration.subsec_nanos());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_secs(secs: u64) -> Self {
        Self(Some(time::Duration::from_secs(secs)))
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_millis(2_569);
    ///
    /// assert_eq!(Some(2), duration.as_secs());
    /// assert_eq!(Some(569_000_000), duration.subsec_nanos());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self(Some(time::Duration::from_millis(millis)))
    }

    /// Creates a new `Duration` from the specified number of microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_micros(1_000_002);
    ///
    /// assert_eq!(Some(1), duration.as_secs());
    /// assert_eq!(Some(2000), duration.subsec_nanos());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_micros(micros: u64) -> Self {
        Self(Some(time::Duration::from_micros(micros)))
    }

    /// Creates a new `Duration` from the specified number of nanoseconds.
    ///
    /// Note: Using this on the return value of `as_nanos()` might cause unexpected behavior:
    /// `as_nanos()` returns a u128, and can return values that do not fit in u64, e.g. 585 years.
    /// Instead, consider using the pattern `Duration::new(d.as_secs(), d.subsec_nanos())`
    /// if you cannot copy/clone the Duration directly.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_nanos(1_000_000_123);
    ///
    /// assert_eq!(Some(1), duration.as_secs());
    /// assert_eq!(Some(123), duration.subsec_nanos());
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(Some(time::Duration::from_nanos(nanos)))
    }

    /// Returns true if this `Duration` spans no time.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// assert!(Duration::ZERO.is_zero());
    /// assert!(Duration::new(0, 0).is_zero());
    /// assert!(Duration::from_nanos(0).is_zero());
    /// assert!(Duration::from_secs(0).is_zero());
    ///
    /// assert!(!Duration::new(1, 1).is_zero());
    /// assert!(!Duration::from_nanos(1).is_zero());
    /// assert!(!Duration::from_secs(1).is_zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        matches!((self.as_secs(), self.subsec_nanos()), (Some(0), Some(0)))
    }

    /// Returns the number of _whole_ seconds contained by this `Duration`.
    ///
    /// The returned value does not include the fractional (nanosecond) part of the
    /// duration, which can be obtained using [`subsec_nanos`].
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::new(5, 730_023_852);
    /// assert_eq!(duration.as_secs(), Some(5));
    /// ```
    ///
    /// To determine the total number of seconds represented by the `Duration`
    /// including the fractional part, use [`as_secs_f64`] or [`as_secs_f32`]
    ///
    /// [`as_secs_f64`]: Self::as_secs_f64
    /// [`as_secs_f32`]: Self::as_secs_f32
    /// [`subsec_nanos`]: Self::subsec_nanos
    #[inline]
    #[must_use]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_millis(5_432);
    /// assert_eq!(duration.as_secs(), Some(5));
    /// assert_eq!(duration.subsec_millis(), Some(432));
    /// ```
    #[inline]
    #[must_use]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_micros(1_234_567);
    /// assert_eq!(duration.as_secs(), Some(1));
    /// assert_eq!(duration.subsec_micros(), Some(234_567));
    /// ```
    #[inline]
    #[must_use]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::from_millis(5_010);
    /// assert_eq!(duration.as_secs(), Some(5));
    /// assert_eq!(duration.subsec_nanos(), Some(10_000_000));
    /// ```
    #[inline]
    #[must_use]
    pub const fn subsec_nanos(&self) -> Option<u32> {
        match &self.0 {
            Some(d) => Some(d.subsec_nanos()),
            None => None,
        }
    }

    /// Returns the total number of whole milliseconds contained by this `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::new(5, 730_023_852);
    /// assert_eq!(duration.as_millis(), Some(5_730));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_millis(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_millis()),
            None => None,
        }
    }

    /// Returns the total number of whole microseconds contained by this `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::new(5, 730_023_852);
    /// assert_eq!(duration.as_micros(), Some(5_730_023));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_micros(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_micros()),
            None => None,
        }
    }

    /// Returns the total number of nanoseconds contained by this `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let duration = Duration::new(5, 730_023_852);
    /// assert_eq!(duration.as_nanos(), Some(5_730_023_852));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_nanos(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_nanos()),
            None => None,
        }
    }

    // TODO: duration_abs_diff https://github.com/rust-lang/rust/issues/117618 / stabilized in 1.81 https://github.com/rust-lang/rust/pull/127128
    // /// Computes the absolute difference between `self` and `other`.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use easytime::Duration;
    // ///
    // /// assert_eq!(Duration::new(100, 0).abs_diff(Duration::new(80, 0)), Duration::new(20, 0));
    // /// assert_eq!(
    // ///     Duration::new(100, 400_000_000).abs_diff(Duration::new(110, 0)),
    // ///     Duration::new(9, 600_000_000)
    // /// );
    // /// ```
    // #[inline]
    // #[must_use]
    // pub const fn abs_diff(self, other: Duration) -> Duration {
    //     if let Some(res) = self.checked_sub(other) { res } else { other.checked_sub(self).unwrap() }
    // }

    // TODO: saturating_{add,sub,mul}?

    // TODO: duration_consts_float stabilized in 1.83 https://github.com/rust-lang/rust/pull/131289
    #[allow(clippy::manual_map)]
    /// Returns the number of seconds contained by this `Duration` as `f64`.
    ///
    /// The returned value includes the fractional (nanosecond) part of the duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.as_secs_f64(), Some(2.7));
    /// ```
    #[inline]
    #[must_use]
    pub fn as_secs_f64(&self) -> Option<f64> {
        match &self.0 {
            Some(x) => Some(x.as_secs_f64()),
            None => None,
        }
    }

    // TODO: duration_consts_float stabilized in 1.83 https://github.com/rust-lang/rust/pull/131289
    #[allow(clippy::manual_map)]
    /// Returns the number of seconds contained by this `Duration` as `f32`.
    ///
    /// The returned value includes the fractional (nanosecond) part of the duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.as_secs_f32(), Some(2.7));
    /// ```
    #[inline]
    #[must_use]
    pub fn as_secs_f32(&self) -> Option<f32> {
        match &self.0 {
            Some(x) => Some(x.as_secs_f32()),
            None => None,
        }
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let res = Duration::from_secs_f64(0.0);
    /// assert_eq!(res, Duration::new(0, 0));
    /// let res = Duration::from_secs_f64(1e-20);
    /// assert_eq!(res, Duration::new(0, 0));
    /// let res = Duration::from_secs_f64(4.2e-7);
    /// assert_eq!(res, Duration::new(0, 420));
    /// let res = Duration::from_secs_f64(2.7);
    /// assert_eq!(res, Duration::new(2, 700_000_000));
    /// let res = Duration::from_secs_f64(3e10);
    /// assert_eq!(res, Duration::new(30_000_000_000, 0));
    /// // subnormal float
    /// let res = Duration::from_secs_f64(f64::from_bits(1));
    /// assert_eq!(res, Duration::new(0, 0));
    /// // conversion uses rounding
    /// let res = Duration::from_secs_f64(0.999e-9);
    /// assert_eq!(res, Duration::new(0, 1));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_secs_f64(secs: f64) -> Self {
        Self(Self::try_from_secs_f64(secs))
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let res = Duration::from_secs_f32(0.0);
    /// assert_eq!(res, Duration::new(0, 0));
    /// let res = Duration::from_secs_f32(1e-20);
    /// assert_eq!(res, Duration::new(0, 0));
    /// let res = Duration::from_secs_f32(4.2e-7);
    /// assert_eq!(res, Duration::new(0, 420));
    /// let res = Duration::from_secs_f32(2.7);
    /// assert_eq!(res, Duration::new(2, 700_000_048));
    /// let res = Duration::from_secs_f32(3e10);
    /// assert_eq!(res, Duration::new(30_000_001_024, 0));
    /// // subnormal float
    /// let res = Duration::from_secs_f32(f32::from_bits(1));
    /// assert_eq!(res, Duration::new(0, 0));
    /// // conversion uses rounding
    /// let res = Duration::from_secs_f32(0.999e-9);
    /// assert_eq!(res, Duration::new(0, 1));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_secs_f32(secs: f32) -> Self {
        Self(Self::try_from_secs_f32(secs))
    }

    /// Multiplies `Duration` by `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
    /// assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
    /// ```
    #[inline]
    #[must_use]
    pub fn mul_f64(self, rhs: f64) -> Self {
        self.as_secs_f64().map_or(Self::NONE, |secs| Self::from_secs_f64(rhs * secs))
    }

    /// Multiplies `Duration` by `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.mul_f32(3.14), Duration::new(8, 478_000_641));
    /// assert_eq!(dur.mul_f32(3.14e5), Duration::new(847_800, 0));
    /// ```
    #[inline]
    #[must_use]
    pub fn mul_f32(self, rhs: f32) -> Self {
        self.as_secs_f32().map_or(Self::NONE, |secs| Self::from_secs_f32(rhs * secs))
    }

    /// Divides `Duration` by `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
    /// assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_599));
    /// ```
    #[inline]
    #[must_use]
    pub fn div_f64(self, rhs: f64) -> Self {
        self.as_secs_f64().map_or(Self::NONE, |secs| Self::from_secs_f64(secs / rhs))
    }

    /// Divides `Duration` by `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// // note that due to rounding errors result is slightly
    /// // different from 0.859_872_611
    /// assert_eq!(dur.div_f32(3.14), Duration::new(0, 859_872_580));
    /// assert_eq!(dur.div_f32(3.14e5), Duration::new(0, 8_599));
    /// ```
    #[inline]
    #[must_use]
    pub fn div_f32(self, rhs: f32) -> Self {
        self.as_secs_f32().map_or(Self::NONE, |secs| Self::from_secs_f32(secs / rhs))
    }

    // TODO: div_duration https://github.com/rust-lang/rust/issues/63139 / stabilized in 1.80 https://github.com/rust-lang/rust/pull/124667
    // TODO: duration_consts_float stabilized in 1.83 https://github.com/rust-lang/rust/pull/131289
    // /// Divides `Duration` by `Duration` and returns `f64`.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use easytime::Duration;
    // ///
    // /// let dur1 = Duration::new(2, 700_000_000);
    // /// let dur2 = Duration::new(5, 400_000_000);
    // /// assert_eq!(dur1.div_duration_f64(dur2), 0.5);
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn div_duration_f64(self, rhs: Self) -> f64 {
    //     let self_nanos =
    //         (self.secs as f64) * (NANOS_PER_SEC as f64) + (self.nanos.as_inner() as f64);
    //     let rhs_nanos = (rhs.secs as f64) * (NANOS_PER_SEC as f64) + (rhs.nanos.as_inner() as f64);
    //     self_nanos / rhs_nanos
    // }

    // TODO: div_duration https://github.com/rust-lang/rust/issues/63139 / stabilized in 1.80 https://github.com/rust-lang/rust/pull/124667
    // TODO: duration_consts_float stabilized in 1.83 https://github.com/rust-lang/rust/pull/131289
    // /// Divides `Duration` by `Duration` and returns `f32`.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use easytime::Duration;
    // ///
    // /// let dur1 = Duration::new(2, 700_000_000);
    // /// let dur2 = Duration::new(5, 400_000_000);
    // /// assert_eq!(dur1.div_duration_f32(dur2), 0.5);
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn div_duration_f32(self, rhs: Self) -> f32 {
    //     let self_nanos =
    //         (self.secs as f32) * (NANOS_PER_SEC as f32) + (self.nanos.as_inner() as f32);
    //     let rhs_nanos = (rhs.secs as f32) * (NANOS_PER_SEC as f32) + (rhs.nanos.as_inner() as f32);
    //     self_nanos / rhs_nanos
    // }

    // -------------------------------------------------------------------------
    // Option based method implementations

    /// Returns `true` if [`into_inner`] returns `Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let zero = Duration::new(0, 0);
    /// let one_sec = Duration::new(1, 0);
    /// assert!((one_sec - zero).is_some());
    /// assert!(!(zero - one_sec).is_some());
    /// ```
    ///
    /// [`into_inner`]: Self::into_inner
    #[inline]
    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Returns `true` if [`into_inner`] returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let zero = Duration::new(0, 0);
    /// let one_sec = Duration::new(1, 0);
    /// assert!(!(one_sec - zero).is_none());
    /// assert!((zero - one_sec).is_none());
    /// ```
    ///
    /// [`into_inner`]: Self::into_inner
    #[inline]
    #[must_use]
    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Returns the contained [`std::time::Duration`] or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let zero = Duration::new(0, 0);
    /// let one_sec = Duration::new(1, 0);
    /// assert_eq!((one_sec - zero).into_inner(), Some(std::time::Duration::from_secs(1)));
    /// assert_eq!((zero - one_sec).into_inner(), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> Option<time::Duration> {
        self.0
    }

    /// Returns the contained [`std::time::Duration`] or a default.
    ///
    /// `dur.unwrap_or(default)` is equivalent to `dur.into_inner().unwrap_or(default)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let zero = Duration::new(0, 0);
    /// let one_sec = Duration::new(1, 0);
    /// assert_eq!(
    ///     (one_sec - zero).unwrap_or(std::time::Duration::from_secs(2)),
    ///     std::time::Duration::from_secs(1)
    /// );
    /// assert_eq!(
    ///     (zero - one_sec).unwrap_or(std::time::Duration::from_secs(2)),
    ///     std::time::Duration::from_secs(2)
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn unwrap_or(self, default: time::Duration) -> time::Duration {
        match self.0 {
            Some(d) => d,
            None => default,
        }
    }

    /// Returns the contained [`std::time::Duration`] or computes it from a closure.
    ///
    /// `dur.unwrap_or_else(default)` is equivalent to `dur.into_inner().unwrap_or_else(default)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let zero = Duration::new(0, 0);
    /// let one_sec = Duration::new(1, 0);
    /// assert_eq!(
    ///     (one_sec - zero).unwrap_or_else(|| std::time::Duration::from_secs(2)),
    ///     std::time::Duration::from_secs(1)
    /// );
    /// assert_eq!(
    ///     (zero - one_sec).unwrap_or_else(|| std::time::Duration::from_secs(2)),
    ///     std::time::Duration::from_secs(2)
    /// );
    /// ```
    #[inline]
    pub fn unwrap_or_else<F>(self, default: F) -> time::Duration
    where
        F: FnOnce() -> time::Duration,
    {
        self.0.unwrap_or_else(default)
    }
}

// -----------------------------------------------------------------------------
// Trait implementations

impl PartialEq<time::Duration> for Duration {
    fn eq(&self, other: &time::Duration) -> bool {
        self.0 == Some(*other)
    }
}

impl PartialEq<Duration> for time::Duration {
    fn eq(&self, other: &Duration) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<time::Duration> for Duration {
    fn partial_cmp(&self, other: &time::Duration) -> Option<cmp::Ordering> {
        self.0.as_ref().and_then(|this| this.partial_cmp(other))
    }
}

impl PartialOrd<Duration> for time::Duration {
    fn partial_cmp(&self, other: &Duration) -> Option<cmp::Ordering> {
        other.0.as_ref().and_then(|other| self.partial_cmp(other))
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

impl From<Option<time::Duration>> for Duration {
    fn from(dur: Option<time::Duration>) -> Self {
        Self(dur)
    }
}

impl TryFrom<Duration> for time::Duration {
    type Error = TryFromTimeError;

    fn try_from(dur: Duration) -> Result<Self, Self::Error> {
        dur.into_inner().ok_or(TryFromTimeError(()))
    }
}

macro_rules! try_from_secs {
    (
        secs = $secs: expr,
        mantissa_bits = $mant_bits: literal,
        exponent_bits = $exp_bits: literal,
        offset = $offset: literal,
        bits_ty = $bits_ty:ty,
        double_ty = $double_ty:ty,
    ) => {{
        const MIN_EXP: i16 = 1 - (1i16 << $exp_bits) / 2;
        const MANT_MASK: $bits_ty = (1 << $mant_bits) - 1;
        const EXP_MASK: $bits_ty = (1 << $exp_bits) - 1;

        if $secs < 0.0 {
            return None;
        }

        let bits = $secs.to_bits();
        let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
        let exp = ((bits >> $mant_bits) & EXP_MASK) as i16 + MIN_EXP;

        let (secs, nanos) = if exp < -31 {
            // the input represents less than 1ns and can not be rounded to it
            (0u64, 0u32)
        } else if exp < 0 {
            // the input is less than 1 second
            let t = <$double_ty>::from(mant) << ($offset + exp);
            let nanos_offset = $mant_bits + $offset;
            let nanos_tmp = u128::from(NANOS_PER_SEC) * u128::from(t);
            let nanos = (nanos_tmp >> nanos_offset) as u32;

            let rem_mask = (1 << nanos_offset) - 1;
            let rem_msb_mask = 1 << (nanos_offset - 1);
            let rem = nanos_tmp & rem_mask;
            let is_tie = rem == rem_msb_mask;
            let is_even = (nanos & 1) == 0;
            let rem_msb = nanos_tmp & rem_msb_mask == 0;
            let add_ns = !(rem_msb || (is_even && is_tie));

            // f32 does not have enough precision to trigger the second branch
            // since it can not represent numbers between 0.999_999_940_395 and 1.0.
            let nanos = nanos + add_ns as u32;
            if ($mant_bits == 23) || (nanos != NANOS_PER_SEC) { (0, nanos) } else { (1, 0) }
        } else if exp < $mant_bits {
            let secs = u64::from(mant >> ($mant_bits - exp));
            let t = <$double_ty>::from((mant << exp) & MANT_MASK);
            let nanos_offset = $mant_bits;
            let nanos_tmp = <$double_ty>::from(NANOS_PER_SEC) * t;
            let nanos = (nanos_tmp >> nanos_offset) as u32;

            let rem_mask = (1 << nanos_offset) - 1;
            let rem_msb_mask = 1 << (nanos_offset - 1);
            let rem = nanos_tmp & rem_mask;
            let is_tie = rem == rem_msb_mask;
            let is_even = (nanos & 1) == 0;
            let rem_msb = nanos_tmp & rem_msb_mask == 0;
            let add_ns = !(rem_msb || (is_even && is_tie));

            // f32 does not have enough precision to trigger the second branch.
            // For example, it can not represent numbers between 1.999_999_880...
            // and 2.0. Bigger values result in even smaller precision of the
            // fractional part.
            let nanos = nanos + add_ns as u32;
            if ($mant_bits == 23) || (nanos != NANOS_PER_SEC) {
                (secs, nanos)
            } else {
                (secs + 1, 0)
            }
        } else if exp < 64 {
            // the input has no fractional part
            let secs = u64::from(mant) << (exp - $mant_bits);
            (secs, 0)
        } else {
            return None;
        };

        Some(time::Duration::new(secs, nanos))
    }};
}

impl Duration {
    #[inline]
    fn try_from_secs_f32(secs: f32) -> Option<time::Duration> {
        try_from_secs!(
            secs = secs,
            mantissa_bits = 23,
            exponent_bits = 8,
            offset = 41,
            bits_ty = u32,
            double_ty = u64,
        )
    }

    #[inline]
    fn try_from_secs_f64(secs: f64) -> Option<time::Duration> {
        try_from_secs!(
            secs = secs,
            mantissa_bits = 52,
            exponent_bits = 11,
            offset = 44,
            bits_ty = u64,
            double_ty = u128,
        )
    }
}
