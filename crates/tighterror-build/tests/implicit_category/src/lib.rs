//! This is a test crate to check `tighterror.yaml`
//! with implicit category.

#![deny(missing_docs)]

/// Crate errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

#[cfg(test)]
mod tests {
    use super::errors::*;

    #[test]
    fn test_err_into_result() {
        let _res: Result<(), Error> = Error::from(codes::BAD_INPUT).into();
    }
}
