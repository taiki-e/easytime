use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time,
};

use super::{pair_and_then, Duration};

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(pub(crate) Option<time::Instant>);

impl Instant {
    /// Returns an instant corresponding to "now".
    pub fn now() -> Instant {
        Instant(Some(time::Instant::now()))
    }

    /// Returns `true` if [`into_inner`] returns `Some`.
    ///
    /// [`into_inner`]: #method.into_inner
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Returns `true` if [`into_inner`] returns `None`.
    ///
    /// [`into_inner`]: #method.into_inner
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the contained [`std::time::Instant`] or `None`.
    ///
    /// [`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
    #[inline]
    pub const fn into_inner(self) -> Option<time::Instant> {
        self.0
    }

    /// Returns the contained [`std::time::Instant`] or a default.
    ///
    /// `instant.unwrap_or(f)` is equivalent to `instant.into_inner().unwrap_or(default)`.
    ///
    /// [`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
    #[inline]
    pub fn unwrap_or(self, default: time::Instant) -> time::Instant {
        self.0.unwrap_or(default)
    }

    /// Returns the contained [`std::time::Instant`] or computes it from a closure.
    ///
    /// `instant.unwrap_or_else(default)` is equivalent to `instant.into_inner().unwrap_or_else(default)`.
    ///
    /// [`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
    #[inline]
    pub fn unwrap_or_else<F>(self, default: F) -> time::Instant
    where
        F: FnOnce() -> time::Instant,
    {
        self.0.unwrap_or_else(default)
    }

    /// Returns the amount of time elapsed from another instant to this one.
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration(pair_and_then(
            self.0.as_ref(),
            earlier.0,
            |this, earlier| {
                // https://github.com/rust-lang/rust/pull/58395
                if *this >= earlier {
                    Some(this.duration_since(earlier))
                } else {
                    None
                }
            },
        ))
    }

    /// Returns the amount of time elapsed since this instant was created.
    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
    }
}

impl From<time::Instant> for Instant {
    fn from(dur: time::Instant) -> Instant {
        Instant(Some(dur))
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, other: Duration) -> Instant {
        Instant(pair_and_then(self.0, other.0, |this, other| {
            this.checked_add(other)
        }))
    }
}

impl Add<time::Duration> for Instant {
    type Output = Instant;
    fn add(self, other: time::Duration) -> Instant {
        Instant(self.0.and_then(|this| this.checked_add(other)))
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
    type Output = Instant;
    fn sub(self, other: Duration) -> Instant {
        Instant(pair_and_then(self.0, other.0, |this, other| {
            this.checked_sub(other)
        }))
    }
}

impl Sub<time::Duration> for Instant {
    type Output = Instant;
    fn sub(self, other: time::Duration) -> Instant {
        Instant(self.0.and_then(|this| this.checked_sub(other)))
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
    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

impl Sub<time::Instant> for Instant {
    type Output = Duration;
    fn sub(self, other: time::Instant) -> Duration {
        self.duration_since(Instant::from(other))
    }
}
