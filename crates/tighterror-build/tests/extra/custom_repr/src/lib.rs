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
        // u32: one category bit above two variant bits, with the masks
        // computed from the 32 bits of the type rather than from the 8 bits
        // the kinds would fit in.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 2);
            assert_eq!(_p::KIND_BITS, 3);
            assert_eq!(_p::VAR_MASK, 0b011);
            assert_eq!(_p::CAT_MASK, 0b100);

            // the kind values, in the set type
            let value: u32 = kind::TIMEOUT.value();
            assert_eq!(value, 0b001);
            assert_eq!(kind::QUEUE_FULL.value(), 0b010);
            assert_eq!(kind::CONNECTION_REFUSED.value(), 0b101);

            // the error takes the size of the set type
            assert_eq!(core::mem::size_of::<Error>(), 4);

            // the value 0 and the unused variants of the second category are
            // rejected
            assert_eq!(ErrorKind::from_value(0), None);
            assert_eq!(ErrorKind::from_value(0b100), None);
            assert_eq!(ErrorKind::from_value(0b110), None);

            // bits above the kind bits are rejected, rather than dropped
            assert_eq!(ErrorKind::from_value(0b1001), None);
            assert_eq!(ErrorKind::from_value(1 << 31 | 0b001), None);
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
        // u64: two variant bits and no category bits, with the mask computed
        // from the 64 bits of the type.
        #[test]
        fn test_layout() {
            // the bits and the mask
            assert_eq!(_p::CAT_BITS, 0);
            assert_eq!(_p::VAR_BITS, 2);
            assert_eq!(_p::KIND_BITS, 2);
            assert_eq!(_p::VAR_MASK, 0b11);

            // the kind values, in the set type
            let value: u64 = kind::TIMEOUT.value();
            assert_eq!(value, 1);
            assert_eq!(kind::QUEUE_FULL.value(), 2);

            // the error takes the size of the set type
            assert_eq!(core::mem::size_of::<Error>(), 8);

            // the value 0 and a value past the last variant are rejected
            assert_eq!(ErrorKind::from_value(0), None);
            assert_eq!(ErrorKind::from_value(3), None);
        }
    }
}
