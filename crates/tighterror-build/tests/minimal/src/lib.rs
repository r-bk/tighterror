//! This is a test crate to check `tighterror.yaml`
//! with minimal configuration.

#![deny(warnings)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::errors::*;

    #[test]
    fn test_result_from_err() {
        let _res: Result<(), Error> = Error::from(kinds::BAD_FILE).into();
    }

    #[test]
    fn test_result_from_err_kind() {
        let _res: Result<(), Error> = kinds::BAD_FILE.into();
    }

    #[test]
    fn test_error_trait() {
        fn foo<T: std::error::Error>(e: T) -> String {
            format!("{e}")
        }
        let e: Error = kinds::BAD_FILE.into();
        assert_eq!(foo(e), "BAD_FILE");
    }

    #[test]
    fn test_into_anyhow() {
        let e: anyhow::Error = Error::from(kinds::BAD_FILE).into();
        assert_eq!(format!("{e}"), "BAD_FILE");
    }
}
