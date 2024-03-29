//! This is a test crate to check `tighterror.yaml`
//! with minimal configuration.

#![deny(warnings)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}
