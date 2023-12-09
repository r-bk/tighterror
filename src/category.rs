use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt::{Debug, Display},
    hash::Hash,
};

/// The interface of error categories.
pub trait TightErrorCategory:
    Eq + PartialEq + Ord + PartialOrd + Debug + Display + Copy + Clone + Hash
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

    /// Returns the category name.
    fn name(&self) -> &'static str;
}
