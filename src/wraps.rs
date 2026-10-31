use crate::Kind;

/// The trait of error kinds that wrap the error kinds `K` of another crate in
/// one of their categories.
///
/// The code of a module that wraps the errors of another crate implements it
/// for every wrapped kind type, so that the type of a wrapped kind names its
/// category.
pub trait Wraps<K: Kind>: Kind {
    /// The category that holds the wrapped error kinds.
    const CATEGORY: Self::Category;
}
