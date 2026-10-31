//! This is a test crate to check `tighterror.kdl`
//! with categories that fill every bit of the underlying type.

#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod layout_tests {
        use super::*;

        // Checks the layout of an error kind with two categories that fill
        // every bit of a u8: one category bit above seven variant bits, which
        // the second category fills with 127 kinds and the unused variant 0.
        // It simulates the largest category that a u8 holds next to another
        // one.
        #[test]
        fn test_layout() {
            // the bits and the masks
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 7);
            assert_eq!(_p::KIND_BITS, 8);
            assert_eq!(_p::VAR_MASK, 0x7f);
            assert_eq!(_p::CAT_MASK, 0x80);

            // the first kind of each category, and the last kind of the
            // second one at the top of the u8
            let value: u8 = kind::ONLY.value();
            assert_eq!(value, 0x01);
            assert_eq!(kind::ERR0.value(), 0x81);
            assert_eq!(kind::ERR126.value(), u8::MAX);
            assert_eq!(ErrorKind::from_value(u8::MAX), Some(kind::ERR126));
            assert_eq!(kind::ERR126.category(), category::SECOND);
            assert_eq!(kind::ERR126.name(), "ERR126");

            // the variant 0 of each category and the unused variants of the
            // first one are rejected
            assert_eq!(ErrorKind::from_value(0), None);
            assert_eq!(ErrorKind::from_value(0x80), None);
            assert_eq!(ErrorKind::from_value(0x02), None);
            assert_eq!(ErrorKind::from_value(0x7f), None);

            // the error, and an Option of it, take a single byte
            assert_eq!(core::mem::size_of::<Error>(), 1);
            assert_eq!(core::mem::size_of::<Option<Error>>(), 1);
        }
    }
}
