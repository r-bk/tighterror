use crate::{Category, Kind};
use core::fmt::{Debug, Display};

/// The trait of variant types.
pub trait VariantType: Clone + Debug + Display {
    /// The underlying Rust type of error kind.
    ///
    /// A concrete builtin type, e.g., `u8`.
    type R;

    /// The error category concrete type.
    type Category: Category<R = Self::R>;

    /// The error kind concrete type.
    type Kind: Kind<R = Self::R, Category = Self::Category>;

    /// The error category of the variant type.
    const CATEGORY: Self::Category;

    /// The error kind of the variant type.
    const KIND: Self::Kind;

    /// The concrete type name.
    const NAME: &'static str;

    /// Returns the error category of the variant type.
    ///
    /// This is a convenience method that returns [CATEGORY](Self::CATEGORY).
    #[inline]
    fn category(&self) -> Self::Category {
        Self::CATEGORY
    }

    /// Returns the error kind of the variant type.
    ///
    /// This is a convenience method that returns [KIND](Self::KIND).
    #[inline]
    fn kind(&self) -> Self::Kind {
        Self::KIND
    }

    /// Returns the variant type name.
    ///
    /// This is a convenience method that returns [NAME](Self::NAME).
    #[inline]
    fn name(&self) -> &'static str {
        Self::NAME
    }
}
