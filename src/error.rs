use crate::{Category, Kind, Location};
use core::fmt::{Debug, Display};

/// The trait of error types.
///
/// See the crate documentation for more information.
pub trait Error: Debug + Display {
    /// The underlying Rust type of error kind.
    ///
    /// A concrete builtin type, e.g., `u8`.
    type R;

    /// The error category concrete type.
    type Category: Category<R = Self::R>;

    /// The error kind concrete type.
    type Kind: Kind<R = Self::R, Category = Self::Category>;

    /// Returns the error kind.
    ///
    /// The error kind is unique per `tighterror::Error` instantiation.
    fn kind(&self) -> Self::Kind;

    /// Returns the error category.
    ///
    /// This method is a shorthand for `self.kind().category()`.
    #[inline]
    fn category(&self) -> Self::Category {
        self.kind().category()
    }

    /// Returns the error's source location.
    ///
    /// By default an *undefined* location is returned, unless the
    /// concrete error type supports it and the error instance is initialized
    /// with it.
    fn location(&self) -> Location {
        Location::undefined()
    }
}
