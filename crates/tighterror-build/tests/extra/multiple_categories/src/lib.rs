//! This is a test crate to check `tighterror.kdl`
//! with multiple categories.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);
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
