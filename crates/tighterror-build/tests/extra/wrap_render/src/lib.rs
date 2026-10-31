//! This is a test crate to check `tighterror.kdl`
//! with a module that wraps the errors of another crate.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod wrap_tests {
        use super::*;
        use test_wrap_text::errors as text;

        // Checks the layout of a module that wraps the kinds of another
        // crate: the three bits of the wrapped kinds are more than the two
        // of the module's own kinds, and the u64 of the wrapped kinds doesn't
        // widen the u8 that the module needs.
        #[test]
        fn test_layout() {
            // the wrapped kinds set the variant bits
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 3);
            assert_eq!(_p::KIND_BITS, 4);

            // an own kind and a wrapped one, in a u8
            let value: u8 = kind::TIMEOUT.value();
            assert_eq!(value, 0b0001);
            assert_eq!(ErrorKind::from(text::kind::NO_FONT).value(), 0b1010);
            assert_eq!(ErrorKind::from(text::kind::OVERFLOW).value(), 0b1101);

            // the error, and an Option of it, take a single byte
            assert_eq!(core::mem::size_of::<Error>(), 1);
            assert_eq!(core::mem::size_of::<Option<Error>>(), 1);
        }

        // Checks that an error of the wrapped crate propagates with `?` and
        // keeps its name, display and kind. It simulates a function that
        // calls into the wrapped crate.
        #[test]
        fn test_wrapped_error() {
            // a function of the wrapped crate fails, and `?` converts its error
            fn layout() -> Result<(), text::Error> {
                Err(text::kind::OVERFLOW.into())
            }
            fn render() -> Result<(), Error> {
                layout()?;
                Ok(())
            }
            let e = render().unwrap_err();

            // the error is in the wrapping category, under the wrapped name
            assert_eq!(e.category(), category::TEXT);
            assert_eq!(e.kind().name(), "OVERFLOW");
            assert_eq!(e.to_string(), "Line overflows.");
            assert_eq!(
                format!("{:?}", e.kind()),
                "ErrorKind { cat: TEXT, var: OVERFLOW, val: 13 }"
            );

            // the wrapped kind comes back by its type, and an own kind has none
            assert_eq!(e.kind().wrapped(), Some(text::kind::OVERFLOW));
            assert_eq!(kind::TIMEOUT.wrapped::<text::ErrorKind>(), None);

            // an own error keeps its own display
            assert_eq!(
                Error::from(kind::TIMEOUT).to_string(),
                "Operation timed out."
            );
        }

        // A caller that matches the kinds with guards that get the wrapped
        // kind back by its type, next to arms on its own kinds.
        fn act_with_guards(k: ErrorKind) -> &'static str {
            match k {
                _ if let Some(t) = k.wrapped() => match t {
                    text::kind::NO_FONT => "retry",
                    _ => "skip",
                },
                kind::TIMEOUT => "wait",
                _ => "fail",
            }
        }

        // Checks that a match with guards reaches the arm of a wrapped kind
        // and of an own one. It simulates a caller that handles the errors
        // of the module.
        #[test]
        fn test_call_sites() {
            let cases = [
                (ErrorKind::from(text::kind::NO_FONT), "retry"),
                (ErrorKind::from(text::kind::BAD_GLYPH), "skip"),
                (ErrorKind::from(text::kind::OVERFLOW), "skip"),
                (kind::TIMEOUT, "wait"),
                (kind::BAD_ARG, "fail"),
            ];
            for (k, expected) in cases {
                assert_eq!(act_with_guards(k), expected, "{k:?}");
            }
        }

        // Checks that from_value() accepts the values of the wrapped kinds
        // and rejects every other value in the wrapping category.
        #[test]
        fn test_from_value() {
            // every wrapped kind comes back
            for k in [
                text::kind::BAD_GLYPH,
                text::kind::NO_FONT,
                text::kind::OVERFLOW,
            ] {
                let wrapped = ErrorKind::from(k);
                assert_eq!(ErrorKind::from_value(wrapped.value()), Some(wrapped));
            }

            // the values that no wrapped kind has are rejected: the variant 0
            // and the unused variants of both wrapped categories
            for v in [0b1000, 0b1011, 0b1100, 0b1110, 0b1111] {
                assert_eq!(ErrorKind::from_value(v), None, "{v:#b}");
            }

            // the unused variants of the own category are rejected
            for v in [0b0000, 0b0011, 0b0111] {
                assert_eq!(ErrorKind::from_value(v), None, "{v:#b}");
            }

            // a bit above the kind bits is rejected
            assert_eq!(ErrorKind::from_value(0b1_1101), None);
        }
    }
}
