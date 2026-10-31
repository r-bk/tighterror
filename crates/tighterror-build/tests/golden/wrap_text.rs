
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
pub struct ErrorKind(core::num::NonZero<_p::R>);

impl ErrorKind {
    const fn new(cat: ErrorCategory, variant: _p::R) -> Self {
        Self(core::num::NonZero::new(cat.0 << _p::VAR_BITS | variant).unwrap())
    }


    #[inline]
    fn category_value(&self) -> _p::R {
        (self.value() & _p::CAT_MASK) >> _p::VAR_BITS
    }


    #[inline]
    fn variant_value(&self) -> _p::R {
        self.value() & _p::VAR_MASK
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
        self.0.get()
    }


    /// Creates an error kind from a raw value of the underlying Rust type.
    #[inline]
    pub fn from_value(value: _p::R) -> Option<Self> {
        let cat = value >> _p::VAR_BITS;
        let variant = value & _p::VAR_MASK;
        if cat <= _p::CAT_MAX && variant != 0 && variant <= _p::VAR_MAXES[cat as usize] {
            core::num::NonZero::new(value).map(Self)
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
    pub const GLYPH: &str = "GLYPH";
    pub const LAYOUT: &str = "LAYOUT";
    pub static A: [&str; 2] = [GLYPH, LAYOUT];
}

mod _n {
    pub(crate) mod glyph {
        pub(crate) const BAD_GLYPH: &str = "BAD_GLYPH";
        pub(crate) const NO_FONT: &str = "NO_FONT";
        pub static A: [&str; 3] = ["", BAD_GLYPH, NO_FONT];
    }

    pub(crate) mod layout {
        pub(crate) const OVERFLOW: &str = "OVERFLOW";
        pub static A: [&str; 2] = ["", OVERFLOW];
    }

    pub static A: [&[&str]; 2] = [&glyph::A, &layout::A];
}

mod _d {
    pub(crate) mod glyph {
        pub(crate) const BAD_GLYPH: &str = "Glyph is invalid.";
        pub(crate) const NO_FONT: &str = "Font is missing.";
        pub static A: [&str; 3] = ["", BAD_GLYPH, NO_FONT];
    }

    pub(crate) mod layout {
        pub(crate) const OVERFLOW: &str = "Line overflows.";
        pub static A: [&str; 2] = ["", OVERFLOW];
    }

    pub static A: [&[&str]; 2] = [&glyph::A, &layout::A];
}

mod _p {
    pub const CAT_BITS: usize = 1;
    pub const VAR_BITS: usize = 2;
    pub const KIND_BITS: usize = CAT_BITS + VAR_BITS;
    pub type R = u64;
    pub const CAT_MAX: R = 1;
    pub const VAR_MASK: R = R::MAX >> (R::BITS as usize - VAR_BITS);
    pub static VAR_MAXES: [R; 2] = [2, 1];
    pub const CAT_MASK: R = (R::MAX >> (R::BITS as usize - CAT_BITS)) << VAR_BITS;
    const _: () = assert!(
        KIND_BITS <= R::BITS as usize,
        "module `errors`: `repr u64` holds fewer bits than the error kinds need"
    );
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

    /// Glyph errors category.
    pub const GLYPH: C = C::new(0);

    /// Layout errors category.
    pub const LAYOUT: C = C::new(1);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;

    /// Glyph is invalid.
    pub const BAD_GLYPH: EK = EK::new(c::GLYPH, 1);

    /// Font is missing.
    pub const NO_FONT: EK = EK::new(c::GLYPH, 2);

    /// Line overflows.
    pub const OVERFLOW: EK = EK::new(c::LAYOUT, 1);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(GLYPH.name(), "GLYPH");
        assert_eq!(tighterror::Category::name(& GLYPH), "GLYPH");
        assert_eq!(LAYOUT.name(), "LAYOUT");
        assert_eq!(tighterror::Category::name(& LAYOUT), "LAYOUT")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", GLYPH), "GLYPH");
        assert_eq!(format!("{}", LAYOUT), "LAYOUT");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [ErrorCategory; 2] = [GLYPH, LAYOUT];
        let set = HashSet::<ErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 2);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [ErrorCategory; 2] = [GLYPH, LAYOUT];
        for c in cats {
            assert!(c.0 <= 1);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(BAD_GLYPH.name(), "BAD_GLYPH");
        assert_eq!(tighterror::Kind::name(& BAD_GLYPH), "BAD_GLYPH");
        assert_eq!(NO_FONT.name(), "NO_FONT");
        assert_eq!(tighterror::Kind::name(& NO_FONT), "NO_FONT");
        assert_eq!(OVERFLOW.name(), "OVERFLOW");
        assert_eq!(tighterror::Kind::name(& OVERFLOW), "OVERFLOW");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", BAD_GLYPH), "BAD_GLYPH");
        assert_eq!(format!("{}", NO_FONT), "NO_FONT");
        assert_eq!(format!("{}", OVERFLOW), "OVERFLOW");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [BAD_GLYPH, NO_FONT, OVERFLOW];
        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 3] = [BAD_GLYPH, NO_FONT, OVERFLOW];
        let set = HashSet::<_p::R>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(BAD_GLYPH.category(), category::GLYPH);
        assert_eq!(NO_FONT.category(), category::GLYPH);
        assert_eq!(OVERFLOW.category(), category::LAYOUT);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(ErrorKind::from_value(BAD_GLYPH.value()).unwrap(), BAD_GLYPH);
        assert_eq!(ErrorKind::from_value(NO_FONT.value()).unwrap(), NO_FONT);
        assert_eq!(ErrorKind::from_value(OVERFLOW.value()).unwrap(), OVERFLOW);
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(format!("{}", Error::from(BAD_GLYPH)), "Glyph is invalid.");
        assert_eq!(format!("{}", Error::from(NO_FONT)), "Font is missing.");
        assert_eq!(format!("{}", Error::from(OVERFLOW)), "Line overflows.");
    }

}

