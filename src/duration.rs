use const_fn::const_fn;
use core::{
    convert::TryFrom,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    time,
};

use super::{pair_and_then, TryFromTimeError};

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
    // TODO: duration_constants https://github.com/rust-lang/rust/issues/57391
    // TODO: duration_zero https://github.com/rust-lang/rust/issues/73544
    // TODO: div_duration https://github.com/rust-lang/rust/issues/63139

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
    pub fn new(secs: u64, nanos: u32) -> Self {
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
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(Some(time::Duration::from_nanos(nanos)))
    }

    /// Returns the number of _whole_ seconds contained by this `Duration`.
    ///
    /// The returned value does not include the fractional (nanosecond) part of the
    /// duration, which can be obtained using [`subsec_nanos`].
    ///
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
    #[const_fn("1.46")]
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
    #[const_fn("1.46")]
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
    #[const_fn("1.46")]
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
    #[const_fn("1.46")]
    pub const fn as_nanos(&self) -> Option<u128> {
        match &self.0 {
            Some(d) => Some(d.as_nanos()),
            None => None,
        }
    }

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
    pub fn as_secs_f64(&self) -> Option<f64> {
        // TODO: replace with `self.0.as_ref().map(time::Duration::as_secs_f64)` on Rust 1.38+.
        self.0.map(|this| {
            (this.as_secs() as f64) + (this.subsec_nanos() as f64) / (NANOS_PER_SEC as f64)
        })
    }

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
    pub fn as_secs_f32(&self) -> Option<f32> {
        // TODO: replace with `self.0.as_ref().map(time::Duration::as_secs_f32)` on Rust 1.38+.
        self.0.map(|this| {
            (this.as_secs() as f32) + (this.subsec_nanos() as f32) / (NANOS_PER_SEC as f32)
        })
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
    pub fn from_secs_f64(secs: f64) -> Self {
        const MAX_NANOS_F64: f64 =
            ((u64::max_value() as u128 + 1) * (NANOS_PER_SEC as u128)) as f64;
        let nanos = secs * (NANOS_PER_SEC as f64);
        if !nanos.is_finite() || nanos >= MAX_NANOS_F64 || nanos < 0.0 {
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
    pub fn from_secs_f32(secs: f32) -> Duration {
        const MAX_NANOS_F32: f32 =
            ((u64::max_value() as u128 + 1) * (NANOS_PER_SEC as u128)) as f32;
        let nanos = secs * (NANOS_PER_SEC as f32);
        if !nanos.is_finite() || nanos >= MAX_NANOS_F32 || nanos < 0.0 {
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
    pub fn mul_f64(self, rhs: f64) -> Duration {
        self.as_secs_f64()
            .map(|secs| Duration::from_secs_f64(rhs * secs))
            .unwrap_or_else(|| Self(None))
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
    pub fn mul_f32(self, rhs: f32) -> Duration {
        self.as_secs_f32()
            .map(|secs| Duration::from_secs_f32(rhs * secs))
            .unwrap_or_else(|| Self(None))
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
    pub fn div_f64(self, rhs: f64) -> Duration {
        self.as_secs_f64()
            .map(|secs| Duration::from_secs_f64(secs / rhs))
            .unwrap_or_else(|| Self(None))
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
    pub fn div_f32(self, rhs: f32) -> Duration {
        self.as_secs_f32()
            .map(|secs| Duration::from_secs_f32(secs / rhs))
            .unwrap_or_else(|| Self(None))
    }

    // =============================================================================
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
    #[allow(clippy::redundant_pattern_matching)] // const Option::is_some requires Rust 1.48
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
    #[const_fn("1.46")]
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
