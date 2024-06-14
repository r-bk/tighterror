//! This is a test crate to check `tighterror.yaml`
//! with multiple modules.

#![deny(missing_docs)]
#![deny(warnings)]

include!(concat!(env!("OUT_DIR"), "/errors.rs"));

#[cfg(test)]
mod tests {
    use crate::{errors, internal_errors};

    #[test]
    fn test_kind_constants_are_placed_in_different_modules() {
        assert_ne!(
            errors::kinds::parsing::QUEUE_FULL,
            errors::kinds::processing::QUEUE_FULL
        );
        assert_ne!(
            internal_errors::kinds::parser::BAD_FILE,
            internal_errors::kinds::processor::BAD_FILE,
        );
    }
}
