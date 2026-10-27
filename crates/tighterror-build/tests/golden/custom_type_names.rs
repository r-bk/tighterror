
/// A custom category doc.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IcErrorCategory(_p::R);

impl IcErrorCategory {
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


impl tighterror::Category for IcErrorCategory {
    type R = _p::R;
    const BITS: usize = _p::CAT_BITS;

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }

}


impl core::fmt::Display for IcErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }

}


impl core::fmt::Debug for IcErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IcErrorCategory").field(&_p::Ident(self.name())).finish()
    }

}


/// A custom error doc.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IcErrorKind(_p::R);

impl IcErrorKind {
    const fn new(cat: IcErrorCategory, variant: _p::R) -> Self {
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
    pub fn category(&self) -> IcErrorCategory {
        IcErrorCategory::new(self.category_value())
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
            Some(Self::new(IcErrorCategory::new(0), value))
        } else {
            None
        }

    }

}


impl tighterror::Kind for IcErrorKind {
    type R = _p::R;
    type Category = IcErrorCategory;
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


impl core::fmt::Display for IcErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }

}


impl core::fmt::Debug for IcErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IcErrorKind")
            .field("cat", &_p::Ident(self.category().name()))
            .field("var", &_p::Ident(self.name()))
            .field("val", &self.0)
            .finish()
    }

}


/// A custom error struct doc.
#[derive(Debug)]
#[repr(transparent)]
pub struct IcError(IcErrorKind);

impl IcError {
    /// Returns the error kind.
    #[inline]
    pub fn kind(&self) -> IcErrorKind {
        self.0
    }


    /// Returns the error category.
    #[inline]
    pub fn category(&self) -> IcErrorCategory {
        self.0.category()
    }


    /// Returns the error origin location.
    #[inline]
    pub fn location(&self) -> tighterror::Location {
        tighterror::Location::undefined()
    }

}


impl tighterror::Error for IcError {
    type R = _p::R;
    type Category = IcErrorCategory;
    type Kind = IcErrorKind;

    #[inline]
    fn kind(&self) -> Self::Kind {
        self.kind()
    }


    #[inline]
    fn location(&self) -> tighterror::Location {
        self.location()
    }

}


impl core::convert::From<IcErrorKind> for IcError {
    #[inline]
    fn from(kind: IcErrorKind) -> Self {
        Self(kind)
    }

}


impl core::fmt::Display for IcError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.kind().display())
    }

}


impl core::cmp::PartialEq for IcError {
    /// Checks equality based on the error kind only.
    #[inline]
    fn eq(&self, other: &IcError) -> bool {
        self.0 == other.0
    }

}

mod _cn {
    pub const GENERAL: &str = "GENERAL";
    pub static A: [&str; 1] = [GENERAL];
}

mod _n {
    pub(crate) mod general {
        pub(crate) const FAILED_TO_OPEN_FILE: &str = "FAILED_TO_OPEN_FILE";
        pub(crate) const BAD_INPUT: &str = "BAD_INPUT";
        pub(crate) const LONG_DESCRIPTION: &str = "LONG_DESCRIPTION";
        pub static A: [&str; 3] = [FAILED_TO_OPEN_FILE, BAD_INPUT, LONG_DESCRIPTION];
    }

    pub static A: [&[&str]; 1] = [&general::A];
}

mod _d {
    pub(crate) mod general {
        use super::super::_n;
        pub(crate) const FAILED_TO_OPEN_FILE: &str = "Input file could not be open.";
        pub(crate) const BAD_INPUT: &str = "Input file has invalid format.";
        const LONG_DESCRIPTION: &str = _n::general::LONG_DESCRIPTION;
        pub static A: [&str; 3] = [FAILED_TO_OPEN_FILE, BAD_INPUT, LONG_DESCRIPTION];
    }

    pub static A: [&[&str]; 1] = [&general::A];
}

mod _p {
    pub type R = u8;
    pub const KIND_BITS: usize = 2;
    pub const CAT_BITS: usize = 0;
    pub const CAT_MAX: R = 0;
    pub const VAR_MASK: R = 3;
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
    use super::IcErrorCategory as C;

    /// General error category.
    pub const GENERAL: C = C::new(0);
}


/// Error kind constants.
pub mod kind {
    use super::IcErrorKind as EK;
    use super::category as c;

    /// Input file could not be open.
    pub const FAILED_TO_OPEN_FILE: EK = EK::new(c::GENERAL, 0);

    /// Input file has invalid format.
    pub const BAD_INPUT: EK = EK::new(c::GENERAL, 1);

    /**
 * This is an example for a long doc string.
 *
 * It continues on several lines. And can include Rust doc-string
 * formatting [`String`].
 */
    pub const LONG_DESCRIPTION: EK = EK::new(c::GENERAL, 2);
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
        let cats: [IcErrorCategory; 1] = [GENERAL];
        let set = HashSet::<IcErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 1);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [IcErrorCategory; 1] = [GENERAL];
        for c in cats {
            assert!(c.0 == 0);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(FAILED_TO_OPEN_FILE.name(), "FAILED_TO_OPEN_FILE");
        assert_eq!(tighterror::Kind::name(& FAILED_TO_OPEN_FILE), "FAILED_TO_OPEN_FILE");
        assert_eq!(BAD_INPUT.name(), "BAD_INPUT");
        assert_eq!(tighterror::Kind::name(& BAD_INPUT), "BAD_INPUT");
        assert_eq!(LONG_DESCRIPTION.name(), "LONG_DESCRIPTION");
        assert_eq!(tighterror::Kind::name(& LONG_DESCRIPTION), "LONG_DESCRIPTION");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", FAILED_TO_OPEN_FILE), "FAILED_TO_OPEN_FILE");
        assert_eq!(format!("{}", BAD_INPUT), "BAD_INPUT");
        assert_eq!(format!("{}", LONG_DESCRIPTION), "LONG_DESCRIPTION");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [IcErrorKind; 3] = [FAILED_TO_OPEN_FILE, BAD_INPUT, LONG_DESCRIPTION];
        let set = HashSet::<IcErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [IcErrorKind; 3] = [FAILED_TO_OPEN_FILE, BAD_INPUT, LONG_DESCRIPTION];
        let set = HashSet::<u8>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(FAILED_TO_OPEN_FILE.category(), category::GENERAL);
        assert_eq!(BAD_INPUT.category(), category::GENERAL);
        assert_eq!(LONG_DESCRIPTION.category(), category::GENERAL);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(
            IcErrorKind::from_value(FAILED_TO_OPEN_FILE.value()).unwrap(),
            FAILED_TO_OPEN_FILE
        );
        assert_eq!(IcErrorKind::from_value(BAD_INPUT.value()).unwrap(), BAD_INPUT);
        assert_eq!(
            IcErrorKind::from_value(LONG_DESCRIPTION.value()).unwrap(), LONG_DESCRIPTION
        );
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(
            format!("{}", IcError::from(FAILED_TO_OPEN_FILE)),
            "Input file could not be open."
        );
        assert_eq!(
            format!("{}", IcError::from(BAD_INPUT)), "Input file has invalid format."
        );
        assert_eq!(format!("{}", IcError::from(LONG_DESCRIPTION)), "LONG_DESCRIPTION");
    }

}

