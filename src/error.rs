use crate::{Location, TightErrorCode};
use core::fmt::{Debug, Display};

/// The interface of error types.
///
/// See the crate documentation for more information.
pub trait TightError: Debug + Display {
    /// The number of bits required for an error category.
    const CATEGORY_BITS: usize;

    /// The number of bits required for an error kind.
    const KIND_BITS: usize;

    /// The number of categories.
    const CATEGORIES_COUNT: usize;

    /// The underlying Rust type of an error code.
    ///
    /// A concrete builtin type, e.g. `u8`.
    type ReprType;

    /// The error code concrete type.
    type CodeType: TightErrorCode<ReprType = Self::ReprType>;

    /// Returns the error code.
    ///
    /// The error code is unique per `TightError` instantiation.
    fn code(&self) -> Self::CodeType;

    /// Returns the error's source location.
    ///
    /// By default an *undefined* location is returned, unless the
    /// concrete error type supports it and the error instance is initialized
    /// with it.
    fn location(&self) -> Location {
        Location::undefined()
    }
}
