// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use core::{
    cmp, fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    time,
};

use crate::{utils::pair_and_then, TryFromTimeError};

const NANOS_PER_SEC: u32 = 1_000_000_000;

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
///
/// [`ops`]: std::ops
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub(crate) Option<time::Duration>);

impl Duration {
    // TODO: add the followings once stabilized:
    // - duration_constants https://github.com/rust-lang/rust/issues/57391
    // - duration_constructors https://github.com/rust-lang/rust/issues/120301
    // - duration_millis_float https://github.com/rust-lang/rust/issues/122451

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

    /// Returns `true` if this `Duration` spans no time.
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
    //     if let Some(res) = self.checked_sub(other) {
    //         res
    //     } else {
    //         other.checked_sub(self).unwrap()
    //     }
    // }

    // TODO: duration_consts_float stabilized in 1.83 https://github.com/rust-lang/rust/pull/131289
    /// Returns the number of seconds contained by this `Duration` as `f64`.
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
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
        self.0.as_ref().map(time::Duration::as_secs_f64)
    }

    // TODO: duration_consts_float stabilized in 1.83 https://github.com/rust-lang/rust/pull/131289
    /// Returns the number of seconds contained by this `Duration` as `f32`.
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
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
        self.0.as_ref().map(time::Duration::as_secs_f32)
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::from_secs_f64(2.7);
    /// assert_eq!(dur, Duration::new(2, 700_000_000));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_secs_f64(secs: f64) -> Self {
        // TODO: update implementation based on https://github.com/rust-lang/rust/commit/e0bcf771d6e670988a3d4fdc785ecd5857916f10
        const MAX_NANOS_F64: f64 = ((u64::MAX as u128 + 1) * (NANOS_PER_SEC as u128)) as f64;
        let nanos = secs * (NANOS_PER_SEC as f64);
        if !nanos.is_finite() || nanos >= MAX_NANOS_F64 || nanos < 0. {
            return Self(None);
        }
        let nanos = nanos as u128;
        Self::new(
            (nanos / (NANOS_PER_SEC as u128)) as u64,
            (nanos % (NANOS_PER_SEC as u128)) as u32,
        )
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::from_secs_f32(2.7);
    /// assert_eq!(dur, Duration::new(2, 700_000_000));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_secs_f32(secs: f32) -> Duration {
        // TODO: update implementation based on https://github.com/rust-lang/rust/commit/e0bcf771d6e670988a3d4fdc785ecd5857916f10
        const MAX_NANOS_F32: f32 = ((u64::MAX as u128 + 1) * (NANOS_PER_SEC as u128)) as f32;
        let nanos = secs * (NANOS_PER_SEC as f32);
        if !nanos.is_finite() || nanos >= MAX_NANOS_F32 || nanos < 0. {
            return Self(None);
        }
        let nanos = nanos as u128;
        Self::new(
            (nanos / (NANOS_PER_SEC as u128)) as u64,
            (nanos % (NANOS_PER_SEC as u128)) as u32,
        )
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
    pub fn mul_f64(self, rhs: f64) -> Duration {
        self.as_secs_f64().map_or(Self::NONE, |secs| Duration::from_secs_f64(rhs * secs))
    }

    /// Multiplies `Duration` by `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// // note that due to rounding errors result is slightly different
    /// // from 8.478 and 847800.0
    /// assert_eq!(dur.mul_f32(3.14), Duration::new(8, 478_000_640));
    /// assert_eq!(dur.mul_f32(3.14e5), Duration::new(847799, 969_120_256));
    /// ```
    #[inline]
    #[must_use]
    pub fn mul_f32(self, rhs: f32) -> Duration {
        self.as_secs_f32().map_or(Self::NONE, |secs| Duration::from_secs_f32(rhs * secs))
    }

    /// Divide `Duration` by `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
    /// // note that truncation is used, not rounding
    /// assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_598));
    /// ```
    #[inline]
    #[must_use]
    pub fn div_f64(self, rhs: f64) -> Duration {
        self.as_secs_f64().map_or(Self::NONE, |secs| Duration::from_secs_f64(secs / rhs))
    }

    /// Divide `Duration` by `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use easytime::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
    /// // note that truncation is used, not rounding
    /// assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_598));
    /// ```
    #[inline]
    #[must_use]
    pub fn div_f32(self, rhs: f32) -> Duration {
        self.as_secs_f32().map_or(Self::NONE, |secs| Duration::from_secs_f32(secs / rhs))
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
    // pub fn div_duration_f64(self, rhs: Duration) -> f64 {
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
    // pub fn div_duration_f32(self, rhs: Duration) -> f32 {
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
