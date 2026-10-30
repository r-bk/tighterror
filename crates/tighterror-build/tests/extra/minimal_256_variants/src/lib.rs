//! This is a test crate to check `tighterror.kdl`
//! with minimal configuration.

#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod more_tests {
        use super::*;

        #[test]
        fn test_constants() {
            assert_eq!(_p::KIND_BITS, 9);
            assert_eq!(_p::CAT_BITS, 0);
            assert_eq!(_p::VAR_MASK, 0x1ff);
        }

        // Checks that 256 variants of a single category take a u16: the
        // value 0 is not used, so the last variant has the value 256.
        #[test]
        fn test_last_variant_takes_a_wider_type() {
            // the first and the last variant, in the wider type
            let value: u16 = kind::ERR255.value();
            assert_eq!(value, 256);
            assert_eq!(kind::ERR0.value(), 1);
            assert_eq!(ErrorKind::from_value(256), Some(kind::ERR255));

            // the value 0 and a value past the last variant are rejected
            assert_eq!(ErrorKind::from_value(0), None);
            assert_eq!(ErrorKind::from_value(257), None);

            // the error takes the size of the wider type
            assert_eq!(core::mem::size_of::<Error>(), 2);
        }
    }
}
