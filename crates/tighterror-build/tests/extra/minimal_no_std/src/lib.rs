//! This is a test crate to check `tighterror.kdl`
//! with minimal configuration in no_std environment.

#![deny(warnings)]
#![no_std]

/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);
}

#[cfg(test)]
mod tests {
    use super::errors::*;

    #[test]
    fn test_result_from_err() {
        let _res: Result<(), Error> = Error::from(kind::BAD_FILE).into();
    }

    #[test]
    fn test_result_from_err_kind() {
        let _res: Result<(), Error> = kind::BAD_FILE.into();
    }

    // Checks that a no_std crate gets the error trait by default: the error
    // implements the trait from core, which needs no std.
    #[test]
    fn test_error_trait() {
        fn is_error<T: core::error::Error>(_: &T) {}
        is_error(&Error::from(kind::BAD_FILE));
    }
}
