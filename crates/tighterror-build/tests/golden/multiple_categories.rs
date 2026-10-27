
/// A custom category doc.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct McErrorCategory(_p::R);

impl McErrorCategory {
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


impl tighterror::Category for McErrorCategory {
    type R = _p::R;
    const BITS: usize = _p::CAT_BITS;

    #[inline]
    fn name(&self) -> &'static str {
        self.name()
    }

}


impl core::fmt::Display for McErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }

}


impl core::fmt::Debug for McErrorCategory {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("McErrorCategory").field(&_p::Ident(self.name())).finish()
    }

}


/// A custom error doc.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct McErrorKind(_p::R);

impl McErrorKind {
    const fn new(cat: McErrorCategory, variant: _p::R) -> Self {
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
    pub fn category(&self) -> McErrorCategory {
        McErrorCategory::new(self.category_value())
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
            Some(Self::new(McErrorCategory::new(cat), variant))
        } else {
            None
        }

    }

}


impl tighterror::Kind for McErrorKind {
    type R = _p::R;
    type Category = McErrorCategory;
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


impl core::fmt::Display for McErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.name())
    }

}


impl core::fmt::Debug for McErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("McErrorKind")
            .field("cat", &_p::Ident(self.category().name()))
            .field("var", &_p::Ident(self.name()))
            .field("val", &self.0)
            .finish()
    }

}


impl<T> core::convert::From<McErrorKind> for Result<T, McError> {
    #[inline]
    fn from(v: McErrorKind) -> Self {
        Err(v.into())
    }

}


/// A custom error struct doc.
#[derive(Debug)]
#[repr(transparent)]
pub struct McError(McErrorKind);

impl McError {
    /// Returns the error kind.
    #[inline]
    pub fn kind(&self) -> McErrorKind {
        self.0
    }


    /// Returns the error origin location.
    #[inline]
    pub fn location(&self) -> tighterror::Location {
        tighterror::Location::undefined()
    }

}


impl tighterror::Error for McError {
    type R = _p::R;
    type Category = McErrorCategory;
    type Kind = McErrorKind;

    #[inline]
    fn kind(&self) -> Self::Kind {
        self.kind()
    }


    #[inline]
    fn location(&self) -> tighterror::Location {
        self.location()
    }

}


impl core::convert::From<McErrorKind> for McError {
    #[inline]
    fn from(kind: McErrorKind) -> Self {
        Self(kind)
    }

}


impl core::fmt::Display for McError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.kind().display())
    }

}


impl core::cmp::PartialEq for McError {
    /// Checks equality based on the error kind only.
    #[inline]
    fn eq(&self, other: &McError) -> bool {
        self.0 == other.0
    }

}


impl<T> core::convert::From<McError> for core::result::Result<T, McError> {
    #[inline]
    fn from(err: McError) -> Self {
        Err(err)
    }

}


impl std::error::Error for McError {}

mod _cn {
    pub const PARSING: &str = "PARSING";
    pub const PROCESSING: &str = "PROCESSING";
    pub const GENERAL: &str = "GENERAL";
    pub static A: [&str; 3] = [PARSING, PROCESSING, GENERAL];
}

mod _n {
    pub(crate) mod parsing {
        pub(crate) const BAD_TOKEN: &str = "BAD_TOKEN";
        pub(crate) const QUEUE_FULL: &str = "QUEUE_FULL";
        pub static A: [&str; 2] = [BAD_TOKEN, QUEUE_FULL];
    }

    pub(crate) mod processing {
        pub(crate) const OUT_OF_MEMORY: &str = "OUT_OF_MEMORY";
        pub(crate) const QUEUE_FULL: &str = "QUEUE_FULL";
        pub static A: [&str; 2] = [OUT_OF_MEMORY, QUEUE_FULL];
    }

    pub(crate) mod general {
        pub(crate) const BAD_ARG: &str = "BAD_ARG";
        pub(crate) const TIMEOUT: &str = "TIMEOUT";
        pub static A: [&str; 2] = [BAD_ARG, TIMEOUT];
    }

    pub static A: [&[&str]; 3] = [&parsing::A, &processing::A, &general::A];
}

mod _d {
    pub(crate) mod parsing {
        pub(crate) const BAD_TOKEN: &str = "Decoded an invalid token.";
        pub(crate) const QUEUE_FULL: &str = "Parsing queue is full.";
        pub static A: [&str; 2] = [BAD_TOKEN, QUEUE_FULL];
    }

    pub(crate) mod processing {
        pub(crate) const OUT_OF_MEMORY: &str = "Memory pool is empty.";
        pub(crate) const QUEUE_FULL: &str = "Processing queue is full.";
        pub static A: [&str; 2] = [OUT_OF_MEMORY, QUEUE_FULL];
    }

    pub(crate) mod general {
        pub(crate) const BAD_ARG: &str = "Invalid argument received.";
        pub(crate) const TIMEOUT: &str = "Operation timed out.";
        pub static A: [&str; 2] = [BAD_ARG, TIMEOUT];
    }

    pub static A: [&[&str]; 3] = [&parsing::A, &processing::A, &general::A];
}

mod _p {
    pub type R = u8;
    pub const KIND_BITS: usize = 3;
    pub const CAT_BITS: usize = 2;
    pub const CAT_MAX: R = 2;
    pub const VAR_MASK: R = 1;
    pub static VAR_MAXES: [R; 3] = [1, 1, 1];
    pub const CAT_MASK: R = 6;
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
    use super::McErrorCategory as C;

    /// Parsing errors category.
    pub const PARSING: C = C::new(0);

    /// Processing errors category.
    pub const PROCESSING: C = C::new(1);

    /// General errors category.
    pub const GENERAL: C = C::new(2);
}


/// Error kind constants.
pub mod kind {
    use super::McErrorKind as EK;
    use super::category as c;

    /// Parsing category error kind constants.
    pub mod parsing {
        use super::c;
        use super::EK;

        /// Decoded an invalid token.
        pub const BAD_TOKEN: EK = EK::new(c::PARSING, 0);

        /// Parsing queue is full.
        pub const QUEUE_FULL: EK = EK::new(c::PARSING, 1);
    }


    /// Processing category error kind constants.
    pub mod processing {
        use super::c;
        use super::EK;

        /// Memory pool is empty.
        pub const OUT_OF_MEMORY: EK = EK::new(c::PROCESSING, 0);

        /// Processing queue is full.
        pub const QUEUE_FULL: EK = EK::new(c::PROCESSING, 1);
    }


    /// General category error kind constants.
    pub mod general {
        use super::c;
        use super::EK;

        /**
 * An invalid argument was received.
 *
 * # Examples
 *
 * ```rust
 * # struct Error;
 * # impl From<u32> for Error {
 * #     fn from(v: u32) -> Self {
 * #         Self{}
 * #     }
 * # }
 * # const BAD_ARG: u32 = 0;
 * let e: Error = BAD_ARG.into();
 * ```
 */
        pub const BAD_ARG: EK = EK::new(c::GENERAL, 0);

        /// Operation timed out.
        pub const TIMEOUT: EK = EK::new(c::GENERAL, 1);
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(PARSING.name(), "PARSING");
        assert_eq!(tighterror::Category::name(& PARSING), "PARSING");
        assert_eq!(PROCESSING.name(), "PROCESSING");
        assert_eq!(tighterror::Category::name(& PROCESSING), "PROCESSING");
        assert_eq!(GENERAL.name(), "GENERAL");
        assert_eq!(tighterror::Category::name(& GENERAL), "GENERAL")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", PARSING), "PARSING");
        assert_eq!(format!("{}", PROCESSING), "PROCESSING");
        assert_eq!(format!("{}", GENERAL), "GENERAL");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [McErrorCategory; 3] = [PARSING, PROCESSING, GENERAL];
        let set = HashSet::<McErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 3);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [McErrorCategory; 3] = [PARSING, PROCESSING, GENERAL];
        for c in cats {
            assert!(c.0 <= 2);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(parsing::BAD_TOKEN.name(), "BAD_TOKEN");
        assert_eq!(tighterror::Kind::name(& parsing::BAD_TOKEN), "BAD_TOKEN");
        assert_eq!(parsing::QUEUE_FULL.name(), "QUEUE_FULL");
        assert_eq!(tighterror::Kind::name(& parsing::QUEUE_FULL), "QUEUE_FULL");
        assert_eq!(processing::OUT_OF_MEMORY.name(), "OUT_OF_MEMORY");
        assert_eq!(tighterror::Kind::name(& processing::OUT_OF_MEMORY), "OUT_OF_MEMORY");
        assert_eq!(processing::QUEUE_FULL.name(), "QUEUE_FULL");
        assert_eq!(tighterror::Kind::name(& processing::QUEUE_FULL), "QUEUE_FULL");
        assert_eq!(general::BAD_ARG.name(), "BAD_ARG");
        assert_eq!(tighterror::Kind::name(& general::BAD_ARG), "BAD_ARG");
        assert_eq!(general::TIMEOUT.name(), "TIMEOUT");
        assert_eq!(tighterror::Kind::name(& general::TIMEOUT), "TIMEOUT");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", parsing::BAD_TOKEN), "BAD_TOKEN");
        assert_eq!(format!("{}", parsing::QUEUE_FULL), "QUEUE_FULL");
        assert_eq!(format!("{}", processing::OUT_OF_MEMORY), "OUT_OF_MEMORY");
        assert_eq!(format!("{}", processing::QUEUE_FULL), "QUEUE_FULL");
        assert_eq!(format!("{}", general::BAD_ARG), "BAD_ARG");
        assert_eq!(format!("{}", general::TIMEOUT), "TIMEOUT");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [McErrorKind; 6] = [
            parsing::BAD_TOKEN,
            parsing::QUEUE_FULL,
            processing::OUT_OF_MEMORY,
            processing::QUEUE_FULL,
            general::BAD_ARG,
            general::TIMEOUT,
        ];

        let set = HashSet::<McErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 6);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [McErrorKind; 6] = [
            parsing::BAD_TOKEN,
            parsing::QUEUE_FULL,
            processing::OUT_OF_MEMORY,
            processing::QUEUE_FULL,
            general::BAD_ARG,
            general::TIMEOUT,
        ];

        let set = HashSet::<u8>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 6);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(parsing::BAD_TOKEN.category(), category::PARSING);
        assert_eq!(parsing::QUEUE_FULL.category(), category::PARSING);
        assert_eq!(processing::OUT_OF_MEMORY.category(), category::PROCESSING);
        assert_eq!(processing::QUEUE_FULL.category(), category::PROCESSING);
        assert_eq!(general::BAD_ARG.category(), category::GENERAL);
        assert_eq!(general::TIMEOUT.category(), category::GENERAL);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(
            McErrorKind::from_value(parsing::BAD_TOKEN.value()).unwrap(),
            parsing::BAD_TOKEN
        );
        assert_eq!(
            McErrorKind::from_value(parsing::QUEUE_FULL.value()).unwrap(),
            parsing::QUEUE_FULL
        );
        assert_eq!(
            McErrorKind::from_value(processing::OUT_OF_MEMORY.value()).unwrap(),
            processing::OUT_OF_MEMORY
        );
        assert_eq!(
            McErrorKind::from_value(processing::QUEUE_FULL.value()).unwrap(),
            processing::QUEUE_FULL
        );
        assert_eq!(
            McErrorKind::from_value(general::BAD_ARG.value()).unwrap(), general::BAD_ARG
        );
        assert_eq!(
            McErrorKind::from_value(general::TIMEOUT.value()).unwrap(), general::TIMEOUT
        );
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(
            format!("{}", McError::from(parsing::BAD_TOKEN)), "Decoded an invalid token."
        );
        assert_eq!(
            format!("{}", McError::from(parsing::QUEUE_FULL)), "Parsing queue is full."
        );
        assert_eq!(
            format!("{}", McError::from(processing::OUT_OF_MEMORY)),
            "Memory pool is empty."
        );
        assert_eq!(
            format!("{}", McError::from(processing::QUEUE_FULL)),
            "Processing queue is full."
        );
        assert_eq!(
            format!("{}", McError::from(general::BAD_ARG)), "Invalid argument received."
        );
        assert_eq!(
            format!("{}", McError::from(general::TIMEOUT)), "Operation timed out."
        );
    }

}

