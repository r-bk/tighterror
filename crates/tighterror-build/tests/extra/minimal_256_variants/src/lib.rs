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
            assert_eq!(_p::KIND_BITS, u8::BITS as usize);
            assert_eq!(_p::CAT_BITS, 0);
            assert_eq!(_p::VAR_MASK, u8::MAX);
        }

        // Checks that 256 variants of a single category take the whole u8:
        // the last variant has the largest value of the type.
        #[test]
        fn test_last_variant_takes_the_largest_value() {
            let value: u8 = kind::ERR255.value();
            assert_eq!(value, u8::MAX);
            assert_eq!(ErrorKind::from_value(u8::MAX), Some(kind::ERR255));
        }
    }
}
