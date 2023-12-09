//! This is a test crate to check `tighterror.yaml`
//! with implicit category.

#![deny(missing_docs)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}
