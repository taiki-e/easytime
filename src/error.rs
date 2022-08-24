use core::fmt;

/// The error type returned when a conversion from `easytime` types to `std::time` types fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryFromTimeError(pub(crate) ());

impl fmt::Display for TryFromTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid arithmetic attempted on instants or durations")
    }
}

#[allow(clippy::std_instead_of_core)] // core::error::Error requires 1.65
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for TryFromTimeError {}
