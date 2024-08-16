//! This is a test crate to check `tighterror.yaml`
//! with implicit category.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kinds_are_under_kinds_module() {
        assert_ne!(errors::kind::TIMEOUT, errors::kind::CONNECTION_REFUSED);
    }
}
