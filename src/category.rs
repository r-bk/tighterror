use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// The trait of error categories.
pub trait Category:
    Eq + PartialEq + Ord + PartialOrd + Debug + Display + Copy + Clone + Hash
{
    /// The number of bits required for the error category.
    const BITS: usize;

    /// The underlying Rust type of the error kind.
    ///
    /// A concrete builtin type, e.g., `u8`.
    type R;

    /// Returns the category name.
    fn name(&self) -> &'static str;
}
