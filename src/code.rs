use crate::TightErrorCategory;
use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt::{Debug, Display},
    hash::Hash,
};

/// The interface of error codes.
pub trait TightErrorCode:
    Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Debug + Display + Hash
{
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

    /// The error category concrete type.
    type CategoryType: TightErrorCategory<ReprType = Self::ReprType>;

    /// Returns the error code category.
    fn category(&self) -> Self::CategoryType;

    /// Returns the error code name.
    fn name(&self) -> &'static str;

    /// Returns the error code numerical value as the underlying Rust type.
    ///
    /// This function is intended for embdeding one instantiation of `TightError`
    /// as a single category within another instantiation of `TightError`.
    /// Persisting the raw values, and/or using them between different invocations
    /// of a program (possibly compiled with another version of error's origin crate)
    /// may lead to bugs because the mapping between an error code and its
    /// underlying raw value may change.
    fn value(&self) -> Self::ReprType;

    /// Creates an error code from a value of the underlying Rust type.
    ///
    /// The function returns `None` if `value` doesn't denote a valid error
    /// code **in its current definition**.
    ///
    /// This function is intended for embdeding one instantiation of `TightError`
    /// as a single category within another instantiation of `TightError`.
    /// Persisting the raw values, and/or using them between different invocations
    /// of a program (possibly compiled with another version of error's origin crate)
    /// may lead to bugs because the mapping between an error code and its
    /// underlying raw value may change.
    fn from_value(value: Self::ReprType) -> Option<Self>;
}
