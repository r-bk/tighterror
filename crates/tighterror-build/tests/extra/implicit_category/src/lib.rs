//! This is a test crate to check `tighterror.yaml`
//! with implicit category.

#![deny(missing_docs)]
#![deny(warnings)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_name() {
        fn foo(e: errors::IcError, name: &str) {
            assert_eq!(format!("{}", e.kind()), name);
        }
        foo(errors::kinds::general::BAD_INPUT.into(), "BAD_INPUT");
    }

    #[test]
    fn test_error_kind_name() {
        fn foo(ec: errors::IcErrorKind, name: &str) {
            assert_eq!(format!("{ec}"), name);
        }
        foo(errors::kinds::general::BAD_INPUT, "BAD_INPUT");
    }

    #[test]
    fn test_error_cat_name() {
        fn foo(ec: errors::IcErrorCategory, name: &str) {
            assert_eq!(format!("{ec}"), name);
        }
        foo(errors::categories::GENERAL, "GENERAL");
    }
}
