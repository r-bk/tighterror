//! This is a test crate to check `tighterror.yaml`
//! with minimal configuration in no_std environment.

#![deny(warnings)]
#![no_std]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::errors::*;

    #[test]
    fn test_result_from_err() {
        let _res: Result<(), Error> = Error::from(kind::general::BAD_FILE).into();
    }

    #[test]
    fn test_result_from_err_kind() {
        let _res: Result<(), Error> = kind::general::BAD_FILE.into();
    }
}
