//! This is a test crate to check `tighterror.toml`
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
            errors::kind::parsing::QUEUE_FULL,
            errors::kind::processing::QUEUE_FULL
        );
        assert_ne!(
            internal_errors::kind::parser::BAD_FILE,
            internal_errors::kind::processor::BAD_FILE,
        );
    }
}
