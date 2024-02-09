use core::fmt::Display;

const UNDEFINED_LOCATION: &str = "<undefined location>";

/// A location in source code.
///
/// This struct allows reporting an error's origin in source code.
/// It is similar to [`panic::Location`] with the following differences:
///
/// - it is `'static`
/// - it can be created as *undefined*
///
/// A `Location` is *undefined* when the file name is an empty string.
///
/// See [`TightError::location`] for more information.
///
/// [`panic::Location`]: core::panic::Location
/// [`TightError::location`]: crate::TightError::location
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Location {
    /// The file name.
    pub file: &'static str,
    /// The line number.
    pub line: u32,
}

impl Location {
    /// Returns the source location of the caller of this function.
    ///
    /// See [`panic::Location::caller`](core::panic::Location::caller) for more
    /// information.
    #[track_caller]
    pub fn caller() -> Self {
        let loc = ::core::panic::Location::caller();
        Self {
            file: loc.file(),
            line: loc.line(),
        }
    }

    /// Creates an *undefined* Location.
    ///
    /// A location is *undefined* when the file name is an empty string.
    pub fn undefined() -> Location {
        Self { file: "", line: 0 }
    }

    /// Checks if the Location is *undefined*.
    #[inline]
    pub fn is_undefined(&self) -> bool {
        self.file.is_empty()
    }
}

impl Display for Location {
    /// Formats the Location.
    ///
    /// *Undefined* Locations are displayed as `<undefined location>`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_undefined() {
            write!(f, "{}", UNDEFINED_LOCATION)
        } else {
            write!(f, "{}:{}", self.file, self.line)
        }
    }
}
