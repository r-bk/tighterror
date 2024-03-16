use crate::TightErrorCategory;
use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// The trait of error kinds.
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
    type R;

    /// The error category concrete type.
    type Category: TightErrorCategory<R = Self::R>;

    /// Returns the error category.
    fn category(&self) -> Self::Category;

    /// Returns the error kind name.
    fn name(&self) -> &'static str;

    /// Returns the error kind numerical value as the underlying Rust type.
    ///
    /// This function allows embedding one instantiation of `TightError`
    /// as a single category within another instantiation of `TightError`.
    ///
    /// Persisting the raw values, and/or using them between different invocations
    /// of a program (possibly compiled with another version of error's origin crate)
    /// may lead to bugs because the mapping between an error kind and its
    /// underlying raw value may change.
    fn value(&self) -> Self::R;

    /// Creates an error kind from a value of the underlying Rust type.
    ///
    /// The function returns `None` if `value` doesn't denote a valid error
    /// kind **in its current definition**.
    ///
    /// This function allows embedding one instantiation of `TightError`
    /// as a single category within another instantiation of `TightError`.
    ///
    /// Persisting the raw values, and/or using them between different invocations
    /// of a program (possibly compiled with another version of error's origin crate)
    /// may lead to bugs because the mapping between an error kind and its
    /// underlying raw value may change.
    fn from_value(value: Self::R) -> Option<Self>;
}
