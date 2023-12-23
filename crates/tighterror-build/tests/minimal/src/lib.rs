//! This is a test crate to check `tighterror.yaml`
//! with minimal configuration.

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::errors::*;

    #[test]
    fn test_err_into_result() {
        let _res: Result<(), Error> = Error::from(codes::BAD_FILE).into();
    }

    #[test]
    fn test_err_code_into_result() {
        let _res: Result<(), Error> = codes::BAD_FILE.into();
    }
}
