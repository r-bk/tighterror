use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt::{Debug, Display},
    hash::Hash,
};

/// The interface of error categories.
pub trait TightErrorCategory:
    Eq + PartialEq + Ord + PartialOrd + Debug + Display + Copy + Clone + Hash
{
    /// The number of bits required for the error category.
    const BITS: usize;

    /// The underlying Rust type of the error kind.
    ///
    /// A concrete builtin type, e.g., `u8`.
    type ReprType;

    /// Returns the category name.
    fn name(&self) -> &'static str;
}
