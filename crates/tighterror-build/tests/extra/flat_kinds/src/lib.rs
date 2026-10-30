//! This is a test crate to check `tighterror.kdl`
//! with flat kinds.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod layout_tests {
        use super::*;

        // Checks the layout of an error kind with two categories: one
        // category bit above two variant bits, in a u8. Every category takes
        // one variant more for the unused variant 0, which gives the first
        // one a third variant. It pins the values that the generated code
        // holds for this spec.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 2);
            assert_eq!(_p::KIND_BITS, 3);
            assert_eq!(_p::VAR_MASK, 0b011);
            assert_eq!(_p::CAT_MASK, 0b100);

            // the kind values, in the underlying type
            let value: u8 = kind::TIMEOUT.value();
            assert_eq!(value, 0b001);
            assert_eq!(kind::QUEUE_FULL.value(), 0b010);
            assert_eq!(kind::CONNECTION_REFUSED.value(), 0b101);

            // the variant 0 of every category is rejected, the value 0
            // included
            assert_eq!(ErrorKind::from_value(0), None);
            assert_eq!(ErrorKind::from_value(0b100), None);

            // the unused variants past the last one are rejected
            assert_eq!(ErrorKind::from_value(0b011), None);
            assert_eq!(ErrorKind::from_value(0b110), None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kinds_are_under_kinds_module() {
        assert_ne!(errors::kind::TIMEOUT, errors::kind::CONNECTION_REFUSED);
    }
}
