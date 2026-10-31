//! This is a test crate to check `tighterror.kdl`
//! with minimal configuration.

#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod more_tests {
        use super::*;

        // Checks that 255 variants of a single category fill every bit of a
        // u8: the value 0 is not used, so the last variant has the value 255.
        // It simulates the largest category that a u8 holds.
        #[test]
        fn test_last_variant_fills_the_type() {
            // the bits and the mask
            assert_eq!(_p::KIND_BITS, 8);
            assert_eq!(_p::CAT_BITS, 0);
            assert_eq!(_p::VAR_MASK, 0xff);

            // the first and the last variant, in a u8
            let value: u8 = kind::ERR254.value();
            assert_eq!(value, u8::MAX);
            assert_eq!(kind::ERR0.value(), 1);
            assert_eq!(ErrorKind::from_value(u8::MAX), Some(kind::ERR254));
            assert_eq!(kind::ERR254.name(), "ERR254");

            // the value 0 is rejected
            assert_eq!(ErrorKind::from_value(0), None);

            // the error, and an Option of it, take a single byte
            assert_eq!(core::mem::size_of::<Error>(), 1);
            assert_eq!(core::mem::size_of::<Option<Error>>(), 1);
        }
    }
}
