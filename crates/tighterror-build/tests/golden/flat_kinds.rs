
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


impl std::error::Error for Error {}

mod _cn {
    pub const GENERAL: &str = "GENERAL";
    pub const IO: &str = "IO";
    pub static A: [&str; 2] = [GENERAL, IO];
}

mod _n {
    pub(crate) mod general {
        pub(crate) const TIMEOUT: &str = "TIMEOUT";
        pub(crate) const QUEUE_FULL: &str = "QUEUE_FULL";
        pub static A: [&str; 2] = [TIMEOUT, QUEUE_FULL];
    }

    pub(crate) mod io {
        pub(crate) const CONNECTION_REFUSED: &str = "CONNECTION_REFUSED";
        pub static A: [&str; 1] = [CONNECTION_REFUSED];
    }

    pub static A: [&[&str]; 2] = [&general::A, &io::A];
}

mod _d {
    pub(crate) mod general {
        pub(crate) const TIMEOUT: &str = "Operation timed out.";
        pub(crate) const QUEUE_FULL: &str = "Queue is full.";
        pub static A: [&str; 2] = [TIMEOUT, QUEUE_FULL];
    }

    pub(crate) mod io {
        pub(crate) const CONNECTION_REFUSED: &str = "Outgoing network connection was refused.";
        pub static A: [&str; 1] = [CONNECTION_REFUSED];
    }

    pub static A: [&[&str]; 2] = [&general::A, &io::A];
}

mod _p {
    pub type R = u8;
    pub const KIND_BITS: usize = 2;
    pub const CAT_BITS: usize = 1;
    pub const CAT_MAX: R = 1;
    pub const VAR_MASK: R = 1;
    pub static VAR_MAXES: [R; 2] = [1, 0];
    pub const CAT_MASK: R = 2;
    pub const VAR_BITS: usize = 1;
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

    /// General errors category.
    pub const GENERAL: C = C::new(0);

    /// I/O errors category.
    pub const IO: C = C::new(1);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;

    /// Operation timed out.
    pub const TIMEOUT: EK = EK::new(c::GENERAL, 0);

    /// Queue is full.
    pub const QUEUE_FULL: EK = EK::new(c::GENERAL, 1);

    /// Outgoing network connection was refused.
    pub const CONNECTION_REFUSED: EK = EK::new(c::IO, 0);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(GENERAL.name(), "GENERAL");
        assert_eq!(tighterror::Category::name(& GENERAL), "GENERAL");
        assert_eq!(IO.name(), "IO");
        assert_eq!(tighterror::Category::name(& IO), "IO")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", GENERAL), "GENERAL");
        assert_eq!(format!("{}", IO), "IO");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [ErrorCategory; 2] = [GENERAL, IO];
        let set = HashSet::<ErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 2);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [ErrorCategory; 2] = [GENERAL, IO];
        for c in cats {
            assert!(c.0 <= 1);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(TIMEOUT.name(), "TIMEOUT");
        assert_eq!(tighterror::Kind::name(& TIMEOUT), "TIMEOUT");
        assert_eq!(QUEUE_FULL.name(), "QUEUE_FULL");
        assert_eq!(tighterror::Kind::name(& QUEUE_FULL), "QUEUE_FULL");
        assert_eq!(CONNECTION_REFUSED.name(), "CONNECTION_REFUSED");
        assert_eq!(tighterror::Kind::name(& CONNECTION_REFUSED), "CONNECTION_REFUSED");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", TIMEOUT), "TIMEOUT");
        assert_eq!(format!("{}", QUEUE_FULL), "QUEUE_FULL");
        assert_eq!(format!("{}", CONNECTION_REFUSED), "CONNECTION_REFUSED");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [TIMEOUT, QUEUE_FULL, CONNECTION_REFUSED];
        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [TIMEOUT, QUEUE_FULL, CONNECTION_REFUSED];
        let set = HashSet::<u8>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(TIMEOUT.category(), category::GENERAL);
        assert_eq!(QUEUE_FULL.category(), category::GENERAL);
        assert_eq!(CONNECTION_REFUSED.category(), category::IO);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(ErrorKind::from_value(TIMEOUT.value()).unwrap(), TIMEOUT);
        assert_eq!(ErrorKind::from_value(QUEUE_FULL.value()).unwrap(), QUEUE_FULL);
        assert_eq!(
            ErrorKind::from_value(CONNECTION_REFUSED.value()).unwrap(),
            CONNECTION_REFUSED
        );
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(format!("{}", Error::from(TIMEOUT)), "Operation timed out.");
        assert_eq!(format!("{}", Error::from(QUEUE_FULL)), "Queue is full.");
        assert_eq!(
            format!("{}", Error::from(CONNECTION_REFUSED)),
            "Outgoing network connection was refused."
        );
    }

}

