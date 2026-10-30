//! This is a test crate to check `tighterror.kdl`
//! with set underlying types.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod layout_tests {
        use super::*;

        // Checks the layout of an error kind with two categories in a set
        // u32: one category bit above one variant bit, with the masks
        // computed from the 32 bits of the type rather than from the 8 bits
        // the kinds would fit in.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 1);
            assert_eq!(_p::KIND_BITS, 2);
            assert_eq!(_p::VAR_MASK, 0b01);
            assert_eq!(_p::CAT_MASK, 0b10);

            // the kind values, in the set type
            let value: u32 = kind::TIMEOUT.value();
            assert_eq!(value, 0b00);
            assert_eq!(kind::QUEUE_FULL.value(), 0b01);
            assert_eq!(kind::CONNECTION_REFUSED.value(), 0b10);

            // the error takes the size of the set type
            assert_eq!(core::mem::size_of::<Error>(), 4);

            // the unused variant of the second category is rejected
            assert_eq!(ErrorKind::from_value(0b11), None);
        }
    }
}

/// Errors with a wide underlying type.
pub mod wide_errors {
    tighterror::include_errors!(wide_errors);

    #[cfg(test)]
    mod layout_tests {
        use super::*;

        // Checks the layout of an error kind with a single category in a set
        // u64: one variant bit and no category bits, with the mask computed
        // from the 64 bits of the type.
        #[test]
        fn test_layout() {
            // the bits and the mask
            assert_eq!(_p::CAT_BITS, 0);
            assert_eq!(_p::VAR_BITS, 1);
            assert_eq!(_p::KIND_BITS, 1);
            assert_eq!(_p::VAR_MASK, 0b1);

            // the kind values, in the set type
            let value: u64 = kind::TIMEOUT.value();
            assert_eq!(value, 0);
            assert_eq!(kind::QUEUE_FULL.value(), 1);

            // the error takes the size of the set type
            assert_eq!(core::mem::size_of::<Error>(), 8);

            // a value past the last variant is rejected
            assert_eq!(ErrorKind::from_value(2), None);
        }
    }
}
