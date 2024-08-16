//! This is a test crate to check `tighterror.yaml`
//! with multiple categories.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use crate::errors;

    #[test]
    fn test_kind_constants_are_placed_in_different_modules() {
        assert_ne!(
            errors::kind::parsing::QUEUE_FULL,
            errors::kind::processing::QUEUE_FULL
        );
    }
}
