
/**
 * Error category type.
 *
 * See the [categories] module for category constants.
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
 * See the [kinds] module for error kind constants.
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
 * See the [kinds] module for error kind constants.
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
    pub const CAT_ONE: &str = "CAT_ONE";
    pub const CAT_TWO: &str = "CAT_TWO";
    pub static A: [&str; 2] = [CAT_ONE, CAT_TWO];
}

mod _n {
    pub(crate) mod cat_one {
        pub(crate) const CAT_ONE_ERR_ONE: &str = "CAT_ONE_ERR_ONE";
        pub(crate) const CAT_ONE_ERR_TWO: &str = "CAT_ONE_ERR_TWO";
        pub static A: [&str; 2] = [CAT_ONE_ERR_ONE, CAT_ONE_ERR_TWO];
    }

    pub(crate) mod cat_two {
        pub(crate) const CAT_TWO_ERR_ONE: &str = "CAT_TWO_ERR_ONE";
        pub(crate) const CAT_TWO_ERR_TWO: &str = "CAT_TWO_ERR_TWO";
        pub(crate) const CAT_TWO_CUSTOM_NAME: &str = "CAT_TWO_CUSTOM_NAME";
        pub static A: [&str; 3] = [
            CAT_TWO_ERR_ONE,
            CAT_TWO_ERR_TWO,
            CAT_TWO_CUSTOM_NAME,
        ];

    }

    pub static A: [&[&str]; 2] = [&cat_one::A, &cat_two::A];
}

mod _d {
    pub(crate) mod cat_one {
        use super::super::_n;
        const CAT_ONE_ERR_ONE: &str = _n::cat_one::CAT_ONE_ERR_ONE;
        const CAT_ONE_ERR_TWO: &str = _n::cat_one::CAT_ONE_ERR_TWO;
        pub static A: [&str; 2] = [CAT_ONE_ERR_ONE, CAT_ONE_ERR_TWO];
    }

    pub(crate) mod cat_two {
        use super::super::_n;
        const CAT_TWO_ERR_ONE: &str = _n::cat_two::CAT_TWO_ERR_ONE;
        const CAT_TWO_ERR_TWO: &str = _n::cat_two::CAT_TWO_ERR_TWO;
        const CAT_TWO_CUSTOM_NAME: &str = _n::cat_two::CAT_TWO_CUSTOM_NAME;
        pub static A: [&str; 3] = [
            CAT_TWO_ERR_ONE,
            CAT_TWO_ERR_TWO,
            CAT_TWO_CUSTOM_NAME,
        ];

    }

    pub static A: [&[&str]; 2] = [&cat_one::A, &cat_two::A];
}

mod _p {
    pub type R = u8;
    pub const KIND_BITS: usize = 3;
    pub const CAT_BITS: usize = 1;
    pub const CAT_MAX: R = 1;
    pub const VAR_MASK: R = 3;
    pub static VAR_MAXES: [R; 2] = [1, 2];
    pub const CAT_MASK: R = 4;
    pub const VAR_BITS: usize = 2;
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

    /// Category #1
    pub const CAT_ONE: C = C::new(0);

    /// Category #2
    pub const CAT_TWO: C = C::new(1);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;

    /// CatOne error #1
    pub const CAT_ONE_ERR_ONE: EK = EK::new(c::CAT_ONE, 0);

    /// CatOne error #2
    pub const CAT_ONE_ERR_TWO: EK = EK::new(c::CAT_ONE, 1);

    /// CatTwo error #1
    pub const CAT_TWO_ERR_ONE: EK = EK::new(c::CAT_TWO, 0);

    /// CatTwo error #2
    pub const CAT_TWO_ERR_TWO: EK = EK::new(c::CAT_TWO, 1);

    /// A CatTwo error without display string.
    pub const CAT_TWO_CUSTOM_NAME: EK = EK::new(c::CAT_TWO, 2);
}


/// Error variant types and constants.
pub mod variant {

    /// Error variant types.
    pub mod types {
        use super::super::{_p, _cn, _n, category, kind, ErrorKind, Error, ErrorCategory};

        /// CatOne error #1
        #[derive(Clone, Copy)]
        #[non_exhaustive]
        pub struct CatOneErrOne;

        impl CatOneErrOne {
            /// Returns the struct name.
            #[inline]
            pub fn name(&self) -> &'static str {
                <CatOneErrOne as tighterror::VariantType>::NAME
            }


            /// Returns the error kind constant.
            #[inline]
            pub fn kind(&self) -> ErrorKind {
                <CatOneErrOne as tighterror::VariantType>::KIND
            }


            /// Returns the error category constant.
            #[inline]
            pub fn category(&self) -> ErrorCategory {
                <CatOneErrOne as tighterror::VariantType>::CATEGORY
            }

        }


        impl core::fmt::Display for CatOneErrOne {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad(<Self as tighterror::VariantType>::NAME)
            }

        }


        impl core::fmt::Debug for CatOneErrOne {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("CatOneErrOne")
                    .field("cat", &_p::Ident(_cn::CAT_ONE))
                    .field("var", &_p::Ident(_n::cat_one::CAT_ONE_ERR_ONE))
                    .finish()
            }

        }


        impl tighterror::VariantType for CatOneErrOne {
            type R = _p::R;
            type Category = ErrorCategory;
            type Kind = ErrorKind;
            const CATEGORY: Self::Category = category::CAT_ONE;
            const KIND: Self::Kind = kind::CAT_ONE_ERR_ONE;
            const NAME: &'static str = "CatOneErrOne";
        }


        impl core::convert::From<CatOneErrOne> for ErrorKind {
            #[inline]
            fn from(_: CatOneErrOne) -> ErrorKind {
                <CatOneErrOne as tighterror::VariantType>::KIND
            }

        }


        impl core::convert::From<CatOneErrOne> for Error {
            #[inline]
            fn from(_: CatOneErrOne) -> Self {
                <CatOneErrOne as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatOneErrOne> for core::result::Result<T, Error> {
            #[inline]
            fn from(_: CatOneErrOne) -> Self {
                <CatOneErrOne as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatOneErrOne>
        for core::result::Result<T, CatOneErrOne> {

            #[inline]
            fn from(v: CatOneErrOne) -> Self {
                Err(v)
            }

        }


        impl std::error::Error for CatOneErrOne {}

        /// CatOne error #2
        #[derive(Clone, Copy)]
        #[non_exhaustive]
        pub struct CatOneErrTwo;

        impl CatOneErrTwo {
            /// Returns the struct name.
            #[inline]
            pub fn name(&self) -> &'static str {
                <CatOneErrTwo as tighterror::VariantType>::NAME
            }


            /// Returns the error kind constant.
            #[inline]
            pub fn kind(&self) -> ErrorKind {
                <CatOneErrTwo as tighterror::VariantType>::KIND
            }


            /// Returns the error category constant.
            #[inline]
            pub fn category(&self) -> ErrorCategory {
                <CatOneErrTwo as tighterror::VariantType>::CATEGORY
            }

        }


        impl core::fmt::Display for CatOneErrTwo {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad(<Self as tighterror::VariantType>::NAME)
            }

        }


        impl core::fmt::Debug for CatOneErrTwo {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("CatOneErrTwo")
                    .field("cat", &_p::Ident(_cn::CAT_ONE))
                    .field("var", &_p::Ident(_n::cat_one::CAT_ONE_ERR_TWO))
                    .finish()
            }

        }


        impl tighterror::VariantType for CatOneErrTwo {
            type R = _p::R;
            type Category = ErrorCategory;
            type Kind = ErrorKind;
            const CATEGORY: Self::Category = category::CAT_ONE;
            const KIND: Self::Kind = kind::CAT_ONE_ERR_TWO;
            const NAME: &'static str = "CatOneErrTwo";
        }


        impl core::convert::From<CatOneErrTwo> for ErrorKind {
            #[inline]
            fn from(_: CatOneErrTwo) -> ErrorKind {
                <CatOneErrTwo as tighterror::VariantType>::KIND
            }

        }


        impl core::convert::From<CatOneErrTwo> for Error {
            #[inline]
            fn from(_: CatOneErrTwo) -> Self {
                <CatOneErrTwo as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatOneErrTwo> for core::result::Result<T, Error> {
            #[inline]
            fn from(_: CatOneErrTwo) -> Self {
                <CatOneErrTwo as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatOneErrTwo>
        for core::result::Result<T, CatOneErrTwo> {

            #[inline]
            fn from(v: CatOneErrTwo) -> Self {
                Err(v)
            }

        }


        impl std::error::Error for CatOneErrTwo {}

        /// CatTwo error #1
        #[derive(Clone, Copy)]
        #[non_exhaustive]
        pub struct CatTwoErrOne;

        impl CatTwoErrOne {
            /// Returns the struct name.
            #[inline]
            pub fn name(&self) -> &'static str {
                <CatTwoErrOne as tighterror::VariantType>::NAME
            }


            /// Returns the error kind constant.
            #[inline]
            pub fn kind(&self) -> ErrorKind {
                <CatTwoErrOne as tighterror::VariantType>::KIND
            }


            /// Returns the error category constant.
            #[inline]
            pub fn category(&self) -> ErrorCategory {
                <CatTwoErrOne as tighterror::VariantType>::CATEGORY
            }

        }


        impl core::fmt::Display for CatTwoErrOne {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad(<Self as tighterror::VariantType>::NAME)
            }

        }


        impl core::fmt::Debug for CatTwoErrOne {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("CatTwoErrOne")
                    .field("cat", &_p::Ident(_cn::CAT_TWO))
                    .field("var", &_p::Ident(_n::cat_two::CAT_TWO_ERR_ONE))
                    .finish()
            }

        }


        impl tighterror::VariantType for CatTwoErrOne {
            type R = _p::R;
            type Category = ErrorCategory;
            type Kind = ErrorKind;
            const CATEGORY: Self::Category = category::CAT_TWO;
            const KIND: Self::Kind = kind::CAT_TWO_ERR_ONE;
            const NAME: &'static str = "CatTwoErrOne";
        }


        impl core::convert::From<CatTwoErrOne> for ErrorKind {
            #[inline]
            fn from(_: CatTwoErrOne) -> ErrorKind {
                <CatTwoErrOne as tighterror::VariantType>::KIND
            }

        }


        impl core::convert::From<CatTwoErrOne> for Error {
            #[inline]
            fn from(_: CatTwoErrOne) -> Self {
                <CatTwoErrOne as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatTwoErrOne> for core::result::Result<T, Error> {
            #[inline]
            fn from(_: CatTwoErrOne) -> Self {
                <CatTwoErrOne as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatTwoErrOne>
        for core::result::Result<T, CatTwoErrOne> {

            #[inline]
            fn from(v: CatTwoErrOne) -> Self {
                Err(v)
            }

        }


        impl std::error::Error for CatTwoErrOne {}

        /// CatTwo error #2
        #[derive(Clone, Copy)]
        #[non_exhaustive]
        pub struct CatTwoErrTwo;

        impl CatTwoErrTwo {
            /// Returns the struct name.
            #[inline]
            pub fn name(&self) -> &'static str {
                <CatTwoErrTwo as tighterror::VariantType>::NAME
            }


            /// Returns the error kind constant.
            #[inline]
            pub fn kind(&self) -> ErrorKind {
                <CatTwoErrTwo as tighterror::VariantType>::KIND
            }


            /// Returns the error category constant.
            #[inline]
            pub fn category(&self) -> ErrorCategory {
                <CatTwoErrTwo as tighterror::VariantType>::CATEGORY
            }

        }


        impl core::fmt::Display for CatTwoErrTwo {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad(<Self as tighterror::VariantType>::NAME)
            }

        }


        impl core::fmt::Debug for CatTwoErrTwo {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("CatTwoErrTwo")
                    .field("cat", &_p::Ident(_cn::CAT_TWO))
                    .field("var", &_p::Ident(_n::cat_two::CAT_TWO_ERR_TWO))
                    .finish()
            }

        }


        impl tighterror::VariantType for CatTwoErrTwo {
            type R = _p::R;
            type Category = ErrorCategory;
            type Kind = ErrorKind;
            const CATEGORY: Self::Category = category::CAT_TWO;
            const KIND: Self::Kind = kind::CAT_TWO_ERR_TWO;
            const NAME: &'static str = "CatTwoErrTwo";
        }


        impl core::convert::From<CatTwoErrTwo> for ErrorKind {
            #[inline]
            fn from(_: CatTwoErrTwo) -> ErrorKind {
                <CatTwoErrTwo as tighterror::VariantType>::KIND
            }

        }


        impl core::convert::From<CatTwoErrTwo> for Error {
            #[inline]
            fn from(_: CatTwoErrTwo) -> Self {
                <CatTwoErrTwo as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatTwoErrTwo> for core::result::Result<T, Error> {
            #[inline]
            fn from(_: CatTwoErrTwo) -> Self {
                <CatTwoErrTwo as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatTwoErrTwo>
        for core::result::Result<T, CatTwoErrTwo> {

            #[inline]
            fn from(v: CatTwoErrTwo) -> Self {
                Err(v)
            }

        }


        impl std::error::Error for CatTwoErrTwo {}

        /// A CatTwo error without display string.
        #[derive(Clone, Copy)]
        #[non_exhaustive]
        pub struct CatTwoCustomVariantTypeName;

        impl CatTwoCustomVariantTypeName {
            /// Returns the struct name.
            #[inline]
            pub fn name(&self) -> &'static str {
                <CatTwoCustomVariantTypeName as tighterror::VariantType>::NAME
            }


            /// Returns the error kind constant.
            #[inline]
            pub fn kind(&self) -> ErrorKind {
                <CatTwoCustomVariantTypeName as tighterror::VariantType>::KIND
            }


            /// Returns the error category constant.
            #[inline]
            pub fn category(&self) -> ErrorCategory {
                <CatTwoCustomVariantTypeName as tighterror::VariantType>::CATEGORY
            }

        }


        impl core::fmt::Display for CatTwoCustomVariantTypeName {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad(<Self as tighterror::VariantType>::NAME)
            }

        }


        impl core::fmt::Debug for CatTwoCustomVariantTypeName {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("CatTwoCustomVariantTypeName")
                    .field("cat", &_p::Ident(_cn::CAT_TWO))
                    .field("var", &_p::Ident(_n::cat_two::CAT_TWO_CUSTOM_NAME))
                    .finish()
            }

        }


        impl tighterror::VariantType for CatTwoCustomVariantTypeName {
            type R = _p::R;
            type Category = ErrorCategory;
            type Kind = ErrorKind;
            const CATEGORY: Self::Category = category::CAT_TWO;
            const KIND: Self::Kind = kind::CAT_TWO_CUSTOM_NAME;
            const NAME: &'static str = "CatTwoCustomVariantTypeName";
        }


        impl core::convert::From<CatTwoCustomVariantTypeName> for ErrorKind {
            #[inline]
            fn from(_: CatTwoCustomVariantTypeName) -> ErrorKind {
                <CatTwoCustomVariantTypeName as tighterror::VariantType>::KIND
            }

        }


        impl core::convert::From<CatTwoCustomVariantTypeName> for Error {
            #[inline]
            fn from(_: CatTwoCustomVariantTypeName) -> Self {
                <CatTwoCustomVariantTypeName as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatTwoCustomVariantTypeName>
        for core::result::Result<T, Error> {

            #[inline]
            fn from(_: CatTwoCustomVariantTypeName) -> Self {
                <CatTwoCustomVariantTypeName as tighterror::VariantType>::KIND.into()
            }

        }


        impl<T> core::convert::From<CatTwoCustomVariantTypeName>
        for core::result::Result<T, CatTwoCustomVariantTypeName> {

            #[inline]
            fn from(v: CatTwoCustomVariantTypeName) -> Self {
                Err(v)
            }

        }


        impl std::error::Error for CatTwoCustomVariantTypeName {}

    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(CAT_ONE.name(), "CAT_ONE");
        assert_eq!(tighterror::Category::name(& CAT_ONE), "CAT_ONE");
        assert_eq!(CAT_TWO.name(), "CAT_TWO");
        assert_eq!(tighterror::Category::name(& CAT_TWO), "CAT_TWO")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", CAT_ONE), "CAT_ONE");
        assert_eq!(format!("{}", CAT_TWO), "CAT_TWO");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [ErrorCategory; 2] = [CAT_ONE, CAT_TWO];
        let set = HashSet::<ErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 2);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [ErrorCategory; 2] = [CAT_ONE, CAT_TWO];
        for c in cats {
            assert!(c.0 <= 1);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(CAT_ONE_ERR_ONE.name(), "CAT_ONE_ERR_ONE");
        assert_eq!(tighterror::Kind::name(& CAT_ONE_ERR_ONE), "CAT_ONE_ERR_ONE");
        assert_eq!(CAT_ONE_ERR_TWO.name(), "CAT_ONE_ERR_TWO");
        assert_eq!(tighterror::Kind::name(& CAT_ONE_ERR_TWO), "CAT_ONE_ERR_TWO");
        assert_eq!(CAT_TWO_ERR_ONE.name(), "CAT_TWO_ERR_ONE");
        assert_eq!(tighterror::Kind::name(& CAT_TWO_ERR_ONE), "CAT_TWO_ERR_ONE");
        assert_eq!(CAT_TWO_ERR_TWO.name(), "CAT_TWO_ERR_TWO");
        assert_eq!(tighterror::Kind::name(& CAT_TWO_ERR_TWO), "CAT_TWO_ERR_TWO");
        assert_eq!(CAT_TWO_CUSTOM_NAME.name(), "CAT_TWO_CUSTOM_NAME");
        assert_eq!(tighterror::Kind::name(& CAT_TWO_CUSTOM_NAME), "CAT_TWO_CUSTOM_NAME");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", CAT_ONE_ERR_ONE), "CAT_ONE_ERR_ONE");
        assert_eq!(format!("{}", CAT_ONE_ERR_TWO), "CAT_ONE_ERR_TWO");
        assert_eq!(format!("{}", CAT_TWO_ERR_ONE), "CAT_TWO_ERR_ONE");
        assert_eq!(format!("{}", CAT_TWO_ERR_TWO), "CAT_TWO_ERR_TWO");
        assert_eq!(format!("{}", CAT_TWO_CUSTOM_NAME), "CAT_TWO_CUSTOM_NAME");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 5] = [
            CAT_ONE_ERR_ONE,
            CAT_ONE_ERR_TWO,
            CAT_TWO_ERR_ONE,
            CAT_TWO_ERR_TWO,
            CAT_TWO_CUSTOM_NAME,
        ];

        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 5);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 5] = [
            CAT_ONE_ERR_ONE,
            CAT_ONE_ERR_TWO,
            CAT_TWO_ERR_ONE,
            CAT_TWO_ERR_TWO,
            CAT_TWO_CUSTOM_NAME,
        ];

        let set = HashSet::<u8>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 5);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(CAT_ONE_ERR_ONE.category(), category::CAT_ONE);
        assert_eq!(CAT_ONE_ERR_TWO.category(), category::CAT_ONE);
        assert_eq!(CAT_TWO_ERR_ONE.category(), category::CAT_TWO);
        assert_eq!(CAT_TWO_ERR_TWO.category(), category::CAT_TWO);
        assert_eq!(CAT_TWO_CUSTOM_NAME.category(), category::CAT_TWO);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(
            ErrorKind::from_value(CAT_ONE_ERR_ONE.value()).unwrap(), CAT_ONE_ERR_ONE
        );
        assert_eq!(
            ErrorKind::from_value(CAT_ONE_ERR_TWO.value()).unwrap(), CAT_ONE_ERR_TWO
        );
        assert_eq!(
            ErrorKind::from_value(CAT_TWO_ERR_ONE.value()).unwrap(), CAT_TWO_ERR_ONE
        );
        assert_eq!(
            ErrorKind::from_value(CAT_TWO_ERR_TWO.value()).unwrap(), CAT_TWO_ERR_TWO
        );
        assert_eq!(
            ErrorKind::from_value(CAT_TWO_CUSTOM_NAME.value()).unwrap(),
            CAT_TWO_CUSTOM_NAME
        );
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(format!("{}", Error::from(CAT_ONE_ERR_ONE)), "CAT_ONE_ERR_ONE");
        assert_eq!(format!("{}", Error::from(CAT_ONE_ERR_TWO)), "CAT_ONE_ERR_TWO");
        assert_eq!(format!("{}", Error::from(CAT_TWO_ERR_ONE)), "CAT_TWO_ERR_ONE");
        assert_eq!(format!("{}", Error::from(CAT_TWO_ERR_TWO)), "CAT_TWO_ERR_TWO");
        assert_eq!(
            format!("{}", Error::from(CAT_TWO_CUSTOM_NAME)), "CAT_TWO_CUSTOM_NAME"
        );
    }


    #[test]
    fn test_variant_types_display() {
        use variant::types::*;
        assert_eq!(format!("{}", CatOneErrOne), "CatOneErrOne");
        assert_eq!(format!("{}", CatOneErrTwo), "CatOneErrTwo");
        assert_eq!(format!("{}", CatTwoErrOne), "CatTwoErrOne");
        assert_eq!(format!("{}", CatTwoErrTwo), "CatTwoErrTwo");
        assert_eq!(
            format!("{}", CatTwoCustomVariantTypeName), "CatTwoCustomVariantTypeName"
        );
    }


    #[test]
    fn test_variant_types_to_kind() {
        use variant::types::*;
        assert_eq!(ErrorKind::from(CatOneErrOne), kind::CAT_ONE_ERR_ONE);
        assert_eq!(ErrorKind::from(CatOneErrTwo), kind::CAT_ONE_ERR_TWO);
        assert_eq!(ErrorKind::from(CatTwoErrOne), kind::CAT_TWO_ERR_ONE);
        assert_eq!(ErrorKind::from(CatTwoErrTwo), kind::CAT_TWO_ERR_TWO);
        assert_eq!(
            ErrorKind::from(CatTwoCustomVariantTypeName), kind::CAT_TWO_CUSTOM_NAME
        );
    }


    #[test]
    fn test_variant_types_to_error() {
        use variant::types::*;
        assert_eq!(Error::from(CatOneErrOne).kind(), kind::CAT_ONE_ERR_ONE);
        assert_eq!(Error::from(CatOneErrTwo).kind(), kind::CAT_ONE_ERR_TWO);
        assert_eq!(Error::from(CatTwoErrOne).kind(), kind::CAT_TWO_ERR_ONE);
        assert_eq!(Error::from(CatTwoErrTwo).kind(), kind::CAT_TWO_ERR_TWO);
        assert_eq!(
            Error::from(CatTwoCustomVariantTypeName).kind(), kind::CAT_TWO_CUSTOM_NAME
        );
    }


    #[test]
    fn test_variant_types_to_result() {
        use variant::types::*;
        let res: Result<(), Error> = CatOneErrOne.into();
        assert_eq!(res.unwrap_err().kind(), kind::CAT_ONE_ERR_ONE);
        let res: Result<(), Error> = CatOneErrTwo.into();
        assert_eq!(res.unwrap_err().kind(), kind::CAT_ONE_ERR_TWO);
        let res: Result<(), Error> = CatTwoErrOne.into();
        assert_eq!(res.unwrap_err().kind(), kind::CAT_TWO_ERR_ONE);
        let res: Result<(), Error> = CatTwoErrTwo.into();
        assert_eq!(res.unwrap_err().kind(), kind::CAT_TWO_ERR_TWO);
        let res: Result<(), Error> = CatTwoCustomVariantTypeName.into();
        assert_eq!(res.unwrap_err().kind(), kind::CAT_TWO_CUSTOM_NAME);
    }

}

