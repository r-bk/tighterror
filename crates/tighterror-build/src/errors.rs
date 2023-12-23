//! Crate errors.

/// Error category type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct TebErrorCategory(_p::T);

impl TebErrorCategory {
    #[inline]
    const fn new(v: _p::T) -> Self {
        debug_assert!(v == _p::CAT_MAX);
        Self(v)
    }

    /// Returns the name of the error category.
    #[inline]
    pub fn name(&self) -> &'static str {
        _cn::A[self.0 as usize]
    }
}

impl tighterror::TightErrorCategory for TebErrorCategory {
    type ReprType = _p::T;
    const CATEGORY_BITS: usize = _p::CAT_BITS;
    const KIND_BITS: usize = _p::KIND_BITS;
    const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }
}

impl core::fmt::Display for TebErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }
}

/// Error code type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct TebErrorCode(_p::T);

impl TebErrorCode {
    const fn new(cat: TebErrorCategory, kind: _p::T) -> Self {
        Self(cat.0 << _p::KIND_BITS | kind)
    }

    #[inline]
    fn category_value(&self) -> _p::T {
        self.0 >> _p::KIND_BITS
    }

    #[inline]
    fn kind_value(&self) -> _p::T {
        self.0 & _p::KIND_MASK
    }

    /// Returns the error code category.
    #[inline]
    pub fn category(&self) -> TebErrorCategory {
        TebErrorCategory::new(self.category_value())
    }

    /// Returns the error code name.
    #[inline]
    pub fn name(&self) -> &'static str {
        _n::A[self.category_value() as usize][self.kind_value() as usize]
    }

    #[inline]
    fn display(&self) -> &'static str {
        _d::A[self.category_value() as usize][self.kind_value() as usize]
    }

    /// Returns the error code value as the underlying Rust type.
    #[inline]
    pub fn value(&self) -> _p::T {
        self.0
    }

    /// Creates an error code from a raw value of the underlying Rust type.
    #[inline]
    pub fn from_value(value: _p::T) -> Option<Self> {
        let cat = (value & _p::CAT_MASK) >> _p::KIND_BITS;
        let kind = value & _p::KIND_MASK;
        if cat == _p::CAT_MAX && kind <= _p::KIND_MAXES[cat as usize] {
            Some(Self::new(TebErrorCategory::new(cat), kind))
        } else {
            None
        }
    }
}

impl tighterror::TightErrorCode for TebErrorCode {
    type ReprType = _p::T;
    type CategoryType = TebErrorCategory;
    const CATEGORY_BITS: usize = _p::CAT_BITS;
    const KIND_BITS: usize = _p::KIND_BITS;
    const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

    #[inline]
    fn category(&self) -> Self::CategoryType {
        self.category()
    }

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }

    #[inline]
    fn value(&self) -> Self::ReprType {
        self.value()
    }

    #[inline]
    fn from_value(value: Self::ReprType) -> Option<Self> {
        Self::from_value(value)
    }
}

impl core::fmt::Display for TebErrorCode {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }
}

impl<T> core::convert::From<TebErrorCode> for Result<T, TebError> {
    #[inline]
    fn from(v: TebErrorCode) -> Self {
        Err(v.into())
    }
}

/// Error type.
#[derive(Debug)]
#[repr(transparent)]
pub struct TebError(TebErrorCode);

impl TebError {
    /// Returns the error code.
    #[inline]
    pub fn code(&self) -> TebErrorCode {
        self.0
    }

    /// Returns the error origin location.
    #[inline]
    pub fn location(&self) -> tighterror::Location {
        tighterror::Location::undefined()
    }
}

impl tighterror::TightError for TebError {
    type ReprType = _p::T;
    type CodeType = TebErrorCode;
    const CATEGORY_BITS: usize = _p::CAT_BITS;
    const KIND_BITS: usize = _p::KIND_BITS;
    const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

    #[inline]
    fn code(&self) -> Self::CodeType {
        self.code()
    }

    #[inline]
    fn location(&self) -> tighterror::Location {
        self.location()
    }
}

impl core::convert::From<TebErrorCode> for TebError {
    #[inline]
    fn from(code: TebErrorCode) -> Self {
        Self(code)
    }
}

impl core::fmt::Display for TebError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.code().display())
    }
}

impl core::cmp::PartialEq for TebError {
    /// Checks equality based on the error code only.
    #[inline]
    fn eq(&self, other: &TebError) -> bool {
        self.0 == other.0
    }
}

impl<T> core::convert::From<TebError> for core::result::Result<T, TebError> {
    #[inline]
    fn from(err: TebError) -> Self {
        Err(err)
    }
}

impl std::error::Error for TebError {}

mod _cn {
    pub const GENERAL: &str = "GENERAL";
    pub static A: [&str; 1] = [GENERAL];
}

mod _n {
    pub const FAILED_TO_OPEN_SPEC_FILE: &str = "FAILED_TO_OPEN_SPEC_FILE";
    pub const BAD_SPEC: &str = "BAD_SPEC";
    pub const BAD_YAML: &str = "BAD_YAML";
    pub const BAD_TOML: &str = "BAD_TOML";
    pub const BAD_SPEC_FILE_EXTENSION: &str = "BAD_SPEC_FILE_EXTENSION";
    pub const FAILED_TO_WRITE_TO_DST_FILE: &str = "FAILED_TO_WRITE_TO_DST_FILE";
    pub const FAILED_TO_PARSE_TOKENS: &str = "FAILED_TO_PARSE_TOKENS";
    pub const RUSTFMT_FAILED: &str = "RUSTFMT_FAILED";
    pub static GENERAL__NAMES: [&str; 8] = [
        FAILED_TO_OPEN_SPEC_FILE,
        BAD_SPEC,
        BAD_YAML,
        BAD_TOML,
        BAD_SPEC_FILE_EXTENSION,
        FAILED_TO_WRITE_TO_DST_FILE,
        FAILED_TO_PARSE_TOKENS,
        RUSTFMT_FAILED,
    ];

    pub static A: [&[&str]; 1] = [&GENERAL__NAMES];
}

mod _d {
    pub const FAILED_TO_OPEN_SPEC_FILE: &str = "Specification file couldn't be opened.";
    pub const BAD_SPEC: &str = "Bad specification file format.";
    pub const BAD_YAML: &str = "Bad YAML file format.";
    pub const BAD_TOML: &str = "Bad TOML file format.";
    pub const BAD_SPEC_FILE_EXTENSION: &str = "Bad specification file name extension.";
    pub const FAILED_TO_WRITE_TO_DST_FILE: &str = "Destination file couldn't be written.";
    pub const FAILED_TO_PARSE_TOKENS: &str = "Generated code tokens couldn't be parsed.";
    pub const RUSTFMT_FAILED: &str = "Rustfmt tool exited with an error.";
    pub static GENERAL__DISPLAY: [&str; 8] = [
        FAILED_TO_OPEN_SPEC_FILE,
        BAD_SPEC,
        BAD_YAML,
        BAD_TOML,
        BAD_SPEC_FILE_EXTENSION,
        FAILED_TO_WRITE_TO_DST_FILE,
        FAILED_TO_PARSE_TOKENS,
        RUSTFMT_FAILED,
    ];

    pub static A: [&[&str]; 1] = [&GENERAL__DISPLAY];
}

mod _p {
    pub type T = u8;
    pub const CAT_BITS: usize = 0;
    pub const CAT_COUNT: usize = 1;
    pub const CAT_MASK: T = 0;
    pub const CAT_MAX: T = 0;
    pub const KIND_BITS: usize = 3;
    pub const KIND_MASK: T = 7;
    pub static KIND_MAXES: [T; 1] = [7];
    const _: () = assert!(CAT_BITS + KIND_BITS <= T::BITS as usize);
    const _: () = assert!(CAT_COUNT <= i16::MAX as usize);
}

/// Error category constants.
pub mod categories {
    use super::TebErrorCategory as C;

    /// General error category.
    pub const GENERAL: C = C::new(0);
}

/// Error-code constants.
pub mod codes {
    use super::categories as c;
    use super::TebErrorCode as E;

    /// Specification file couldn't be opened.
    pub const FAILED_TO_OPEN_SPEC_FILE: E = E::new(c::GENERAL, 0);

    /// Bad specification file format.
    pub const BAD_SPEC: E = E::new(c::GENERAL, 1);

    /// Bad YAML file format.
    pub const BAD_YAML: E = E::new(c::GENERAL, 2);

    /// Bad TOML file format.
    pub const BAD_TOML: E = E::new(c::GENERAL, 3);

    /// Bad specification file name extension.
    pub const BAD_SPEC_FILE_EXTENSION: E = E::new(c::GENERAL, 4);

    /// Destination file couldn't be written.
    pub const FAILED_TO_WRITE_TO_DST_FILE: E = E::new(c::GENERAL, 5);

    /// Generated code tokens couldn't be parsed.
    pub const FAILED_TO_PARSE_TOKENS: E = E::new(c::GENERAL, 6);

    /// Rustfmt tool exited with an error.
    pub const RUSTFMT_FAILED: E = E::new(c::GENERAL, 7);
}
