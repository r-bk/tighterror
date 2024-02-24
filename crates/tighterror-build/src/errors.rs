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
    const BITS: usize = _p::CAT_BITS;

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

/// Error kind type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct TebErrorKind(_p::T);

impl TebErrorKind {
    const fn new(cat: TebErrorCategory, variant: _p::T) -> Self {
        Self(variant << _p::CAT_BITS | cat.0)
    }

    #[inline]
    fn category_value(&self) -> _p::T {
        self.0 & _p::CAT_MASK
    }

    #[inline]
    fn variant_value(&self) -> _p::T {
        self.0 >> _p::CAT_BITS
    }

    /// Returns the error category.
    #[inline]
    pub fn category(&self) -> TebErrorCategory {
        TebErrorCategory::new(self.category_value())
    }

    /// Returns the error kind name.
    #[inline]
    pub fn name(&self) -> &'static str {
        _n::A[self.category_value() as usize][self.variant_value() as usize]
    }

    #[inline]
    fn display(&self) -> &'static str {
        _d::A[self.category_value() as usize][self.variant_value() as usize]
    }

    /// Returns the error kind value as the underlying Rust type.
    #[inline]
    pub fn value(&self) -> _p::T {
        self.0
    }

    /// Creates an error kind from a raw value of the underlying Rust type.
    #[inline]
    pub fn from_value(value: _p::T) -> Option<Self> {
        let cat = value & _p::CAT_MASK;
        let variant = value >> _p::CAT_BITS;
        if cat == _p::CAT_MAX && variant <= _p::VAR_MAXES[cat as usize] {
            Some(Self::new(TebErrorCategory::new(cat), variant))
        } else {
            None
        }
    }
}

impl tighterror::TightErrorKind for TebErrorKind {
    type ReprType = _p::T;
    type CategoryType = TebErrorCategory;
    const BITS: usize = _p::KIND_BITS;

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

impl core::fmt::Display for TebErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }
}

impl<T> core::convert::From<TebErrorKind> for Result<T, TebError> {
    #[inline]
    fn from(v: TebErrorKind) -> Self {
        Err(v.into())
    }
}

/// Error type.
#[derive(Debug)]
#[repr(transparent)]
pub struct TebError(TebErrorKind);

impl TebError {
    /// Returns the error kind.
    #[inline]
    pub fn kind(&self) -> TebErrorKind {
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
    type CategoryType = TebErrorCategory;
    type KindType = TebErrorKind;

    #[inline]
    fn kind(&self) -> Self::KindType {
        self.kind()
    }

    #[inline]
    fn location(&self) -> tighterror::Location {
        self.location()
    }
}

impl core::convert::From<TebErrorKind> for TebError {
    #[inline]
    fn from(kind: TebErrorKind) -> Self {
        Self(kind)
    }
}

impl core::fmt::Display for TebError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.kind().display())
    }
}

impl core::cmp::PartialEq for TebError {
    /// Checks equality based on the error kind only.
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
    pub const SPEC_FILE_NOT_FOUND: &str = "SPEC_FILE_NOT_FOUND";
    pub const FAILED_TO_OPEN_SPEC_FILE: &str = "FAILED_TO_OPEN_SPEC_FILE";
    pub const BAD_SPEC: &str = "BAD_SPEC";
    pub const BAD_YAML: &str = "BAD_YAML";
    pub const BAD_TOML: &str = "BAD_TOML";
    pub const BAD_SPEC_FILE_EXTENSION: &str = "BAD_SPEC_FILE_EXTENSION";
    pub const FAILED_TO_WRITE_DST_FILE: &str = "FAILED_TO_WRITE_DST_FILE";
    pub const FAILED_TO_READ_DST_FILE: &str = "FAILED_TO_READ_DST_FILE";
    pub const FAILED_TO_PARSE_TOKENS: &str = "FAILED_TO_PARSE_TOKENS";
    pub const RUSTFMT_FAILED: &str = "RUSTFMT_FAILED";
    pub static GENERAL_NAMES: [&str; 10] = [
        SPEC_FILE_NOT_FOUND,
        FAILED_TO_OPEN_SPEC_FILE,
        BAD_SPEC,
        BAD_YAML,
        BAD_TOML,
        BAD_SPEC_FILE_EXTENSION,
        FAILED_TO_WRITE_DST_FILE,
        FAILED_TO_READ_DST_FILE,
        FAILED_TO_PARSE_TOKENS,
        RUSTFMT_FAILED,
    ];

    pub static A: [&[&str]; 1] = [&GENERAL_NAMES];
}

mod _d {
    pub const SPEC_FILE_NOT_FOUND: &str = "Specification file couldn't be found.";
    pub const FAILED_TO_OPEN_SPEC_FILE: &str = "Specification file couldn't be opened.";
    pub const BAD_SPEC: &str = "Bad specification file format.";
    pub const BAD_YAML: &str = "Bad YAML file format.";
    pub const BAD_TOML: &str = "Bad TOML file format.";
    pub const BAD_SPEC_FILE_EXTENSION: &str = "Bad specification file name extension.";
    pub const FAILED_TO_WRITE_DST_FILE: &str = "Destination file couldn't be written.";
    pub const FAILED_TO_READ_DST_FILE: &str = "Destination file couldn't be read.";
    pub const FAILED_TO_PARSE_TOKENS: &str = "Generated code tokens couldn't be parsed.";
    pub const RUSTFMT_FAILED: &str = "Rustfmt tool exited with an error.";
    pub static GENERAL_DISPLAY: [&str; 10] = [
        SPEC_FILE_NOT_FOUND,
        FAILED_TO_OPEN_SPEC_FILE,
        BAD_SPEC,
        BAD_YAML,
        BAD_TOML,
        BAD_SPEC_FILE_EXTENSION,
        FAILED_TO_WRITE_DST_FILE,
        FAILED_TO_READ_DST_FILE,
        FAILED_TO_PARSE_TOKENS,
        RUSTFMT_FAILED,
    ];

    pub static A: [&[&str]; 1] = [&GENERAL_DISPLAY];
}

mod _p {
    pub type T = u8;
    pub const KIND_BITS: usize = 4;
    pub const CAT_BITS: usize = 0;
    pub const CAT_MASK: T = 0;
    pub const CAT_MAX: T = 0;
    pub static VAR_MAXES: [T; 1] = [9];
    const _: () = assert!(KIND_BITS <= T::BITS as usize);
    const _: () = assert!(CAT_BITS <= usize::BITS as usize);
}

/// Error category constants.
pub mod categories {
    use super::TebErrorCategory as C;

    /// General error category.
    pub const GENERAL: C = C::new(0);
}

/// Error kind constants.
pub mod kinds {
    use super::categories as c;
    use super::TebErrorKind as EK;

    /// Specification file couldn't be found.
    pub const SPEC_FILE_NOT_FOUND: EK = EK::new(c::GENERAL, 0);

    /// Specification file couldn't be opened.
    pub const FAILED_TO_OPEN_SPEC_FILE: EK = EK::new(c::GENERAL, 1);

    /// Bad specification file format.
    pub const BAD_SPEC: EK = EK::new(c::GENERAL, 2);

    /// Bad YAML file format.
    pub const BAD_YAML: EK = EK::new(c::GENERAL, 3);

    /// Bad TOML file format.
    pub const BAD_TOML: EK = EK::new(c::GENERAL, 4);

    /// Bad specification file name extension.
    pub const BAD_SPEC_FILE_EXTENSION: EK = EK::new(c::GENERAL, 5);

    /// Destination file couldn't be written.
    pub const FAILED_TO_WRITE_DST_FILE: EK = EK::new(c::GENERAL, 6);

    /// Destination file couldn't be read.
    pub const FAILED_TO_READ_DST_FILE: EK = EK::new(c::GENERAL, 7);

    /// Generated code tokens couldn't be parsed.
    pub const FAILED_TO_PARSE_TOKENS: EK = EK::new(c::GENERAL, 8);

    /// Rustfmt tool exited with an error.
    pub const RUSTFMT_FAILED: EK = EK::new(c::GENERAL, 9);
}
