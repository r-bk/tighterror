//! This is a test crate to check `tighterror.kdl`
//! with multiple categories.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod layout_tests {
        use super::*;

        // Checks the layout of an error kind with three categories: two
        // category bits above one variant bit, in a u8. It pins the values
        // that the generated code holds for this spec.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 2);
            assert_eq!(_p::VAR_BITS, 1);
            assert_eq!(_p::KIND_BITS, 3);
            assert_eq!(_p::VAR_MASK, 0b001);
            assert_eq!(_p::CAT_MASK, 0b110);

            // the kind values, in the underlying type
            let value: u8 = kind::parsing::BAD_TOKEN.value();
            assert_eq!(value, 0b000);
            assert_eq!(kind::processing::QUEUE_FULL.value(), 0b011);
            assert_eq!(kind::general::TIMEOUT.value(), 0b101);

            // the unused fourth category is rejected
            assert_eq!(McErrorKind::from_value(0b110), None);

            // a bit above the kind bits is rejected, rather than dropped
            assert_eq!(McErrorKind::from_value(0b1000), None);
            assert_eq!(McErrorKind::from_value(0b1101), None);
        }
    }
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
