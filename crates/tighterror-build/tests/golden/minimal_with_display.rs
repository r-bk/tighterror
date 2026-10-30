
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
        assert!(cat.0 == _p::CAT_MAX);
        Self(variant)
    }


    #[inline]
    fn category_value(&self) -> _p::R {
        _p::CAT_MAX
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
        if value <= _p::VAR_MAXES[0] {
            Some(Self::new(ErrorCategory::new(0), value))
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
    pub const GENERAL: &str = "GENERAL";
    pub static A: [&str; 1] = [GENERAL];
}

mod _n {
    pub(crate) mod general {
        pub(crate) const BAD_CONF_FILE: &str = "BAD_CONF_FILE";
        pub(crate) const BAD_ARG: &str = "BAD_ARG";
        pub(crate) const IO_ERROR: &str = "IO_ERROR";
        pub static A: [&str; 3] = [BAD_CONF_FILE, BAD_ARG, IO_ERROR];
    }

    pub static A: [&[&str]; 1] = [&general::A];
}

mod _d {
    pub(crate) mod general {
        pub(crate) const BAD_CONF_FILE: &str = "Configuration file is malformed.";
        pub(crate) const BAD_ARG: &str = "Invalid argument.";
        pub(crate) const IO_ERROR: &str = "An IO error occurred.";
        pub static A: [&str; 3] = [BAD_CONF_FILE, BAD_ARG, IO_ERROR];
    }

    pub static A: [&[&str]; 1] = [&general::A];
}

mod _p {
    pub const CAT_BITS: usize = 0;
    pub const VAR_BITS: usize = 2;
    pub const KIND_BITS: usize = CAT_BITS + VAR_BITS;
    pub type R = u8;
    pub const CAT_MAX: R = 0;
    pub const VAR_MASK: R = R::MAX >> (R::BITS as usize - VAR_BITS);
    pub static VAR_MAXES: [R; 1] = [2];
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
    pub const GENERAL: C = C::new(0);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;

    /// Configuration file is malformed.
    pub const BAD_CONF_FILE: EK = EK::new(c::GENERAL, 0);

    /// Invalid argument.
    pub const BAD_ARG: EK = EK::new(c::GENERAL, 1);

    /// An IO error occurred.
    pub const IO_ERROR: EK = EK::new(c::GENERAL, 2);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(GENERAL.name(), "GENERAL");
        assert_eq!(tighterror::Category::name(& GENERAL), "GENERAL")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", GENERAL), "GENERAL");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [ErrorCategory; 1] = [GENERAL];
        let set = HashSet::<ErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 1);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [ErrorCategory; 1] = [GENERAL];
        for c in cats {
            assert!(c.0 == 0);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(BAD_CONF_FILE.name(), "BAD_CONF_FILE");
        assert_eq!(tighterror::Kind::name(& BAD_CONF_FILE), "BAD_CONF_FILE");
        assert_eq!(BAD_ARG.name(), "BAD_ARG");
        assert_eq!(tighterror::Kind::name(& BAD_ARG), "BAD_ARG");
        assert_eq!(IO_ERROR.name(), "IO_ERROR");
        assert_eq!(tighterror::Kind::name(& IO_ERROR), "IO_ERROR");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", BAD_CONF_FILE), "BAD_CONF_FILE");
        assert_eq!(format!("{}", BAD_ARG), "BAD_ARG");
        assert_eq!(format!("{}", IO_ERROR), "IO_ERROR");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [BAD_CONF_FILE, BAD_ARG, IO_ERROR];
        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [BAD_CONF_FILE, BAD_ARG, IO_ERROR];
        let set = HashSet::<_p::R>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(BAD_CONF_FILE.category(), category::GENERAL);
        assert_eq!(BAD_ARG.category(), category::GENERAL);
        assert_eq!(IO_ERROR.category(), category::GENERAL);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(ErrorKind::from_value(BAD_CONF_FILE.value()).unwrap(), BAD_CONF_FILE);
        assert_eq!(ErrorKind::from_value(BAD_ARG.value()).unwrap(), BAD_ARG);
        assert_eq!(ErrorKind::from_value(IO_ERROR.value()).unwrap(), IO_ERROR);
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(
            format!("{}", Error::from(BAD_CONF_FILE)), "Configuration file is malformed."
        );
        assert_eq!(format!("{}", Error::from(BAD_ARG)), "Invalid argument.");
        assert_eq!(format!("{}", Error::from(IO_ERROR)), "An IO error occurred.");
    }

}

