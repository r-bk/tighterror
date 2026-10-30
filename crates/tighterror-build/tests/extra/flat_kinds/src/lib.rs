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
        // category bit above one variant bit, in a u8. It pins the values
        // that the generated code holds for this spec.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 1);
            assert_eq!(_p::KIND_BITS, 2);
            assert_eq!(_p::VAR_MASK, 0b01);
            assert_eq!(_p::CAT_MASK, 0b10);

            // the kind values, in the underlying type
            let value: u8 = kind::TIMEOUT.value();
            assert_eq!(value, 0b00);
            assert_eq!(kind::QUEUE_FULL.value(), 0b01);
            assert_eq!(kind::CONNECTION_REFUSED.value(), 0b10);

            // the unused variant of the second category is rejected
            assert_eq!(ErrorKind::from_value(0b11), None);
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
