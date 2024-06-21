//! Crate errors.

/**
 * Error category type.
 *
 * See the [categories] module for category constants.
*/

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct TbErrorCategory(_p::R);

impl TbErrorCategory {
    #[inline]
    const fn new(v: _p::R) -> Self {
        debug_assert!(v <= _p::CAT_MAX);
        Self(v)
    }

    /// Returns the name of the error category.
    #[inline]
    pub fn name(&self) -> &'static str {
        _cn::A[self.0 as usize]
    }
}

impl tighterror::TightErrorCategory for TbErrorCategory {
    type R = _p::R;
    const BITS: usize = _p::CAT_BITS;

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }
}

impl core::fmt::Display for TbErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }
}

impl core::fmt::Debug for TbErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TbErrorCategory")
            .field(&_p::Ident(self.name()))
            .finish()
    }
}

/**
 * Error kind type.
 *
 * See the [kinds] module for error kind constants.
*/

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct TbErrorKind(_p::R);

impl TbErrorKind {
    const fn new(cat: TbErrorCategory, variant: _p::R) -> Self {
        Self(variant << _p::CAT_BITS | cat.0)
    }

    #[inline]
    fn category_value(&self) -> _p::R {
        self.0 & _p::CAT_MASK
    }

    #[inline]
    fn variant_value(&self) -> _p::R {
        self.0 >> _p::CAT_BITS
    }

    /// Returns the error category.
    #[inline]
    pub fn category(&self) -> TbErrorCategory {
        TbErrorCategory::new(self.category_value())
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
    pub fn value(&self) -> _p::R {
        self.0
    }

    /// Creates an error kind from a raw value of the underlying Rust type.
    #[inline]
    pub fn from_value(value: _p::R) -> Option<Self> {
        let cat = value & _p::CAT_MASK;
        let variant = value >> _p::CAT_BITS;
        if cat <= _p::CAT_MAX && variant <= _p::VAR_MAXES[cat as usize] {
            Some(Self::new(TbErrorCategory::new(cat), variant))
        } else {
            None
        }
    }
}

impl tighterror::TightErrorKind for TbErrorKind {
    type R = _p::R;
    type Category = TbErrorCategory;
    const BITS: usize = _p::KIND_BITS;

    #[inline]
    fn category(&self) -> Self::Category {
        self.category()
    }

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }

    #[inline]
    fn value(&self) -> Self::R {
        self.value()
    }

    #[inline]
    fn from_value(value: Self::R) -> Option<Self> {
        Self::from_value(value)
    }
}

impl core::fmt::Display for TbErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }
}

impl core::fmt::Debug for TbErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TbErrorKind")
            .field("cat", &_p::Ident(self.category().name()))
            .field("var", &_p::Ident(self.name()))
            .field("val", &self.0)
            .finish()
    }
}

impl<T> core::convert::From<TbErrorKind> for Result<T, TbError> {
    #[inline]
    fn from(v: TbErrorKind) -> Self {
        Err(v.into())
    }
}

/**
 * Error type.
 *
 * See the [kinds] module for error kind constants.
*/

#[derive(Debug)]
#[repr(transparent)]
pub struct TbError(TbErrorKind);

impl TbError {
    /// Returns the error kind.
    #[inline]
    pub fn kind(&self) -> TbErrorKind {
        self.0
    }

    /// Returns the error origin location.
    #[inline]
    pub fn location(&self) -> tighterror::Location {
        tighterror::Location::undefined()
    }
}

impl tighterror::TightError for TbError {
    type R = _p::R;
    type Category = TbErrorCategory;
    type Kind = TbErrorKind;

    #[inline]
    fn kind(&self) -> Self::Kind {
        self.kind()
    }

    #[inline]
    fn location(&self) -> tighterror::Location {
        self.location()
    }
}

impl core::convert::From<TbErrorKind> for TbError {
    #[inline]
    fn from(kind: TbErrorKind) -> Self {
        Self(kind)
    }
}

impl core::fmt::Display for TbError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.kind().display())
    }
}

impl core::cmp::PartialEq for TbError {
    /// Checks equality based on the error kind only.
    #[inline]
    fn eq(&self, other: &TbError) -> bool {
        self.0 == other.0
    }
}

impl<T> core::convert::From<TbError> for core::result::Result<T, TbError> {
    #[inline]
    fn from(err: TbError) -> Self {
        Err(err)
    }
}

impl std::error::Error for TbError {}

mod _cn {
    pub const PARSER: &str = "PARSER";
    pub const CODER: &str = "CODER";
    pub static A: [&str; 2] = [PARSER, CODER];
}

mod _n {
    mod parser {
        const BAD_IDENTIFIER_CHARACTERS: &str = "BAD_IDENTIFIER_CHARACTERS";
        const BAD_IDENTIFIER_CASE: &str = "BAD_IDENTIFIER_CASE";
        const BAD_KEYWORD_TYPE: &str = "BAD_KEYWORD_TYPE";
        const BAD_MODULE_IDENTIFIER: &str = "BAD_MODULE_IDENTIFIER";
        const BAD_NAME: &str = "BAD_NAME";
        const BAD_OBJECT_ATTRIBUTE: &str = "BAD_OBJECT_ATTRIBUTE";
        const BAD_SPEC_FILE_EXTENSION: &str = "BAD_SPEC_FILE_EXTENSION";
        const BAD_TOML: &str = "BAD_TOML";
        const BAD_ROOT_LEVEL_KEYWORD: &str = "BAD_ROOT_LEVEL_KEYWORD";
        const BAD_VALUE_TYPE: &str = "BAD_VALUE_TYPE";
        const BAD_YAML: &str = "BAD_YAML";
        const EMPTY_IDENTIFIER: &str = "EMPTY_IDENTIFIER";
        const EMPTY_LIST: &str = "EMPTY_LIST";
        const FAILED_TO_OPEN_SPEC_FILE: &str = "FAILED_TO_OPEN_SPEC_FILE";
        const MISSING_ATTRIBUTE: &str = "MISSING_ATTRIBUTE";
        const MUTUALLY_EXCLUSIVE_KEYWORDS: &str = "MUTUALLY_EXCLUSIVE_KEYWORDS";
        const NON_UNIQUE_NAME: &str = "NON_UNIQUE_NAME";
        const SPEC_FILE_NOT_FOUND: &str = "SPEC_FILE_NOT_FOUND";
        pub static A: [&str; 18] = [
            BAD_IDENTIFIER_CHARACTERS,
            BAD_IDENTIFIER_CASE,
            BAD_KEYWORD_TYPE,
            BAD_MODULE_IDENTIFIER,
            BAD_NAME,
            BAD_OBJECT_ATTRIBUTE,
            BAD_SPEC_FILE_EXTENSION,
            BAD_TOML,
            BAD_ROOT_LEVEL_KEYWORD,
            BAD_VALUE_TYPE,
            BAD_YAML,
            EMPTY_IDENTIFIER,
            EMPTY_LIST,
            FAILED_TO_OPEN_SPEC_FILE,
            MISSING_ATTRIBUTE,
            MUTUALLY_EXCLUSIVE_KEYWORDS,
            NON_UNIQUE_NAME,
            SPEC_FILE_NOT_FOUND,
        ];
    }

    mod coder {
        const CATEGORY_REQUIRED: &str = "CATEGORY_REQUIRED";
        const ERROR_REQUIRED: &str = "ERROR_REQUIRED";
        const FAILED_TO_PARSE_TOKENS: &str = "FAILED_TO_PARSE_TOKENS";
        const FAILED_TO_READ_OUTPUT_FILE: &str = "FAILED_TO_READ_OUTPUT_FILE";
        const FAILED_TO_WRITE_OUTPUT_FILE: &str = "FAILED_TO_WRITE_OUTPUT_FILE";
        const RUSTFMT_FAILED: &str = "RUSTFMT_FAILED";
        const TOO_MANY_BITS: &str = "TOO_MANY_BITS";
        pub static A: [&str; 7] = [
            CATEGORY_REQUIRED,
            ERROR_REQUIRED,
            FAILED_TO_PARSE_TOKENS,
            FAILED_TO_READ_OUTPUT_FILE,
            FAILED_TO_WRITE_OUTPUT_FILE,
            RUSTFMT_FAILED,
            TOO_MANY_BITS,
        ];
    }

    pub static A: [&[&str]; 2] = [&parser::A, &coder::A];
}

mod _d {
    mod parser {
        const BAD_IDENTIFIER_CHARACTERS: &str = "Identifier contains unsupported characters.";
        const BAD_IDENTIFIER_CASE: &str = "Identifier is specified in an unsupported case.";
        const BAD_KEYWORD_TYPE: &str = "Specification keyword is not a String.";
        const BAD_MODULE_IDENTIFIER: &str = "Identifier is not valid on module-level.";
        const BAD_NAME: &str = "Invalid name.";
        const BAD_OBJECT_ATTRIBUTE: &str = "An object attribute is invalid.";
        const BAD_SPEC_FILE_EXTENSION: &str =
            "Specification filename extension is not supported or is missing.";
        const BAD_TOML: &str = "TOML deserialization has failed.";
        const BAD_ROOT_LEVEL_KEYWORD: &str =
            "Specification contains an invalid root-level keyword.";
        const BAD_VALUE_TYPE: &str = "Specification value type is invalid.";
        const BAD_YAML: &str = "YAML deserialization has failed.";
        const EMPTY_IDENTIFIER: &str = "An identifier cannot be an empty string.";
        const EMPTY_LIST: &str = "Empty list of objects is not allowed.";
        const FAILED_TO_OPEN_SPEC_FILE: &str = "Specification file couldn't be opened.";
        const MISSING_ATTRIBUTE: &str = "Specification lacks a mandatory attribute.";
        const MUTUALLY_EXCLUSIVE_KEYWORDS: &str =
            "Specification contains mutually exclusive keywords.";
        const NON_UNIQUE_NAME: &str = "A name is not unique.";
        const SPEC_FILE_NOT_FOUND: &str = "Specification file couldn't be found.";
        pub static A: [&str; 18] = [
            BAD_IDENTIFIER_CHARACTERS,
            BAD_IDENTIFIER_CASE,
            BAD_KEYWORD_TYPE,
            BAD_MODULE_IDENTIFIER,
            BAD_NAME,
            BAD_OBJECT_ATTRIBUTE,
            BAD_SPEC_FILE_EXTENSION,
            BAD_TOML,
            BAD_ROOT_LEVEL_KEYWORD,
            BAD_VALUE_TYPE,
            BAD_YAML,
            EMPTY_IDENTIFIER,
            EMPTY_LIST,
            FAILED_TO_OPEN_SPEC_FILE,
            MISSING_ATTRIBUTE,
            MUTUALLY_EXCLUSIVE_KEYWORDS,
            NON_UNIQUE_NAME,
            SPEC_FILE_NOT_FOUND,
        ];
    }

    mod coder {
        const CATEGORY_REQUIRED: &str = "At least one category must be defined.";
        const ERROR_REQUIRED: &str = "At least one error must be defined.";
        const FAILED_TO_PARSE_TOKENS: &str = "Generated code tokens couldn't be parsed.";
        const FAILED_TO_READ_OUTPUT_FILE: &str = "Output file couldn't be read.";
        const FAILED_TO_WRITE_OUTPUT_FILE: &str = "Output file couldn't be written.";
        const RUSTFMT_FAILED: &str = "Rustfmt tool exited with an error.";
        const TOO_MANY_BITS: &str =
            "The number of required bits exceeds the largest supported type u64.";
        pub static A: [&str; 7] = [
            CATEGORY_REQUIRED,
            ERROR_REQUIRED,
            FAILED_TO_PARSE_TOKENS,
            FAILED_TO_READ_OUTPUT_FILE,
            FAILED_TO_WRITE_OUTPUT_FILE,
            RUSTFMT_FAILED,
            TOO_MANY_BITS,
        ];
    }

    pub static A: [&[&str]; 2] = [&parser::A, &coder::A];
}

mod _p {
    pub type R = u8;
    pub const KIND_BITS: usize = 6;
    pub const CAT_BITS: usize = 1;
    pub const CAT_MASK: R = 1;
    pub const CAT_MAX: R = 1;
    pub static VAR_MAXES: [R; 2] = [17, 6];
    const _: () = assert!(KIND_BITS <= R::BITS as usize);
    const _: () = assert!(CAT_BITS <= usize::BITS as usize);
    pub(super) struct Ident<'a>(pub(super) &'a str);
    impl<'a> core::fmt::Debug for Ident<'a> {
        #[inline]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.pad(self.0)
        }
    }
}

/// Error category constants.
pub mod categories {
    use super::TbErrorCategory as C;

    /// Parser errors category.
    pub const PARSER: C = C::new(0);

    /// Coder errors category.
    pub const CODER: C = C::new(1);
}

/// Error kind constants.
pub mod kinds {
    use super::categories as c;
    use super::TbErrorKind as EK;

    /// Parser category error kind constants.
    pub mod parser {
        use super::c;
        use super::EK;

        /// Identifier contains unsupported characters.
        pub const BAD_IDENTIFIER_CHARACTERS: EK = EK::new(c::PARSER, 0);

        /// Identifier is specified in an unsupported case.
        pub const BAD_IDENTIFIER_CASE: EK = EK::new(c::PARSER, 1);

        /// Specification keyword is not a String.
        pub const BAD_KEYWORD_TYPE: EK = EK::new(c::PARSER, 2);

        /// Identifier is not valid on module-level.
        pub const BAD_MODULE_IDENTIFIER: EK = EK::new(c::PARSER, 3);

        /// Invalid name.
        pub const BAD_NAME: EK = EK::new(c::PARSER, 4);

        /// An object attribute is invalid.
        pub const BAD_OBJECT_ATTRIBUTE: EK = EK::new(c::PARSER, 5);

        /// Specification filename extension is not supported or is missing.
        pub const BAD_SPEC_FILE_EXTENSION: EK = EK::new(c::PARSER, 6);

        /// TOML deserialization has failed.
        pub const BAD_TOML: EK = EK::new(c::PARSER, 7);

        /// Specification contains an invalid root-level keyword.
        pub const BAD_ROOT_LEVEL_KEYWORD: EK = EK::new(c::PARSER, 8);

        /// Specification value type is invalid.
        pub const BAD_VALUE_TYPE: EK = EK::new(c::PARSER, 9);

        /// YAML deserialization has failed.
        pub const BAD_YAML: EK = EK::new(c::PARSER, 10);

        /// An identifier cannot be an empty string.
        pub const EMPTY_IDENTIFIER: EK = EK::new(c::PARSER, 11);

        /// Empty list of objects is not allowed.
        pub const EMPTY_LIST: EK = EK::new(c::PARSER, 12);

        /// Specification file couldn't be opened.
        pub const FAILED_TO_OPEN_SPEC_FILE: EK = EK::new(c::PARSER, 13);

        /// Specification lacks a mandatory attribute.
        pub const MISSING_ATTRIBUTE: EK = EK::new(c::PARSER, 14);

        /// Specification contains mutually exclusive keywords.
        pub const MUTUALLY_EXCLUSIVE_KEYWORDS: EK = EK::new(c::PARSER, 15);

        /// A name is not unique.
        pub const NON_UNIQUE_NAME: EK = EK::new(c::PARSER, 16);

        /// Specification file couldn't be found.
        pub const SPEC_FILE_NOT_FOUND: EK = EK::new(c::PARSER, 17);
    }

    /// Coder category error kind constants.
    pub mod coder {
        use super::c;
        use super::EK;

        /// At least one category must be defined.
        pub const CATEGORY_REQUIRED: EK = EK::new(c::CODER, 0);

        /// At least one error must be defined.
        pub const ERROR_REQUIRED: EK = EK::new(c::CODER, 1);

        /// Generated code tokens couldn't be parsed.
        pub const FAILED_TO_PARSE_TOKENS: EK = EK::new(c::CODER, 2);

        /// Output file couldn't be read.
        pub const FAILED_TO_READ_OUTPUT_FILE: EK = EK::new(c::CODER, 3);

        /// Output file couldn't be written.
        pub const FAILED_TO_WRITE_OUTPUT_FILE: EK = EK::new(c::CODER, 4);

        /// Rustfmt tool exited with an error.
        pub const RUSTFMT_FAILED: EK = EK::new(c::CODER, 5);

        /// The number of required bits exceeds the largest supported type u64.
        pub const TOO_MANY_BITS: EK = EK::new(c::CODER, 6);
    }
}
