//! This is a test crate to check `tighterror.kdl`
//! with a module that wraps a module which wraps another one.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);

    #[cfg(test)]
    mod wrap_tests {
        use super::*;
        use test_wrap_render::errors as render;
        use test_wrap_text::errors as text;

        // Checks the layout of a module that wraps two crates, one of which
        // wraps the other: the four bits of the render kinds include the
        // three of the text kinds, and the set u16 holds them.
        #[test]
        fn test_layout() {
            // the widest wrapped kinds set the variant bits
            assert_eq!(_p::CAT_BITS, 2);
            assert_eq!(_p::VAR_BITS, 4);
            assert_eq!(_p::KIND_BITS, 6);

            // the own kind, in the set u16
            let value: u16 = kind::io::CONNECTION_REFUSED.value();
            assert_eq!(value, 0b10_0001);

            // the error, and an Option of it, take the size of the u16
            assert_eq!(core::mem::size_of::<Error>(), 2);
            assert_eq!(core::mem::size_of::<Option<Error>>(), 2);
        }

        // Checks that an error of the innermost crate propagates with `?`
        // through the middle one, and comes back through both. It simulates
        // a call chain across three crates.
        #[test]
        fn test_nested_error() {
            // the text error propagates through render into the app
            fn layout() -> Result<(), text::Error> {
                Err(text::kind::OVERFLOW.into())
            }
            fn render() -> Result<(), render::Error> {
                layout()?;
                Ok(())
            }
            fn app() -> Result<(), Error> {
                render()?;
                Ok(())
            }
            let e = app().unwrap_err();

            // the error is in the render category, under the text name
            assert_eq!(e.category(), category::RENDER);
            assert_eq!(e.kind().name(), "OVERFLOW");
            assert_eq!(e.to_string(), "Line overflows.");

            // the text kind comes back through render
            let r: render::ErrorKind = e.kind().wrapped().unwrap();
            assert_eq!(r.wrapped(), Some(text::kind::OVERFLOW));
            assert_eq!(e.kind().wrapped::<text::ErrorKind>(), None);

            // the same text error converted directly is in the text category
            let direct: Error = text::Error::from(text::kind::OVERFLOW).into();
            assert_eq!(direct.category(), category::TEXT);
            assert_ne!(direct.kind(), e.kind());
            assert_eq!(
                ErrorKind::from_value(direct.kind().value()),
                Some(direct.kind())
            );

            // the text category has the four variant bits of the render
            // kinds, and a bit above the three of the text kinds is rejected
            assert_eq!(
                ErrorKind::from_value(0b01_0010),
                Some(ErrorKind::from(text::kind::NO_FONT))
            );
            assert_eq!(ErrorKind::from_value(0b01_1010), None);
        }

        // A caller that matches the kinds with guards that get the wrapped
        // kinds back by their type, through both crates for a text error that
        // came through render.
        fn act_with_guards(k: ErrorKind) -> &'static str {
            match k {
                _ if let Some(r) = k.wrapped::<render::ErrorKind>() => match r {
                    _ if let Some(t) = r.wrapped() => match t {
                        text::kind::OVERFLOW => "reflow",
                        _ => "render",
                    },
                    _ => "render",
                },
                _ if let Some(t) = k.wrapped() => match t {
                    text::kind::OVERFLOW => "reflow",
                    _ => "text",
                },
                kind::io::CONNECTION_REFUSED => "reconnect",
                _ => "fail",
            }
        }

        // Checks that a match with guards in a module that wraps a wrapping
        // crate reaches the arm of a kind that came through render, directly
        // from text, and of the module itself.
        #[test]
        fn test_call_sites() {
            let via_render = |k| ErrorKind::from(render::ErrorKind::from(k));
            let cases = [
                (via_render(text::kind::OVERFLOW), "reflow"),
                (via_render(text::kind::NO_FONT), "render"),
                (ErrorKind::from(render::kind::TIMEOUT), "render"),
                (ErrorKind::from(text::kind::OVERFLOW), "reflow"),
                (ErrorKind::from(text::kind::NO_FONT), "text"),
                (kind::io::CONNECTION_REFUSED, "reconnect"),
            ];
            for (k, expected) in cases {
                assert_eq!(act_with_guards(k), expected, "{k:?}");
            }
        }
    }
}

/// Errors of the crates this one builds on.
pub mod relay {
    tighterror::include_errors!(relay);

    #[cfg(test)]
    mod wrap_tests {
        use super::*;
        use test_wrap_render::errors as render;
        use test_wrap_text::errors as text;

        // Checks a module that wraps two crates and has no error kinds of
        // its own. It simulates a crate that only relays the errors of the
        // crates it builds on.
        #[test]
        fn test_wrapping_only() {
            // the wrapped kinds alone set the variant bits
            assert_eq!(_p::CAT_BITS, 1);
            assert_eq!(_p::VAR_BITS, 4);
            assert_eq!(core::mem::size_of::<Option<Error>>(), 1);

            // an error of either crate converts, and comes back
            let e: Error = text::Error::from(text::kind::NO_FONT).into();
            assert_eq!(e.category(), category::TEXT);
            assert_eq!(e.to_string(), "Font is missing.");
            assert_eq!(e.kind().wrapped(), Some(text::kind::NO_FONT));
            let e: Error = render::Error::from(render::kind::TIMEOUT).into();
            assert_eq!(e.category(), category::RENDER);
            assert_eq!(e.kind().name(), "TIMEOUT");
            assert_eq!(e.kind().wrapped(), Some(render::kind::TIMEOUT));
            assert_eq!(ErrorKind::from_value(e.kind().value()), Some(e.kind()));
        }
    }
}
