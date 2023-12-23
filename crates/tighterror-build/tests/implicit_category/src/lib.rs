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
/// use test_implicit_category::errors::{codes::BAD_INPUT, Error};
/// let _res: Result<(), Error> = Error::from(BAD_INPUT).into();
/// ```
///
/// Negative `err_code_into_result`:
///
/// ```compile_fail
/// use test_implicit_category::errors::{codes::BAD_INPUT, Error};
/// let _res: Result<(), Error> = BAD_INPUT.into();
/// ```
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}
