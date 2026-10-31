//! This is a test crate to check `tighterror.kdl`
//! with a module whose errors another crate wraps.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);
}
