
/**
 * Error category type.
 *
 * See the [category] module for category constants.
 */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct ErrorCategory(_p::R);

impl ErrorCategory {
    #[inline]
    const fn new(v: _p::R) -> Self {
        Self(v)
    }


    /// Returns the name of the error category.
    #[inline]
    pub fn name(&self) -> &'static str {
        _cn::A[self.0 as usize]
    }

}


impl tighterror::Category for ErrorCategory {
    type R = _p::R;
    const BITS: usize = _p::CAT_BITS;

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }

}


impl core::fmt::Display for ErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }

}


impl core::fmt::Debug for ErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ErrorCategory").field(&_p::Ident(self.name())).finish()
    }

}


/**
 * Error kind type.
 *
 * See the [kind] module for error kind constants.
 */

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct ErrorKind(_p::R);

impl ErrorKind {
    const fn new(cat: ErrorCategory, variant: _p::R) -> Self {
        Self(cat.0 << _p::VAR_BITS | variant)
    }


    #[inline]
    fn category_value(&self) -> _p::R {
        (self.0 & _p::CAT_MASK) >> _p::VAR_BITS
    }


    #[inline]
    fn variant_value(&self) -> _p::R {
        self.0 & _p::VAR_MASK
    }


    /// Returns the error category.
    #[inline]
    pub fn category(&self) -> ErrorCategory {
        ErrorCategory::new(self.category_value())
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
        let cat = (value & _p::CAT_MASK) >> _p::VAR_BITS;
        let variant = value & _p::VAR_MASK;
        if cat <= _p::CAT_MAX && variant <= _p::VAR_MAXES[cat as usize] {
            Some(Self::new(ErrorCategory::new(cat), variant))
        } else {
            None
        }

    }

}


impl tighterror::Kind for ErrorKind {
    type R = _p::R;
    type Category = ErrorCategory;
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


impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }

}


impl core::fmt::Debug for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ErrorKind")
            .field("cat", &_p::Ident(self.category().name()))
            .field("var", &_p::Ident(self.name()))
            .field("val", &self.0)
            .finish()
    }

}


impl<T> core::convert::From<ErrorKind> for Result<T, Error> {
    #[inline]
    fn from(v: ErrorKind) -> Self {
        Err(v.into())
    }

}


/**
 * Error type.
 *
 * See the [kind] module for error kind constants.
 */

#[derive(Debug)]
#[repr(transparent)]
pub struct Error(ErrorKind);

impl Error {
    /// Returns the error kind.
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.0
    }


    /// Returns the error category.
    #[inline]
    pub fn category(&self) -> ErrorCategory {
        self.0.category()
    }


    /// Returns the error origin location.
    #[inline]
    pub fn location(&self) -> tighterror::Location {
        tighterror::Location::undefined()
    }

}


impl tighterror::Error for Error {
    type R = _p::R;
    type Category = ErrorCategory;
    type Kind = ErrorKind;

    #[inline]
    fn kind(&self) -> Self::Kind {
        self.kind()
    }


    #[inline]
    fn location(&self) -> tighterror::Location {
        self.location()
    }

}


impl core::convert::From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Self(kind)
    }

}


impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.kind().display())
    }

}


impl core::cmp::PartialEq for Error {
    /// Checks equality based on the error kind only.
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        self.0 == other.0
    }

}


impl<T> core::convert::From<Error> for core::result::Result<T, Error> {
    #[inline]
    fn from(err: Error) -> Self {
        Err(err)
    }

}


impl core::error::Error for Error {}

mod _cn {
    pub const PARSER: &str = "PARSER";
    pub const PROCESSOR: &str = "PROCESSOR";
    pub static A: [&str; 2] = [PARSER, PROCESSOR];
}

mod _n {
    pub(crate) mod parser {
        pub(crate) const BAD_FILE: &str = "BAD_FILE";
        pub static A: [&str; 1] = [BAD_FILE];
    }

    pub(crate) mod processor {
        pub(crate) const BAD_FILE: &str = "BAD_FILE";
        pub(crate) const BAD_STATE: &str = "BAD_STATE";
        pub static A: [&str; 2] = [BAD_FILE, BAD_STATE];
    }

    pub static A: [&[&str]; 2] = [&parser::A, &processor::A];
}

mod _d {
    pub(crate) mod parser {
        pub(crate) const BAD_FILE: &str = "Configuration file is malformed.";
        pub static A: [&str; 1] = [BAD_FILE];
    }

    pub(crate) mod processor {
        pub(crate) const BAD_FILE: &str = "Processor file is malformed or missing.";
        pub(crate) const BAD_STATE: &str = "Internal error, state mismatch.";
        pub static A: [&str; 2] = [BAD_FILE, BAD_STATE];
    }

    pub static A: [&[&str]; 2] = [&parser::A, &processor::A];
}

mod _p {
    pub const CAT_BITS: usize = 1;
    pub const VAR_BITS: usize = 1;
    pub const KIND_BITS: usize = CAT_BITS + VAR_BITS;
    pub type R = u8;
    pub const CAT_MAX: R = 1;
    pub const VAR_MASK: R = R::MAX >> (R::BITS as usize - VAR_BITS);
    pub static VAR_MAXES: [R; 2] = [0, 1];
    pub const CAT_MASK: R = (R::MAX >> (R::BITS as usize - CAT_BITS)) << VAR_BITS;
    const _: () = assert!(KIND_BITS <= R::BITS as usize);
    const _: () = assert!(CAT_BITS <= usize::BITS as usize);
    pub(super) struct Ident<'a>(pub(super) &'a str);

    impl core::fmt::Debug for Ident<'_> {
        #[inline]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.pad(self.0)
        }

    }

}


/// Error category constants.
pub mod category {
    use super::ErrorCategory as C;

    /// Parser errors category.
    pub const PARSER: C = C::new(0);

    /// Processor errors category.
    pub const PROCESSOR: C = C::new(1);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;

    /// Parser category error kind constants.
    pub mod parser {
        use super::c;
        use super::EK;

        /// Configuration file is malformed.
        pub const BAD_FILE: EK = EK::new(c::PARSER, 0);
    }


    /// Processor category error kind constants.
    pub mod processor {
        use super::c;
        use super::EK;

        /// Processor file is malformed or missing.
        pub const BAD_FILE: EK = EK::new(c::PROCESSOR, 0);

        /// Internal error, state mismatch.
        pub const BAD_STATE: EK = EK::new(c::PROCESSOR, 1);
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(PARSER.name(), "PARSER");
        assert_eq!(tighterror::Category::name(& PARSER), "PARSER");
        assert_eq!(PROCESSOR.name(), "PROCESSOR");
        assert_eq!(tighterror::Category::name(& PROCESSOR), "PROCESSOR")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", PARSER), "PARSER");
        assert_eq!(format!("{}", PROCESSOR), "PROCESSOR");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [ErrorCategory; 2] = [PARSER, PROCESSOR];
        let set = HashSet::<ErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 2);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [ErrorCategory; 2] = [PARSER, PROCESSOR];
        for c in cats {
            assert!(c.0 <= 1);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(parser::BAD_FILE.name(), "BAD_FILE");
        assert_eq!(tighterror::Kind::name(& parser::BAD_FILE), "BAD_FILE");
        assert_eq!(processor::BAD_FILE.name(), "BAD_FILE");
        assert_eq!(tighterror::Kind::name(& processor::BAD_FILE), "BAD_FILE");
        assert_eq!(processor::BAD_STATE.name(), "BAD_STATE");
        assert_eq!(tighterror::Kind::name(& processor::BAD_STATE), "BAD_STATE");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", parser::BAD_FILE), "BAD_FILE");
        assert_eq!(format!("{}", processor::BAD_FILE), "BAD_FILE");
        assert_eq!(format!("{}", processor::BAD_STATE), "BAD_STATE");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [
            parser::BAD_FILE,
            processor::BAD_FILE,
            processor::BAD_STATE,
        ];

        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [
            parser::BAD_FILE,
            processor::BAD_FILE,
            processor::BAD_STATE,
        ];

        let set = HashSet::<_p::R>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(parser::BAD_FILE.category(), category::PARSER);
        assert_eq!(processor::BAD_FILE.category(), category::PROCESSOR);
        assert_eq!(processor::BAD_STATE.category(), category::PROCESSOR);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(
            ErrorKind::from_value(parser::BAD_FILE.value()).unwrap(), parser::BAD_FILE
        );
        assert_eq!(
            ErrorKind::from_value(processor::BAD_FILE.value()).unwrap(),
            processor::BAD_FILE
        );
        assert_eq!(
            ErrorKind::from_value(processor::BAD_STATE.value()).unwrap(),
            processor::BAD_STATE
        );
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(
            format!("{}", Error::from(parser::BAD_FILE)),
            "Configuration file is malformed."
        );
        assert_eq!(
            format!("{}", Error::from(processor::BAD_FILE)),
            "Processor file is malformed or missing."
        );
        assert_eq!(
            format!("{}", Error::from(processor::BAD_STATE)),
            "Internal error, state mismatch."
        );
    }

}

