use crate::TightErrorCategory;
use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt::{Debug, Display},
    hash::Hash,
};

/// The interface of error kinds.
pub trait TightErrorKind:
    Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Debug + Display + Hash
{
    /// The total number of bits required for the error kind.
    ///
    /// This includes both category bits and variant bits.
    const BITS: usize;

    /// The underlying Rust type of the error kind.
    ///
    /// A concrete builtin type, e.g., `u8`.
    type ReprType;

    /// The error category concrete type.
    type CategoryType: TightErrorCategory<ReprType = Self::ReprType>;

    /// Returns the error category.
    fn category(&self) -> Self::CategoryType;

    /// Returns the error kind name.
    fn name(&self) -> &'static str;

    /// Returns the error kind numerical value as the underlying Rust type.
    ///
    /// This function is intended for embedding one instantiation of `TightError`
    /// as a single category within another instantiation of `TightError`.
    /// Persisting the raw values, and/or using them between different invocations
    /// of a program (possibly compiled with another version of error's origin crate)
    /// may lead to bugs because the mapping between an error kind and its
    /// underlying raw value may change.
    fn value(&self) -> Self::ReprType;

    /// Creates an error kind from a value of the underlying Rust type.
    ///
    /// The function returns `None` if `value` doesn't denote a valid error
    /// kind **in its current definition**.
    ///
    /// This function is intended for embedding one instantiation of `TightError`
    /// as a single category within another instantiation of `TightError`.
    /// Persisting the raw values, and/or using them between different invocations
    /// of a program (possibly compiled with another version of error's origin crate)
    /// may lead to bugs because the mapping between an error kind and its
    /// underlying raw value may change.
    fn from_value(value: Self::ReprType) -> Option<Self>;
}