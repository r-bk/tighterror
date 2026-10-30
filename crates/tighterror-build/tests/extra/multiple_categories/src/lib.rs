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
        // category bits above two variant bits, in a u8. Every category
        // takes a third variant for the unused variant 0. It pins the values
        // that the generated code holds for this spec.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 2);
            assert_eq!(_p::VAR_BITS, 2);
            assert_eq!(_p::KIND_BITS, 4);
            assert_eq!(_p::VAR_MASK, 0b0011);
            assert_eq!(_p::CAT_MASK, 0b1100);

            // the kind values, in the underlying type
            let value: u8 = kind::parsing::BAD_TOKEN.value();
            assert_eq!(value, 0b0001);
            assert_eq!(kind::processing::QUEUE_FULL.value(), 0b0110);
            assert_eq!(kind::general::TIMEOUT.value(), 0b1010);

            // the variant 0 of every category is rejected, the value 0
            // included
            assert_eq!(McErrorKind::from_value(0), None);
            assert_eq!(McErrorKind::from_value(0b0100), None);
            assert_eq!(McErrorKind::from_value(0b1000), None);

            // the unused variant 3 and the unused fourth category are rejected
            assert_eq!(McErrorKind::from_value(0b0111), None);
            assert_eq!(McErrorKind::from_value(0b1101), None);

            // a bit above the kind bits is rejected, rather than dropped
            assert_eq!(McErrorKind::from_value(0b10001), None);
            assert_eq!(McErrorKind::from_value(0b11010), None);
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
