//! Error codes.
use std::{error::Error, fmt::Display};

type ReprType = u8;

/// The error type of `tighterror-build` error codes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TebError(ReprType);

impl TebError {
    #[inline]
    const fn new(v: ReprType) -> Self {
        Self(v)
    }
}

impl Display for TebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            kinds::FAILED_TO_OPEN_SPEC_FILE => "FAILED_TO_OPEN_SPEC_FILE",
            kinds::BAD_SPEC => "BAD_SPEC",
            kinds::BAD_YAML => "BAD_YAML",
            kinds::BAD_TOML => "BAD_TOML",
            kinds::BAD_SPEC_FILE_EXTENSION => "BAD_SPEC_FILE_EXTENSION",
            kinds::FAILED_TO_WRITE_TO_DST_FILE => "FAILED_TO_WRITE_TO_DST_FILE",
            kinds::FAILED_TO_PARSE_TOKENS => "FAILED_TO_PARSE_TOKENS",
            kinds::RUSTFMT_FAILED => "RUSTFMT_FAILED",
            _ => "__UNKNOWN_TEB_ERROR__",
        };
        write!(f, "{}", name)
    }
}

impl Error for TebError {}

impl<T> From<TebError> for Result<T, TebError> {
    #[inline]
    fn from(value: TebError) -> Self {
        Err(value)
    }
}

// ----------------------------------------------------------------------------

mod kinds {
    use super::TebError;

    /// A specification file cannot be open.
    pub const FAILED_TO_OPEN_SPEC_FILE: TebError = TebError::new(1);

    /// A specification file contains invalid data.
    pub const BAD_SPEC: TebError = TebError::new(2);

    /// A specification file is not a valid YAML file.
    pub const BAD_YAML: TebError = TebError::new(3);

    /// A specification file is not a valid TOML file.
    pub const BAD_TOML: TebError = TebError::new(4);

    /// A specification file extension isn't supported or is missing.
    pub const BAD_SPEC_FILE_EXTENSION: TebError = TebError::new(5);

    /// Destination file cannot be written.
    pub const FAILED_TO_WRITE_TO_DST_FILE: TebError = TebError::new(6);

    /// Generated code coudn't be parsed
    pub const FAILED_TO_PARSE_TOKENS: TebError = TebError::new(7);

    /// Rustfmt command line tool failed
    pub const RUSTFMT_FAILED: TebError = TebError::new(8);
}

pub use kinds::*;
