//! This is a test crate to check `tighterror.yaml`
//! with minimal configuration.

#![deny(warnings)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));

    #[cfg(test)]
    mod more_tests {
        use super::*;

        #[test]
        fn test_constants() {
            assert_eq!(_p::KIND_BITS, u8::BITS as usize);
            assert_eq!(_p::CAT_BITS, 0);
        }
    }
}
