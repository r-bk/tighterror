//! This is a test crate to check `tighterror.yaml`
//! with implicit category.

#![deny(missing_docs)]

/// Crate errors.
///
/// # Tests
///
/// Negative `err_into_result`:
///
/// ```compile_fail
/// use test_implicit_category::errors::{codes::BAD_INPUT, IcError};
/// let _res: Result<(), IcError> = IcError::from(BAD_INPUT).into();
/// ```
///
/// Negative `err_code_into_result`:
///
/// ```compile_fail
/// use test_implicit_category::errors::{codes::BAD_INPUT, IcError};
/// let _res: Result<(), IcError> = BAD_INPUT.into();
/// ```
///
/// Negative `error_trait`:
///
/// ```compile_fail
/// use test_implicit_category::errors::{codes::BAD_INPUT, IcError};
///
/// fn foo<T: std::error::Error>(e: T) -> String {
///     format!("{e}")
/// }
///
/// let e = IcError::from(BAD_INPUT);
/// assert_eq!(foo(e), "BAD_INPUT");
/// ```
///
/// Negative `into anyhow`:
///
/// ```compile_fail
/// use test_implicit_category::errors::{codes::BAD_INPUT, IcError};
/// let e: anyhow::Error = IcError::from(BAD_INPUT).into();
/// ```
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_name() {
        fn foo(e: errors::IcError, name: &str) {
            assert_eq!(format!("{}", e.code()), name);
        }
        foo(errors::codes::BAD_INPUT.into(), "BAD_INPUT");
    }
}
