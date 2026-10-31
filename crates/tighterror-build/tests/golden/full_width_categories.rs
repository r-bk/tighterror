
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
    pub const FIRST: &str = "FIRST";
    pub const SECOND: &str = "SECOND";
    pub static A: [&str; 2] = [FIRST, SECOND];
}

mod _n {
    pub(crate) mod first {
        pub(crate) const ONLY: &str = "ONLY";
        pub static A: [&str; 2] = ["", ONLY];
    }

    pub(crate) mod second {
        pub(crate) const ERR0: &str = "ERR0";
        pub(crate) const ERR1: &str = "ERR1";
        pub(crate) const ERR2: &str = "ERR2";
        pub(crate) const ERR3: &str = "ERR3";
        pub(crate) const ERR4: &str = "ERR4";
        pub(crate) const ERR5: &str = "ERR5";
        pub(crate) const ERR6: &str = "ERR6";
        pub(crate) const ERR7: &str = "ERR7";
        pub(crate) const ERR8: &str = "ERR8";
        pub(crate) const ERR9: &str = "ERR9";
        pub(crate) const ERR10: &str = "ERR10";
        pub(crate) const ERR11: &str = "ERR11";
        pub(crate) const ERR12: &str = "ERR12";
        pub(crate) const ERR13: &str = "ERR13";
        pub(crate) const ERR14: &str = "ERR14";
        pub(crate) const ERR15: &str = "ERR15";
        pub(crate) const ERR16: &str = "ERR16";
        pub(crate) const ERR17: &str = "ERR17";
        pub(crate) const ERR18: &str = "ERR18";
        pub(crate) const ERR19: &str = "ERR19";
        pub(crate) const ERR20: &str = "ERR20";
        pub(crate) const ERR21: &str = "ERR21";
        pub(crate) const ERR22: &str = "ERR22";
        pub(crate) const ERR23: &str = "ERR23";
        pub(crate) const ERR24: &str = "ERR24";
        pub(crate) const ERR25: &str = "ERR25";
        pub(crate) const ERR26: &str = "ERR26";
        pub(crate) const ERR27: &str = "ERR27";
        pub(crate) const ERR28: &str = "ERR28";
        pub(crate) const ERR29: &str = "ERR29";
        pub(crate) const ERR30: &str = "ERR30";
        pub(crate) const ERR31: &str = "ERR31";
        pub(crate) const ERR32: &str = "ERR32";
        pub(crate) const ERR33: &str = "ERR33";
        pub(crate) const ERR34: &str = "ERR34";
        pub(crate) const ERR35: &str = "ERR35";
        pub(crate) const ERR36: &str = "ERR36";
        pub(crate) const ERR37: &str = "ERR37";
        pub(crate) const ERR38: &str = "ERR38";
        pub(crate) const ERR39: &str = "ERR39";
        pub(crate) const ERR40: &str = "ERR40";
        pub(crate) const ERR41: &str = "ERR41";
        pub(crate) const ERR42: &str = "ERR42";
        pub(crate) const ERR43: &str = "ERR43";
        pub(crate) const ERR44: &str = "ERR44";
        pub(crate) const ERR45: &str = "ERR45";
        pub(crate) const ERR46: &str = "ERR46";
        pub(crate) const ERR47: &str = "ERR47";
        pub(crate) const ERR48: &str = "ERR48";
        pub(crate) const ERR49: &str = "ERR49";
        pub(crate) const ERR50: &str = "ERR50";
        pub(crate) const ERR51: &str = "ERR51";
        pub(crate) const ERR52: &str = "ERR52";
        pub(crate) const ERR53: &str = "ERR53";
        pub(crate) const ERR54: &str = "ERR54";
        pub(crate) const ERR55: &str = "ERR55";
        pub(crate) const ERR56: &str = "ERR56";
        pub(crate) const ERR57: &str = "ERR57";
        pub(crate) const ERR58: &str = "ERR58";
        pub(crate) const ERR59: &str = "ERR59";
        pub(crate) const ERR60: &str = "ERR60";
        pub(crate) const ERR61: &str = "ERR61";
        pub(crate) const ERR62: &str = "ERR62";
        pub(crate) const ERR63: &str = "ERR63";
        pub(crate) const ERR64: &str = "ERR64";
        pub(crate) const ERR65: &str = "ERR65";
        pub(crate) const ERR66: &str = "ERR66";
        pub(crate) const ERR67: &str = "ERR67";
        pub(crate) const ERR68: &str = "ERR68";
        pub(crate) const ERR69: &str = "ERR69";
        pub(crate) const ERR70: &str = "ERR70";
        pub(crate) const ERR71: &str = "ERR71";
        pub(crate) const ERR72: &str = "ERR72";
        pub(crate) const ERR73: &str = "ERR73";
        pub(crate) const ERR74: &str = "ERR74";
        pub(crate) const ERR75: &str = "ERR75";
        pub(crate) const ERR76: &str = "ERR76";
        pub(crate) const ERR77: &str = "ERR77";
        pub(crate) const ERR78: &str = "ERR78";
        pub(crate) const ERR79: &str = "ERR79";
        pub(crate) const ERR80: &str = "ERR80";
        pub(crate) const ERR81: &str = "ERR81";
        pub(crate) const ERR82: &str = "ERR82";
        pub(crate) const ERR83: &str = "ERR83";
        pub(crate) const ERR84: &str = "ERR84";
        pub(crate) const ERR85: &str = "ERR85";
        pub(crate) const ERR86: &str = "ERR86";
        pub(crate) const ERR87: &str = "ERR87";
        pub(crate) const ERR88: &str = "ERR88";
        pub(crate) const ERR89: &str = "ERR89";
        pub(crate) const ERR90: &str = "ERR90";
        pub(crate) const ERR91: &str = "ERR91";
        pub(crate) const ERR92: &str = "ERR92";
        pub(crate) const ERR93: &str = "ERR93";
        pub(crate) const ERR94: &str = "ERR94";
        pub(crate) const ERR95: &str = "ERR95";
        pub(crate) const ERR96: &str = "ERR96";
        pub(crate) const ERR97: &str = "ERR97";
        pub(crate) const ERR98: &str = "ERR98";
        pub(crate) const ERR99: &str = "ERR99";
        pub(crate) const ERR100: &str = "ERR100";
        pub(crate) const ERR101: &str = "ERR101";
        pub(crate) const ERR102: &str = "ERR102";
        pub(crate) const ERR103: &str = "ERR103";
        pub(crate) const ERR104: &str = "ERR104";
        pub(crate) const ERR105: &str = "ERR105";
        pub(crate) const ERR106: &str = "ERR106";
        pub(crate) const ERR107: &str = "ERR107";
        pub(crate) const ERR108: &str = "ERR108";
        pub(crate) const ERR109: &str = "ERR109";
        pub(crate) const ERR110: &str = "ERR110";
        pub(crate) const ERR111: &str = "ERR111";
        pub(crate) const ERR112: &str = "ERR112";
        pub(crate) const ERR113: &str = "ERR113";
        pub(crate) const ERR114: &str = "ERR114";
        pub(crate) const ERR115: &str = "ERR115";
        pub(crate) const ERR116: &str = "ERR116";
        pub(crate) const ERR117: &str = "ERR117";
        pub(crate) const ERR118: &str = "ERR118";
        pub(crate) const ERR119: &str = "ERR119";
        pub(crate) const ERR120: &str = "ERR120";
        pub(crate) const ERR121: &str = "ERR121";
        pub(crate) const ERR122: &str = "ERR122";
        pub(crate) const ERR123: &str = "ERR123";
        pub(crate) const ERR124: &str = "ERR124";
        pub(crate) const ERR125: &str = "ERR125";
        pub(crate) const ERR126: &str = "ERR126";
        pub static A: [&str; 128] = [
            "",
            ERR0,
            ERR1,
            ERR2,
            ERR3,
            ERR4,
            ERR5,
            ERR6,
            ERR7,
            ERR8,
            ERR9,
            ERR10,
            ERR11,
            ERR12,
            ERR13,
            ERR14,
            ERR15,
            ERR16,
            ERR17,
            ERR18,
            ERR19,
            ERR20,
            ERR21,
            ERR22,
            ERR23,
            ERR24,
            ERR25,
            ERR26,
            ERR27,
            ERR28,
            ERR29,
            ERR30,
            ERR31,
            ERR32,
            ERR33,
            ERR34,
            ERR35,
            ERR36,
            ERR37,
            ERR38,
            ERR39,
            ERR40,
            ERR41,
            ERR42,
            ERR43,
            ERR44,
            ERR45,
            ERR46,
            ERR47,
            ERR48,
            ERR49,
            ERR50,
            ERR51,
            ERR52,
            ERR53,
            ERR54,
            ERR55,
            ERR56,
            ERR57,
            ERR58,
            ERR59,
            ERR60,
            ERR61,
            ERR62,
            ERR63,
            ERR64,
            ERR65,
            ERR66,
            ERR67,
            ERR68,
            ERR69,
            ERR70,
            ERR71,
            ERR72,
            ERR73,
            ERR74,
            ERR75,
            ERR76,
            ERR77,
            ERR78,
            ERR79,
            ERR80,
            ERR81,
            ERR82,
            ERR83,
            ERR84,
            ERR85,
            ERR86,
            ERR87,
            ERR88,
            ERR89,
            ERR90,
            ERR91,
            ERR92,
            ERR93,
            ERR94,
            ERR95,
            ERR96,
            ERR97,
            ERR98,
            ERR99,
            ERR100,
            ERR101,
            ERR102,
            ERR103,
            ERR104,
            ERR105,
            ERR106,
            ERR107,
            ERR108,
            ERR109,
            ERR110,
            ERR111,
            ERR112,
            ERR113,
            ERR114,
            ERR115,
            ERR116,
            ERR117,
            ERR118,
            ERR119,
            ERR120,
            ERR121,
            ERR122,
            ERR123,
            ERR124,
            ERR125,
            ERR126,
        ];

    }

    pub static A: [&[&str]; 2] = [&first::A, &second::A];
}

mod _d {
    pub(crate) mod first {
        use super::super::_n;
        const ONLY: &str = _n::first::ONLY;
        pub static A: [&str; 2] = ["", ONLY];
    }

    pub(crate) mod second {
        use super::super::_n;
        const ERR0: &str = _n::second::ERR0;
        const ERR1: &str = _n::second::ERR1;
        const ERR2: &str = _n::second::ERR2;
        const ERR3: &str = _n::second::ERR3;
        const ERR4: &str = _n::second::ERR4;
        const ERR5: &str = _n::second::ERR5;
        const ERR6: &str = _n::second::ERR6;
        const ERR7: &str = _n::second::ERR7;
        const ERR8: &str = _n::second::ERR8;
        const ERR9: &str = _n::second::ERR9;
        const ERR10: &str = _n::second::ERR10;
        const ERR11: &str = _n::second::ERR11;
        const ERR12: &str = _n::second::ERR12;
        const ERR13: &str = _n::second::ERR13;
        const ERR14: &str = _n::second::ERR14;
        const ERR15: &str = _n::second::ERR15;
        const ERR16: &str = _n::second::ERR16;
        const ERR17: &str = _n::second::ERR17;
        const ERR18: &str = _n::second::ERR18;
        const ERR19: &str = _n::second::ERR19;
        const ERR20: &str = _n::second::ERR20;
        const ERR21: &str = _n::second::ERR21;
        const ERR22: &str = _n::second::ERR22;
        const ERR23: &str = _n::second::ERR23;
        const ERR24: &str = _n::second::ERR24;
        const ERR25: &str = _n::second::ERR25;
        const ERR26: &str = _n::second::ERR26;
        const ERR27: &str = _n::second::ERR27;
        const ERR28: &str = _n::second::ERR28;
        const ERR29: &str = _n::second::ERR29;
        const ERR30: &str = _n::second::ERR30;
        const ERR31: &str = _n::second::ERR31;
        const ERR32: &str = _n::second::ERR32;
        const ERR33: &str = _n::second::ERR33;
        const ERR34: &str = _n::second::ERR34;
        const ERR35: &str = _n::second::ERR35;
        const ERR36: &str = _n::second::ERR36;
        const ERR37: &str = _n::second::ERR37;
        const ERR38: &str = _n::second::ERR38;
        const ERR39: &str = _n::second::ERR39;
        const ERR40: &str = _n::second::ERR40;
        const ERR41: &str = _n::second::ERR41;
        const ERR42: &str = _n::second::ERR42;
        const ERR43: &str = _n::second::ERR43;
        const ERR44: &str = _n::second::ERR44;
        const ERR45: &str = _n::second::ERR45;
        const ERR46: &str = _n::second::ERR46;
        const ERR47: &str = _n::second::ERR47;
        const ERR48: &str = _n::second::ERR48;
        const ERR49: &str = _n::second::ERR49;
        const ERR50: &str = _n::second::ERR50;
        const ERR51: &str = _n::second::ERR51;
        const ERR52: &str = _n::second::ERR52;
        const ERR53: &str = _n::second::ERR53;
        const ERR54: &str = _n::second::ERR54;
        const ERR55: &str = _n::second::ERR55;
        const ERR56: &str = _n::second::ERR56;
        const ERR57: &str = _n::second::ERR57;
        const ERR58: &str = _n::second::ERR58;
        const ERR59: &str = _n::second::ERR59;
        const ERR60: &str = _n::second::ERR60;
        const ERR61: &str = _n::second::ERR61;
        const ERR62: &str = _n::second::ERR62;
        const ERR63: &str = _n::second::ERR63;
        const ERR64: &str = _n::second::ERR64;
        const ERR65: &str = _n::second::ERR65;
        const ERR66: &str = _n::second::ERR66;
        const ERR67: &str = _n::second::ERR67;
        const ERR68: &str = _n::second::ERR68;
        const ERR69: &str = _n::second::ERR69;
        const ERR70: &str = _n::second::ERR70;
        const ERR71: &str = _n::second::ERR71;
        const ERR72: &str = _n::second::ERR72;
        const ERR73: &str = _n::second::ERR73;
        const ERR74: &str = _n::second::ERR74;
        const ERR75: &str = _n::second::ERR75;
        const ERR76: &str = _n::second::ERR76;
        const ERR77: &str = _n::second::ERR77;
        const ERR78: &str = _n::second::ERR78;
        const ERR79: &str = _n::second::ERR79;
        const ERR80: &str = _n::second::ERR80;
        const ERR81: &str = _n::second::ERR81;
        const ERR82: &str = _n::second::ERR82;
        const ERR83: &str = _n::second::ERR83;
        const ERR84: &str = _n::second::ERR84;
        const ERR85: &str = _n::second::ERR85;
        const ERR86: &str = _n::second::ERR86;
        const ERR87: &str = _n::second::ERR87;
        const ERR88: &str = _n::second::ERR88;
        const ERR89: &str = _n::second::ERR89;
        const ERR90: &str = _n::second::ERR90;
        const ERR91: &str = _n::second::ERR91;
        const ERR92: &str = _n::second::ERR92;
        const ERR93: &str = _n::second::ERR93;
        const ERR94: &str = _n::second::ERR94;
        const ERR95: &str = _n::second::ERR95;
        const ERR96: &str = _n::second::ERR96;
        const ERR97: &str = _n::second::ERR97;
        const ERR98: &str = _n::second::ERR98;
        const ERR99: &str = _n::second::ERR99;
        const ERR100: &str = _n::second::ERR100;
        const ERR101: &str = _n::second::ERR101;
        const ERR102: &str = _n::second::ERR102;
        const ERR103: &str = _n::second::ERR103;
        const ERR104: &str = _n::second::ERR104;
        const ERR105: &str = _n::second::ERR105;
        const ERR106: &str = _n::second::ERR106;
        const ERR107: &str = _n::second::ERR107;
        const ERR108: &str = _n::second::ERR108;
        const ERR109: &str = _n::second::ERR109;
        const ERR110: &str = _n::second::ERR110;
        const ERR111: &str = _n::second::ERR111;
        const ERR112: &str = _n::second::ERR112;
        const ERR113: &str = _n::second::ERR113;
        const ERR114: &str = _n::second::ERR114;
        const ERR115: &str = _n::second::ERR115;
        const ERR116: &str = _n::second::ERR116;
        const ERR117: &str = _n::second::ERR117;
        const ERR118: &str = _n::second::ERR118;
        const ERR119: &str = _n::second::ERR119;
        const ERR120: &str = _n::second::ERR120;
        const ERR121: &str = _n::second::ERR121;
        const ERR122: &str = _n::second::ERR122;
        const ERR123: &str = _n::second::ERR123;
        const ERR124: &str = _n::second::ERR124;
        const ERR125: &str = _n::second::ERR125;
        const ERR126: &str = _n::second::ERR126;
        pub static A: [&str; 128] = [
            "",
            ERR0,
            ERR1,
            ERR2,
            ERR3,
            ERR4,
            ERR5,
            ERR6,
            ERR7,
            ERR8,
            ERR9,
            ERR10,
            ERR11,
            ERR12,
            ERR13,
            ERR14,
            ERR15,
            ERR16,
            ERR17,
            ERR18,
            ERR19,
            ERR20,
            ERR21,
            ERR22,
            ERR23,
            ERR24,
            ERR25,
            ERR26,
            ERR27,
            ERR28,
            ERR29,
            ERR30,
            ERR31,
            ERR32,
            ERR33,
            ERR34,
            ERR35,
            ERR36,
            ERR37,
            ERR38,
            ERR39,
            ERR40,
            ERR41,
            ERR42,
            ERR43,
            ERR44,
            ERR45,
            ERR46,
            ERR47,
            ERR48,
            ERR49,
            ERR50,
            ERR51,
            ERR52,
            ERR53,
            ERR54,
            ERR55,
            ERR56,
            ERR57,
            ERR58,
            ERR59,
            ERR60,
            ERR61,
            ERR62,
            ERR63,
            ERR64,
            ERR65,
            ERR66,
            ERR67,
            ERR68,
            ERR69,
            ERR70,
            ERR71,
            ERR72,
            ERR73,
            ERR74,
            ERR75,
            ERR76,
            ERR77,
            ERR78,
            ERR79,
            ERR80,
            ERR81,
            ERR82,
            ERR83,
            ERR84,
            ERR85,
            ERR86,
            ERR87,
            ERR88,
            ERR89,
            ERR90,
            ERR91,
            ERR92,
            ERR93,
            ERR94,
            ERR95,
            ERR96,
            ERR97,
            ERR98,
            ERR99,
            ERR100,
            ERR101,
            ERR102,
            ERR103,
            ERR104,
            ERR105,
            ERR106,
            ERR107,
            ERR108,
            ERR109,
            ERR110,
            ERR111,
            ERR112,
            ERR113,
            ERR114,
            ERR115,
            ERR116,
            ERR117,
            ERR118,
            ERR119,
            ERR120,
            ERR121,
            ERR122,
            ERR123,
            ERR124,
            ERR125,
            ERR126,
        ];

    }

    pub static A: [&[&str]; 2] = [&first::A, &second::A];
}

mod _p {
    pub const CAT_BITS: usize = 1;
    pub const VAR_BITS: usize = 7;
    pub const KIND_BITS: usize = CAT_BITS + VAR_BITS;
    pub type R = u8;
    pub const CAT_MAX: R = 1;
    pub const VAR_MASK: R = R::MAX >> (R::BITS as usize - VAR_BITS);
    pub static VAR_MAXES: [R; 2] = [1, 127];
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
    pub const FIRST: C = C::new(0);
    pub const SECOND: C = C::new(1);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;
    pub const ONLY: EK = EK::new(c::FIRST, 1);
    pub const ERR0: EK = EK::new(c::SECOND, 1);
    pub const ERR1: EK = EK::new(c::SECOND, 2);
    pub const ERR2: EK = EK::new(c::SECOND, 3);
    pub const ERR3: EK = EK::new(c::SECOND, 4);
    pub const ERR4: EK = EK::new(c::SECOND, 5);
    pub const ERR5: EK = EK::new(c::SECOND, 6);
    pub const ERR6: EK = EK::new(c::SECOND, 7);
    pub const ERR7: EK = EK::new(c::SECOND, 8);
    pub const ERR8: EK = EK::new(c::SECOND, 9);
    pub const ERR9: EK = EK::new(c::SECOND, 10);
    pub const ERR10: EK = EK::new(c::SECOND, 11);
    pub const ERR11: EK = EK::new(c::SECOND, 12);
    pub const ERR12: EK = EK::new(c::SECOND, 13);
    pub const ERR13: EK = EK::new(c::SECOND, 14);
    pub const ERR14: EK = EK::new(c::SECOND, 15);
    pub const ERR15: EK = EK::new(c::SECOND, 16);
    pub const ERR16: EK = EK::new(c::SECOND, 17);
    pub const ERR17: EK = EK::new(c::SECOND, 18);
    pub const ERR18: EK = EK::new(c::SECOND, 19);
    pub const ERR19: EK = EK::new(c::SECOND, 20);
    pub const ERR20: EK = EK::new(c::SECOND, 21);
    pub const ERR21: EK = EK::new(c::SECOND, 22);
    pub const ERR22: EK = EK::new(c::SECOND, 23);
    pub const ERR23: EK = EK::new(c::SECOND, 24);
    pub const ERR24: EK = EK::new(c::SECOND, 25);
    pub const ERR25: EK = EK::new(c::SECOND, 26);
    pub const ERR26: EK = EK::new(c::SECOND, 27);
    pub const ERR27: EK = EK::new(c::SECOND, 28);
    pub const ERR28: EK = EK::new(c::SECOND, 29);
    pub const ERR29: EK = EK::new(c::SECOND, 30);
    pub const ERR30: EK = EK::new(c::SECOND, 31);
    pub const ERR31: EK = EK::new(c::SECOND, 32);
    pub const ERR32: EK = EK::new(c::SECOND, 33);
    pub const ERR33: EK = EK::new(c::SECOND, 34);
    pub const ERR34: EK = EK::new(c::SECOND, 35);
    pub const ERR35: EK = EK::new(c::SECOND, 36);
    pub const ERR36: EK = EK::new(c::SECOND, 37);
    pub const ERR37: EK = EK::new(c::SECOND, 38);
    pub const ERR38: EK = EK::new(c::SECOND, 39);
    pub const ERR39: EK = EK::new(c::SECOND, 40);
    pub const ERR40: EK = EK::new(c::SECOND, 41);
    pub const ERR41: EK = EK::new(c::SECOND, 42);
    pub const ERR42: EK = EK::new(c::SECOND, 43);
    pub const ERR43: EK = EK::new(c::SECOND, 44);
    pub const ERR44: EK = EK::new(c::SECOND, 45);
    pub const ERR45: EK = EK::new(c::SECOND, 46);
    pub const ERR46: EK = EK::new(c::SECOND, 47);
    pub const ERR47: EK = EK::new(c::SECOND, 48);
    pub const ERR48: EK = EK::new(c::SECOND, 49);
    pub const ERR49: EK = EK::new(c::SECOND, 50);
    pub const ERR50: EK = EK::new(c::SECOND, 51);
    pub const ERR51: EK = EK::new(c::SECOND, 52);
    pub const ERR52: EK = EK::new(c::SECOND, 53);
    pub const ERR53: EK = EK::new(c::SECOND, 54);
    pub const ERR54: EK = EK::new(c::SECOND, 55);
    pub const ERR55: EK = EK::new(c::SECOND, 56);
    pub const ERR56: EK = EK::new(c::SECOND, 57);
    pub const ERR57: EK = EK::new(c::SECOND, 58);
    pub const ERR58: EK = EK::new(c::SECOND, 59);
    pub const ERR59: EK = EK::new(c::SECOND, 60);
    pub const ERR60: EK = EK::new(c::SECOND, 61);
    pub const ERR61: EK = EK::new(c::SECOND, 62);
    pub const ERR62: EK = EK::new(c::SECOND, 63);
    pub const ERR63: EK = EK::new(c::SECOND, 64);
    pub const ERR64: EK = EK::new(c::SECOND, 65);
    pub const ERR65: EK = EK::new(c::SECOND, 66);
    pub const ERR66: EK = EK::new(c::SECOND, 67);
    pub const ERR67: EK = EK::new(c::SECOND, 68);
    pub const ERR68: EK = EK::new(c::SECOND, 69);
    pub const ERR69: EK = EK::new(c::SECOND, 70);
    pub const ERR70: EK = EK::new(c::SECOND, 71);
    pub const ERR71: EK = EK::new(c::SECOND, 72);
    pub const ERR72: EK = EK::new(c::SECOND, 73);
    pub const ERR73: EK = EK::new(c::SECOND, 74);
    pub const ERR74: EK = EK::new(c::SECOND, 75);
    pub const ERR75: EK = EK::new(c::SECOND, 76);
    pub const ERR76: EK = EK::new(c::SECOND, 77);
    pub const ERR77: EK = EK::new(c::SECOND, 78);
    pub const ERR78: EK = EK::new(c::SECOND, 79);
    pub const ERR79: EK = EK::new(c::SECOND, 80);
    pub const ERR80: EK = EK::new(c::SECOND, 81);
    pub const ERR81: EK = EK::new(c::SECOND, 82);
    pub const ERR82: EK = EK::new(c::SECOND, 83);
    pub const ERR83: EK = EK::new(c::SECOND, 84);
    pub const ERR84: EK = EK::new(c::SECOND, 85);
    pub const ERR85: EK = EK::new(c::SECOND, 86);
    pub const ERR86: EK = EK::new(c::SECOND, 87);
    pub const ERR87: EK = EK::new(c::SECOND, 88);
    pub const ERR88: EK = EK::new(c::SECOND, 89);
    pub const ERR89: EK = EK::new(c::SECOND, 90);
    pub const ERR90: EK = EK::new(c::SECOND, 91);
    pub const ERR91: EK = EK::new(c::SECOND, 92);
    pub const ERR92: EK = EK::new(c::SECOND, 93);
    pub const ERR93: EK = EK::new(c::SECOND, 94);
    pub const ERR94: EK = EK::new(c::SECOND, 95);
    pub const ERR95: EK = EK::new(c::SECOND, 96);
    pub const ERR96: EK = EK::new(c::SECOND, 97);
    pub const ERR97: EK = EK::new(c::SECOND, 98);
    pub const ERR98: EK = EK::new(c::SECOND, 99);
    pub const ERR99: EK = EK::new(c::SECOND, 100);
    pub const ERR100: EK = EK::new(c::SECOND, 101);
    pub const ERR101: EK = EK::new(c::SECOND, 102);
    pub const ERR102: EK = EK::new(c::SECOND, 103);
    pub const ERR103: EK = EK::new(c::SECOND, 104);
    pub const ERR104: EK = EK::new(c::SECOND, 105);
    pub const ERR105: EK = EK::new(c::SECOND, 106);
    pub const ERR106: EK = EK::new(c::SECOND, 107);
    pub const ERR107: EK = EK::new(c::SECOND, 108);
    pub const ERR108: EK = EK::new(c::SECOND, 109);
    pub const ERR109: EK = EK::new(c::SECOND, 110);
    pub const ERR110: EK = EK::new(c::SECOND, 111);
    pub const ERR111: EK = EK::new(c::SECOND, 112);
    pub const ERR112: EK = EK::new(c::SECOND, 113);
    pub const ERR113: EK = EK::new(c::SECOND, 114);
    pub const ERR114: EK = EK::new(c::SECOND, 115);
    pub const ERR115: EK = EK::new(c::SECOND, 116);
    pub const ERR116: EK = EK::new(c::SECOND, 117);
    pub const ERR117: EK = EK::new(c::SECOND, 118);
    pub const ERR118: EK = EK::new(c::SECOND, 119);
    pub const ERR119: EK = EK::new(c::SECOND, 120);
    pub const ERR120: EK = EK::new(c::SECOND, 121);
    pub const ERR121: EK = EK::new(c::SECOND, 122);
    pub const ERR122: EK = EK::new(c::SECOND, 123);
    pub const ERR123: EK = EK::new(c::SECOND, 124);
    pub const ERR124: EK = EK::new(c::SECOND, 125);
    pub const ERR125: EK = EK::new(c::SECOND, 126);
    pub const ERR126: EK = EK::new(c::SECOND, 127);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_name() {
        use category::*;
        assert_eq!(FIRST.name(), "FIRST");
        assert_eq!(tighterror::Category::name(& FIRST), "FIRST");
        assert_eq!(SECOND.name(), "SECOND");
        assert_eq!(tighterror::Category::name(& SECOND), "SECOND")
    }


    #[test]
    fn test_category_display() {
        use category::*;
        assert_eq!(format!("{}", FIRST), "FIRST");
        assert_eq!(format!("{}", SECOND), "SECOND");
    }


    #[test]
    fn test_category_uniqueness() {
        use category::*;
        use std::collections::HashSet;
        let cats: [ErrorCategory; 2] = [FIRST, SECOND];
        let set = HashSet::<ErrorCategory>::from_iter(cats);
        assert_eq!(set.len(), 2);
    }


    #[test]
    fn test_category_values() {
        use category::*;
        let cats: [ErrorCategory; 2] = [FIRST, SECOND];
        for c in cats {
            assert!(c.0 <= 1);
        }

    }


    #[test]
    fn test_err_kind_name() {
        use kind::*;
        assert_eq!(ONLY.name(), "ONLY");
        assert_eq!(tighterror::Kind::name(& ONLY), "ONLY");
        assert_eq!(ERR0.name(), "ERR0");
        assert_eq!(tighterror::Kind::name(& ERR0), "ERR0");
        assert_eq!(ERR1.name(), "ERR1");
        assert_eq!(tighterror::Kind::name(& ERR1), "ERR1");
        assert_eq!(ERR2.name(), "ERR2");
        assert_eq!(tighterror::Kind::name(& ERR2), "ERR2");
        assert_eq!(ERR3.name(), "ERR3");
        assert_eq!(tighterror::Kind::name(& ERR3), "ERR3");
        assert_eq!(ERR4.name(), "ERR4");
        assert_eq!(tighterror::Kind::name(& ERR4), "ERR4");
        assert_eq!(ERR5.name(), "ERR5");
        assert_eq!(tighterror::Kind::name(& ERR5), "ERR5");
        assert_eq!(ERR6.name(), "ERR6");
        assert_eq!(tighterror::Kind::name(& ERR6), "ERR6");
        assert_eq!(ERR7.name(), "ERR7");
        assert_eq!(tighterror::Kind::name(& ERR7), "ERR7");
        assert_eq!(ERR8.name(), "ERR8");
        assert_eq!(tighterror::Kind::name(& ERR8), "ERR8");
        assert_eq!(ERR9.name(), "ERR9");
        assert_eq!(tighterror::Kind::name(& ERR9), "ERR9");
        assert_eq!(ERR10.name(), "ERR10");
        assert_eq!(tighterror::Kind::name(& ERR10), "ERR10");
        assert_eq!(ERR11.name(), "ERR11");
        assert_eq!(tighterror::Kind::name(& ERR11), "ERR11");
        assert_eq!(ERR12.name(), "ERR12");
        assert_eq!(tighterror::Kind::name(& ERR12), "ERR12");
        assert_eq!(ERR13.name(), "ERR13");
        assert_eq!(tighterror::Kind::name(& ERR13), "ERR13");
        assert_eq!(ERR14.name(), "ERR14");
        assert_eq!(tighterror::Kind::name(& ERR14), "ERR14");
        assert_eq!(ERR15.name(), "ERR15");
        assert_eq!(tighterror::Kind::name(& ERR15), "ERR15");
        assert_eq!(ERR16.name(), "ERR16");
        assert_eq!(tighterror::Kind::name(& ERR16), "ERR16");
        assert_eq!(ERR17.name(), "ERR17");
        assert_eq!(tighterror::Kind::name(& ERR17), "ERR17");
        assert_eq!(ERR18.name(), "ERR18");
        assert_eq!(tighterror::Kind::name(& ERR18), "ERR18");
        assert_eq!(ERR19.name(), "ERR19");
        assert_eq!(tighterror::Kind::name(& ERR19), "ERR19");
        assert_eq!(ERR20.name(), "ERR20");
        assert_eq!(tighterror::Kind::name(& ERR20), "ERR20");
        assert_eq!(ERR21.name(), "ERR21");
        assert_eq!(tighterror::Kind::name(& ERR21), "ERR21");
        assert_eq!(ERR22.name(), "ERR22");
        assert_eq!(tighterror::Kind::name(& ERR22), "ERR22");
        assert_eq!(ERR23.name(), "ERR23");
        assert_eq!(tighterror::Kind::name(& ERR23), "ERR23");
        assert_eq!(ERR24.name(), "ERR24");
        assert_eq!(tighterror::Kind::name(& ERR24), "ERR24");
        assert_eq!(ERR25.name(), "ERR25");
        assert_eq!(tighterror::Kind::name(& ERR25), "ERR25");
        assert_eq!(ERR26.name(), "ERR26");
        assert_eq!(tighterror::Kind::name(& ERR26), "ERR26");
        assert_eq!(ERR27.name(), "ERR27");
        assert_eq!(tighterror::Kind::name(& ERR27), "ERR27");
        assert_eq!(ERR28.name(), "ERR28");
        assert_eq!(tighterror::Kind::name(& ERR28), "ERR28");
        assert_eq!(ERR29.name(), "ERR29");
        assert_eq!(tighterror::Kind::name(& ERR29), "ERR29");
        assert_eq!(ERR30.name(), "ERR30");
        assert_eq!(tighterror::Kind::name(& ERR30), "ERR30");
        assert_eq!(ERR31.name(), "ERR31");
        assert_eq!(tighterror::Kind::name(& ERR31), "ERR31");
        assert_eq!(ERR32.name(), "ERR32");
        assert_eq!(tighterror::Kind::name(& ERR32), "ERR32");
        assert_eq!(ERR33.name(), "ERR33");
        assert_eq!(tighterror::Kind::name(& ERR33), "ERR33");
        assert_eq!(ERR34.name(), "ERR34");
        assert_eq!(tighterror::Kind::name(& ERR34), "ERR34");
        assert_eq!(ERR35.name(), "ERR35");
        assert_eq!(tighterror::Kind::name(& ERR35), "ERR35");
        assert_eq!(ERR36.name(), "ERR36");
        assert_eq!(tighterror::Kind::name(& ERR36), "ERR36");
        assert_eq!(ERR37.name(), "ERR37");
        assert_eq!(tighterror::Kind::name(& ERR37), "ERR37");
        assert_eq!(ERR38.name(), "ERR38");
        assert_eq!(tighterror::Kind::name(& ERR38), "ERR38");
        assert_eq!(ERR39.name(), "ERR39");
        assert_eq!(tighterror::Kind::name(& ERR39), "ERR39");
        assert_eq!(ERR40.name(), "ERR40");
        assert_eq!(tighterror::Kind::name(& ERR40), "ERR40");
        assert_eq!(ERR41.name(), "ERR41");
        assert_eq!(tighterror::Kind::name(& ERR41), "ERR41");
        assert_eq!(ERR42.name(), "ERR42");
        assert_eq!(tighterror::Kind::name(& ERR42), "ERR42");
        assert_eq!(ERR43.name(), "ERR43");
        assert_eq!(tighterror::Kind::name(& ERR43), "ERR43");
        assert_eq!(ERR44.name(), "ERR44");
        assert_eq!(tighterror::Kind::name(& ERR44), "ERR44");
        assert_eq!(ERR45.name(), "ERR45");
        assert_eq!(tighterror::Kind::name(& ERR45), "ERR45");
        assert_eq!(ERR46.name(), "ERR46");
        assert_eq!(tighterror::Kind::name(& ERR46), "ERR46");
        assert_eq!(ERR47.name(), "ERR47");
        assert_eq!(tighterror::Kind::name(& ERR47), "ERR47");
        assert_eq!(ERR48.name(), "ERR48");
        assert_eq!(tighterror::Kind::name(& ERR48), "ERR48");
        assert_eq!(ERR49.name(), "ERR49");
        assert_eq!(tighterror::Kind::name(& ERR49), "ERR49");
        assert_eq!(ERR50.name(), "ERR50");
        assert_eq!(tighterror::Kind::name(& ERR50), "ERR50");
        assert_eq!(ERR51.name(), "ERR51");
        assert_eq!(tighterror::Kind::name(& ERR51), "ERR51");
        assert_eq!(ERR52.name(), "ERR52");
        assert_eq!(tighterror::Kind::name(& ERR52), "ERR52");
        assert_eq!(ERR53.name(), "ERR53");
        assert_eq!(tighterror::Kind::name(& ERR53), "ERR53");
        assert_eq!(ERR54.name(), "ERR54");
        assert_eq!(tighterror::Kind::name(& ERR54), "ERR54");
        assert_eq!(ERR55.name(), "ERR55");
        assert_eq!(tighterror::Kind::name(& ERR55), "ERR55");
        assert_eq!(ERR56.name(), "ERR56");
        assert_eq!(tighterror::Kind::name(& ERR56), "ERR56");
        assert_eq!(ERR57.name(), "ERR57");
        assert_eq!(tighterror::Kind::name(& ERR57), "ERR57");
        assert_eq!(ERR58.name(), "ERR58");
        assert_eq!(tighterror::Kind::name(& ERR58), "ERR58");
        assert_eq!(ERR59.name(), "ERR59");
        assert_eq!(tighterror::Kind::name(& ERR59), "ERR59");
        assert_eq!(ERR60.name(), "ERR60");
        assert_eq!(tighterror::Kind::name(& ERR60), "ERR60");
        assert_eq!(ERR61.name(), "ERR61");
        assert_eq!(tighterror::Kind::name(& ERR61), "ERR61");
        assert_eq!(ERR62.name(), "ERR62");
        assert_eq!(tighterror::Kind::name(& ERR62), "ERR62");
        assert_eq!(ERR63.name(), "ERR63");
        assert_eq!(tighterror::Kind::name(& ERR63), "ERR63");
        assert_eq!(ERR64.name(), "ERR64");
        assert_eq!(tighterror::Kind::name(& ERR64), "ERR64");
        assert_eq!(ERR65.name(), "ERR65");
        assert_eq!(tighterror::Kind::name(& ERR65), "ERR65");
        assert_eq!(ERR66.name(), "ERR66");
        assert_eq!(tighterror::Kind::name(& ERR66), "ERR66");
        assert_eq!(ERR67.name(), "ERR67");
        assert_eq!(tighterror::Kind::name(& ERR67), "ERR67");
        assert_eq!(ERR68.name(), "ERR68");
        assert_eq!(tighterror::Kind::name(& ERR68), "ERR68");
        assert_eq!(ERR69.name(), "ERR69");
        assert_eq!(tighterror::Kind::name(& ERR69), "ERR69");
        assert_eq!(ERR70.name(), "ERR70");
        assert_eq!(tighterror::Kind::name(& ERR70), "ERR70");
        assert_eq!(ERR71.name(), "ERR71");
        assert_eq!(tighterror::Kind::name(& ERR71), "ERR71");
        assert_eq!(ERR72.name(), "ERR72");
        assert_eq!(tighterror::Kind::name(& ERR72), "ERR72");
        assert_eq!(ERR73.name(), "ERR73");
        assert_eq!(tighterror::Kind::name(& ERR73), "ERR73");
        assert_eq!(ERR74.name(), "ERR74");
        assert_eq!(tighterror::Kind::name(& ERR74), "ERR74");
        assert_eq!(ERR75.name(), "ERR75");
        assert_eq!(tighterror::Kind::name(& ERR75), "ERR75");
        assert_eq!(ERR76.name(), "ERR76");
        assert_eq!(tighterror::Kind::name(& ERR76), "ERR76");
        assert_eq!(ERR77.name(), "ERR77");
        assert_eq!(tighterror::Kind::name(& ERR77), "ERR77");
        assert_eq!(ERR78.name(), "ERR78");
        assert_eq!(tighterror::Kind::name(& ERR78), "ERR78");
        assert_eq!(ERR79.name(), "ERR79");
        assert_eq!(tighterror::Kind::name(& ERR79), "ERR79");
        assert_eq!(ERR80.name(), "ERR80");
        assert_eq!(tighterror::Kind::name(& ERR80), "ERR80");
        assert_eq!(ERR81.name(), "ERR81");
        assert_eq!(tighterror::Kind::name(& ERR81), "ERR81");
        assert_eq!(ERR82.name(), "ERR82");
        assert_eq!(tighterror::Kind::name(& ERR82), "ERR82");
        assert_eq!(ERR83.name(), "ERR83");
        assert_eq!(tighterror::Kind::name(& ERR83), "ERR83");
        assert_eq!(ERR84.name(), "ERR84");
        assert_eq!(tighterror::Kind::name(& ERR84), "ERR84");
        assert_eq!(ERR85.name(), "ERR85");
        assert_eq!(tighterror::Kind::name(& ERR85), "ERR85");
        assert_eq!(ERR86.name(), "ERR86");
        assert_eq!(tighterror::Kind::name(& ERR86), "ERR86");
        assert_eq!(ERR87.name(), "ERR87");
        assert_eq!(tighterror::Kind::name(& ERR87), "ERR87");
        assert_eq!(ERR88.name(), "ERR88");
        assert_eq!(tighterror::Kind::name(& ERR88), "ERR88");
        assert_eq!(ERR89.name(), "ERR89");
        assert_eq!(tighterror::Kind::name(& ERR89), "ERR89");
        assert_eq!(ERR90.name(), "ERR90");
        assert_eq!(tighterror::Kind::name(& ERR90), "ERR90");
        assert_eq!(ERR91.name(), "ERR91");
        assert_eq!(tighterror::Kind::name(& ERR91), "ERR91");
        assert_eq!(ERR92.name(), "ERR92");
        assert_eq!(tighterror::Kind::name(& ERR92), "ERR92");
        assert_eq!(ERR93.name(), "ERR93");
        assert_eq!(tighterror::Kind::name(& ERR93), "ERR93");
        assert_eq!(ERR94.name(), "ERR94");
        assert_eq!(tighterror::Kind::name(& ERR94), "ERR94");
        assert_eq!(ERR95.name(), "ERR95");
        assert_eq!(tighterror::Kind::name(& ERR95), "ERR95");
        assert_eq!(ERR96.name(), "ERR96");
        assert_eq!(tighterror::Kind::name(& ERR96), "ERR96");
        assert_eq!(ERR97.name(), "ERR97");
        assert_eq!(tighterror::Kind::name(& ERR97), "ERR97");
        assert_eq!(ERR98.name(), "ERR98");
        assert_eq!(tighterror::Kind::name(& ERR98), "ERR98");
        assert_eq!(ERR99.name(), "ERR99");
        assert_eq!(tighterror::Kind::name(& ERR99), "ERR99");
        assert_eq!(ERR100.name(), "ERR100");
        assert_eq!(tighterror::Kind::name(& ERR100), "ERR100");
        assert_eq!(ERR101.name(), "ERR101");
        assert_eq!(tighterror::Kind::name(& ERR101), "ERR101");
        assert_eq!(ERR102.name(), "ERR102");
        assert_eq!(tighterror::Kind::name(& ERR102), "ERR102");
        assert_eq!(ERR103.name(), "ERR103");
        assert_eq!(tighterror::Kind::name(& ERR103), "ERR103");
        assert_eq!(ERR104.name(), "ERR104");
        assert_eq!(tighterror::Kind::name(& ERR104), "ERR104");
        assert_eq!(ERR105.name(), "ERR105");
        assert_eq!(tighterror::Kind::name(& ERR105), "ERR105");
        assert_eq!(ERR106.name(), "ERR106");
        assert_eq!(tighterror::Kind::name(& ERR106), "ERR106");
        assert_eq!(ERR107.name(), "ERR107");
        assert_eq!(tighterror::Kind::name(& ERR107), "ERR107");
        assert_eq!(ERR108.name(), "ERR108");
        assert_eq!(tighterror::Kind::name(& ERR108), "ERR108");
        assert_eq!(ERR109.name(), "ERR109");
        assert_eq!(tighterror::Kind::name(& ERR109), "ERR109");
        assert_eq!(ERR110.name(), "ERR110");
        assert_eq!(tighterror::Kind::name(& ERR110), "ERR110");
        assert_eq!(ERR111.name(), "ERR111");
        assert_eq!(tighterror::Kind::name(& ERR111), "ERR111");
        assert_eq!(ERR112.name(), "ERR112");
        assert_eq!(tighterror::Kind::name(& ERR112), "ERR112");
        assert_eq!(ERR113.name(), "ERR113");
        assert_eq!(tighterror::Kind::name(& ERR113), "ERR113");
        assert_eq!(ERR114.name(), "ERR114");
        assert_eq!(tighterror::Kind::name(& ERR114), "ERR114");
        assert_eq!(ERR115.name(), "ERR115");
        assert_eq!(tighterror::Kind::name(& ERR115), "ERR115");
        assert_eq!(ERR116.name(), "ERR116");
        assert_eq!(tighterror::Kind::name(& ERR116), "ERR116");
        assert_eq!(ERR117.name(), "ERR117");
        assert_eq!(tighterror::Kind::name(& ERR117), "ERR117");
        assert_eq!(ERR118.name(), "ERR118");
        assert_eq!(tighterror::Kind::name(& ERR118), "ERR118");
        assert_eq!(ERR119.name(), "ERR119");
        assert_eq!(tighterror::Kind::name(& ERR119), "ERR119");
        assert_eq!(ERR120.name(), "ERR120");
        assert_eq!(tighterror::Kind::name(& ERR120), "ERR120");
        assert_eq!(ERR121.name(), "ERR121");
        assert_eq!(tighterror::Kind::name(& ERR121), "ERR121");
        assert_eq!(ERR122.name(), "ERR122");
        assert_eq!(tighterror::Kind::name(& ERR122), "ERR122");
        assert_eq!(ERR123.name(), "ERR123");
        assert_eq!(tighterror::Kind::name(& ERR123), "ERR123");
        assert_eq!(ERR124.name(), "ERR124");
        assert_eq!(tighterror::Kind::name(& ERR124), "ERR124");
        assert_eq!(ERR125.name(), "ERR125");
        assert_eq!(tighterror::Kind::name(& ERR125), "ERR125");
        assert_eq!(ERR126.name(), "ERR126");
        assert_eq!(tighterror::Kind::name(& ERR126), "ERR126");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", ONLY), "ONLY");
        assert_eq!(format!("{}", ERR0), "ERR0");
        assert_eq!(format!("{}", ERR1), "ERR1");
        assert_eq!(format!("{}", ERR2), "ERR2");
        assert_eq!(format!("{}", ERR3), "ERR3");
        assert_eq!(format!("{}", ERR4), "ERR4");
        assert_eq!(format!("{}", ERR5), "ERR5");
        assert_eq!(format!("{}", ERR6), "ERR6");
        assert_eq!(format!("{}", ERR7), "ERR7");
        assert_eq!(format!("{}", ERR8), "ERR8");
        assert_eq!(format!("{}", ERR9), "ERR9");
        assert_eq!(format!("{}", ERR10), "ERR10");
        assert_eq!(format!("{}", ERR11), "ERR11");
        assert_eq!(format!("{}", ERR12), "ERR12");
        assert_eq!(format!("{}", ERR13), "ERR13");
        assert_eq!(format!("{}", ERR14), "ERR14");
        assert_eq!(format!("{}", ERR15), "ERR15");
        assert_eq!(format!("{}", ERR16), "ERR16");
        assert_eq!(format!("{}", ERR17), "ERR17");
        assert_eq!(format!("{}", ERR18), "ERR18");
        assert_eq!(format!("{}", ERR19), "ERR19");
        assert_eq!(format!("{}", ERR20), "ERR20");
        assert_eq!(format!("{}", ERR21), "ERR21");
        assert_eq!(format!("{}", ERR22), "ERR22");
        assert_eq!(format!("{}", ERR23), "ERR23");
        assert_eq!(format!("{}", ERR24), "ERR24");
        assert_eq!(format!("{}", ERR25), "ERR25");
        assert_eq!(format!("{}", ERR26), "ERR26");
        assert_eq!(format!("{}", ERR27), "ERR27");
        assert_eq!(format!("{}", ERR28), "ERR28");
        assert_eq!(format!("{}", ERR29), "ERR29");
        assert_eq!(format!("{}", ERR30), "ERR30");
        assert_eq!(format!("{}", ERR31), "ERR31");
        assert_eq!(format!("{}", ERR32), "ERR32");
        assert_eq!(format!("{}", ERR33), "ERR33");
        assert_eq!(format!("{}", ERR34), "ERR34");
        assert_eq!(format!("{}", ERR35), "ERR35");
        assert_eq!(format!("{}", ERR36), "ERR36");
        assert_eq!(format!("{}", ERR37), "ERR37");
        assert_eq!(format!("{}", ERR38), "ERR38");
        assert_eq!(format!("{}", ERR39), "ERR39");
        assert_eq!(format!("{}", ERR40), "ERR40");
        assert_eq!(format!("{}", ERR41), "ERR41");
        assert_eq!(format!("{}", ERR42), "ERR42");
        assert_eq!(format!("{}", ERR43), "ERR43");
        assert_eq!(format!("{}", ERR44), "ERR44");
        assert_eq!(format!("{}", ERR45), "ERR45");
        assert_eq!(format!("{}", ERR46), "ERR46");
        assert_eq!(format!("{}", ERR47), "ERR47");
        assert_eq!(format!("{}", ERR48), "ERR48");
        assert_eq!(format!("{}", ERR49), "ERR49");
        assert_eq!(format!("{}", ERR50), "ERR50");
        assert_eq!(format!("{}", ERR51), "ERR51");
        assert_eq!(format!("{}", ERR52), "ERR52");
        assert_eq!(format!("{}", ERR53), "ERR53");
        assert_eq!(format!("{}", ERR54), "ERR54");
        assert_eq!(format!("{}", ERR55), "ERR55");
        assert_eq!(format!("{}", ERR56), "ERR56");
        assert_eq!(format!("{}", ERR57), "ERR57");
        assert_eq!(format!("{}", ERR58), "ERR58");
        assert_eq!(format!("{}", ERR59), "ERR59");
        assert_eq!(format!("{}", ERR60), "ERR60");
        assert_eq!(format!("{}", ERR61), "ERR61");
        assert_eq!(format!("{}", ERR62), "ERR62");
        assert_eq!(format!("{}", ERR63), "ERR63");
        assert_eq!(format!("{}", ERR64), "ERR64");
        assert_eq!(format!("{}", ERR65), "ERR65");
        assert_eq!(format!("{}", ERR66), "ERR66");
        assert_eq!(format!("{}", ERR67), "ERR67");
        assert_eq!(format!("{}", ERR68), "ERR68");
        assert_eq!(format!("{}", ERR69), "ERR69");
        assert_eq!(format!("{}", ERR70), "ERR70");
        assert_eq!(format!("{}", ERR71), "ERR71");
        assert_eq!(format!("{}", ERR72), "ERR72");
        assert_eq!(format!("{}", ERR73), "ERR73");
        assert_eq!(format!("{}", ERR74), "ERR74");
        assert_eq!(format!("{}", ERR75), "ERR75");
        assert_eq!(format!("{}", ERR76), "ERR76");
        assert_eq!(format!("{}", ERR77), "ERR77");
        assert_eq!(format!("{}", ERR78), "ERR78");
        assert_eq!(format!("{}", ERR79), "ERR79");
        assert_eq!(format!("{}", ERR80), "ERR80");
        assert_eq!(format!("{}", ERR81), "ERR81");
        assert_eq!(format!("{}", ERR82), "ERR82");
        assert_eq!(format!("{}", ERR83), "ERR83");
        assert_eq!(format!("{}", ERR84), "ERR84");
        assert_eq!(format!("{}", ERR85), "ERR85");
        assert_eq!(format!("{}", ERR86), "ERR86");
        assert_eq!(format!("{}", ERR87), "ERR87");
        assert_eq!(format!("{}", ERR88), "ERR88");
        assert_eq!(format!("{}", ERR89), "ERR89");
        assert_eq!(format!("{}", ERR90), "ERR90");
        assert_eq!(format!("{}", ERR91), "ERR91");
        assert_eq!(format!("{}", ERR92), "ERR92");
        assert_eq!(format!("{}", ERR93), "ERR93");
        assert_eq!(format!("{}", ERR94), "ERR94");
        assert_eq!(format!("{}", ERR95), "ERR95");
        assert_eq!(format!("{}", ERR96), "ERR96");
        assert_eq!(format!("{}", ERR97), "ERR97");
        assert_eq!(format!("{}", ERR98), "ERR98");
        assert_eq!(format!("{}", ERR99), "ERR99");
        assert_eq!(format!("{}", ERR100), "ERR100");
        assert_eq!(format!("{}", ERR101), "ERR101");
        assert_eq!(format!("{}", ERR102), "ERR102");
        assert_eq!(format!("{}", ERR103), "ERR103");
        assert_eq!(format!("{}", ERR104), "ERR104");
        assert_eq!(format!("{}", ERR105), "ERR105");
        assert_eq!(format!("{}", ERR106), "ERR106");
        assert_eq!(format!("{}", ERR107), "ERR107");
        assert_eq!(format!("{}", ERR108), "ERR108");
        assert_eq!(format!("{}", ERR109), "ERR109");
        assert_eq!(format!("{}", ERR110), "ERR110");
        assert_eq!(format!("{}", ERR111), "ERR111");
        assert_eq!(format!("{}", ERR112), "ERR112");
        assert_eq!(format!("{}", ERR113), "ERR113");
        assert_eq!(format!("{}", ERR114), "ERR114");
        assert_eq!(format!("{}", ERR115), "ERR115");
        assert_eq!(format!("{}", ERR116), "ERR116");
        assert_eq!(format!("{}", ERR117), "ERR117");
        assert_eq!(format!("{}", ERR118), "ERR118");
        assert_eq!(format!("{}", ERR119), "ERR119");
        assert_eq!(format!("{}", ERR120), "ERR120");
        assert_eq!(format!("{}", ERR121), "ERR121");
        assert_eq!(format!("{}", ERR122), "ERR122");
        assert_eq!(format!("{}", ERR123), "ERR123");
        assert_eq!(format!("{}", ERR124), "ERR124");
        assert_eq!(format!("{}", ERR125), "ERR125");
        assert_eq!(format!("{}", ERR126), "ERR126");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 128] = [
            ONLY,
            ERR0,
            ERR1,
            ERR2,
            ERR3,
            ERR4,
            ERR5,
            ERR6,
            ERR7,
            ERR8,
            ERR9,
            ERR10,
            ERR11,
            ERR12,
            ERR13,
            ERR14,
            ERR15,
            ERR16,
            ERR17,
            ERR18,
            ERR19,
            ERR20,
            ERR21,
            ERR22,
            ERR23,
            ERR24,
            ERR25,
            ERR26,
            ERR27,
            ERR28,
            ERR29,
            ERR30,
            ERR31,
            ERR32,
            ERR33,
            ERR34,
            ERR35,
            ERR36,
            ERR37,
            ERR38,
            ERR39,
            ERR40,
            ERR41,
            ERR42,
            ERR43,
            ERR44,
            ERR45,
            ERR46,
            ERR47,
            ERR48,
            ERR49,
            ERR50,
            ERR51,
            ERR52,
            ERR53,
            ERR54,
            ERR55,
            ERR56,
            ERR57,
            ERR58,
            ERR59,
            ERR60,
            ERR61,
            ERR62,
            ERR63,
            ERR64,
            ERR65,
            ERR66,
            ERR67,
            ERR68,
            ERR69,
            ERR70,
            ERR71,
            ERR72,
            ERR73,
            ERR74,
            ERR75,
            ERR76,
            ERR77,
            ERR78,
            ERR79,
            ERR80,
            ERR81,
            ERR82,
            ERR83,
            ERR84,
            ERR85,
            ERR86,
            ERR87,
            ERR88,
            ERR89,
            ERR90,
            ERR91,
            ERR92,
            ERR93,
            ERR94,
            ERR95,
            ERR96,
            ERR97,
            ERR98,
            ERR99,
            ERR100,
            ERR101,
            ERR102,
            ERR103,
            ERR104,
            ERR105,
            ERR106,
            ERR107,
            ERR108,
            ERR109,
            ERR110,
            ERR111,
            ERR112,
            ERR113,
            ERR114,
            ERR115,
            ERR116,
            ERR117,
            ERR118,
            ERR119,
            ERR120,
            ERR121,
            ERR122,
            ERR123,
            ERR124,
            ERR125,
            ERR126,
        ];

        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 128);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 128] = [
            ONLY,
            ERR0,
            ERR1,
            ERR2,
            ERR3,
            ERR4,
            ERR5,
            ERR6,
            ERR7,
            ERR8,
            ERR9,
            ERR10,
            ERR11,
            ERR12,
            ERR13,
            ERR14,
            ERR15,
            ERR16,
            ERR17,
            ERR18,
            ERR19,
            ERR20,
            ERR21,
            ERR22,
            ERR23,
            ERR24,
            ERR25,
            ERR26,
            ERR27,
            ERR28,
            ERR29,
            ERR30,
            ERR31,
            ERR32,
            ERR33,
            ERR34,
            ERR35,
            ERR36,
            ERR37,
            ERR38,
            ERR39,
            ERR40,
            ERR41,
            ERR42,
            ERR43,
            ERR44,
            ERR45,
            ERR46,
            ERR47,
            ERR48,
            ERR49,
            ERR50,
            ERR51,
            ERR52,
            ERR53,
            ERR54,
            ERR55,
            ERR56,
            ERR57,
            ERR58,
            ERR59,
            ERR60,
            ERR61,
            ERR62,
            ERR63,
            ERR64,
            ERR65,
            ERR66,
            ERR67,
            ERR68,
            ERR69,
            ERR70,
            ERR71,
            ERR72,
            ERR73,
            ERR74,
            ERR75,
            ERR76,
            ERR77,
            ERR78,
            ERR79,
            ERR80,
            ERR81,
            ERR82,
            ERR83,
            ERR84,
            ERR85,
            ERR86,
            ERR87,
            ERR88,
            ERR89,
            ERR90,
            ERR91,
            ERR92,
            ERR93,
            ERR94,
            ERR95,
            ERR96,
            ERR97,
            ERR98,
            ERR99,
            ERR100,
            ERR101,
            ERR102,
            ERR103,
            ERR104,
            ERR105,
            ERR106,
            ERR107,
            ERR108,
            ERR109,
            ERR110,
            ERR111,
            ERR112,
            ERR113,
            ERR114,
            ERR115,
            ERR116,
            ERR117,
            ERR118,
            ERR119,
            ERR120,
            ERR121,
            ERR122,
            ERR123,
            ERR124,
            ERR125,
            ERR126,
        ];

        let set = HashSet::<_p::R>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 128);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(ONLY.category(), category::FIRST);
        assert_eq!(ERR0.category(), category::SECOND);
        assert_eq!(ERR1.category(), category::SECOND);
        assert_eq!(ERR2.category(), category::SECOND);
        assert_eq!(ERR3.category(), category::SECOND);
        assert_eq!(ERR4.category(), category::SECOND);
        assert_eq!(ERR5.category(), category::SECOND);
        assert_eq!(ERR6.category(), category::SECOND);
        assert_eq!(ERR7.category(), category::SECOND);
        assert_eq!(ERR8.category(), category::SECOND);
        assert_eq!(ERR9.category(), category::SECOND);
        assert_eq!(ERR10.category(), category::SECOND);
        assert_eq!(ERR11.category(), category::SECOND);
        assert_eq!(ERR12.category(), category::SECOND);
        assert_eq!(ERR13.category(), category::SECOND);
        assert_eq!(ERR14.category(), category::SECOND);
        assert_eq!(ERR15.category(), category::SECOND);
        assert_eq!(ERR16.category(), category::SECOND);
        assert_eq!(ERR17.category(), category::SECOND);
        assert_eq!(ERR18.category(), category::SECOND);
        assert_eq!(ERR19.category(), category::SECOND);
        assert_eq!(ERR20.category(), category::SECOND);
        assert_eq!(ERR21.category(), category::SECOND);
        assert_eq!(ERR22.category(), category::SECOND);
        assert_eq!(ERR23.category(), category::SECOND);
        assert_eq!(ERR24.category(), category::SECOND);
        assert_eq!(ERR25.category(), category::SECOND);
        assert_eq!(ERR26.category(), category::SECOND);
        assert_eq!(ERR27.category(), category::SECOND);
        assert_eq!(ERR28.category(), category::SECOND);
        assert_eq!(ERR29.category(), category::SECOND);
        assert_eq!(ERR30.category(), category::SECOND);
        assert_eq!(ERR31.category(), category::SECOND);
        assert_eq!(ERR32.category(), category::SECOND);
        assert_eq!(ERR33.category(), category::SECOND);
        assert_eq!(ERR34.category(), category::SECOND);
        assert_eq!(ERR35.category(), category::SECOND);
        assert_eq!(ERR36.category(), category::SECOND);
        assert_eq!(ERR37.category(), category::SECOND);
        assert_eq!(ERR38.category(), category::SECOND);
        assert_eq!(ERR39.category(), category::SECOND);
        assert_eq!(ERR40.category(), category::SECOND);
        assert_eq!(ERR41.category(), category::SECOND);
        assert_eq!(ERR42.category(), category::SECOND);
        assert_eq!(ERR43.category(), category::SECOND);
        assert_eq!(ERR44.category(), category::SECOND);
        assert_eq!(ERR45.category(), category::SECOND);
        assert_eq!(ERR46.category(), category::SECOND);
        assert_eq!(ERR47.category(), category::SECOND);
        assert_eq!(ERR48.category(), category::SECOND);
        assert_eq!(ERR49.category(), category::SECOND);
        assert_eq!(ERR50.category(), category::SECOND);
        assert_eq!(ERR51.category(), category::SECOND);
        assert_eq!(ERR52.category(), category::SECOND);
        assert_eq!(ERR53.category(), category::SECOND);
        assert_eq!(ERR54.category(), category::SECOND);
        assert_eq!(ERR55.category(), category::SECOND);
        assert_eq!(ERR56.category(), category::SECOND);
        assert_eq!(ERR57.category(), category::SECOND);
        assert_eq!(ERR58.category(), category::SECOND);
        assert_eq!(ERR59.category(), category::SECOND);
        assert_eq!(ERR60.category(), category::SECOND);
        assert_eq!(ERR61.category(), category::SECOND);
        assert_eq!(ERR62.category(), category::SECOND);
        assert_eq!(ERR63.category(), category::SECOND);
        assert_eq!(ERR64.category(), category::SECOND);
        assert_eq!(ERR65.category(), category::SECOND);
        assert_eq!(ERR66.category(), category::SECOND);
        assert_eq!(ERR67.category(), category::SECOND);
        assert_eq!(ERR68.category(), category::SECOND);
        assert_eq!(ERR69.category(), category::SECOND);
        assert_eq!(ERR70.category(), category::SECOND);
        assert_eq!(ERR71.category(), category::SECOND);
        assert_eq!(ERR72.category(), category::SECOND);
        assert_eq!(ERR73.category(), category::SECOND);
        assert_eq!(ERR74.category(), category::SECOND);
        assert_eq!(ERR75.category(), category::SECOND);
        assert_eq!(ERR76.category(), category::SECOND);
        assert_eq!(ERR77.category(), category::SECOND);
        assert_eq!(ERR78.category(), category::SECOND);
        assert_eq!(ERR79.category(), category::SECOND);
        assert_eq!(ERR80.category(), category::SECOND);
        assert_eq!(ERR81.category(), category::SECOND);
        assert_eq!(ERR82.category(), category::SECOND);
        assert_eq!(ERR83.category(), category::SECOND);
        assert_eq!(ERR84.category(), category::SECOND);
        assert_eq!(ERR85.category(), category::SECOND);
        assert_eq!(ERR86.category(), category::SECOND);
        assert_eq!(ERR87.category(), category::SECOND);
        assert_eq!(ERR88.category(), category::SECOND);
        assert_eq!(ERR89.category(), category::SECOND);
        assert_eq!(ERR90.category(), category::SECOND);
        assert_eq!(ERR91.category(), category::SECOND);
        assert_eq!(ERR92.category(), category::SECOND);
        assert_eq!(ERR93.category(), category::SECOND);
        assert_eq!(ERR94.category(), category::SECOND);
        assert_eq!(ERR95.category(), category::SECOND);
        assert_eq!(ERR96.category(), category::SECOND);
        assert_eq!(ERR97.category(), category::SECOND);
        assert_eq!(ERR98.category(), category::SECOND);
        assert_eq!(ERR99.category(), category::SECOND);
        assert_eq!(ERR100.category(), category::SECOND);
        assert_eq!(ERR101.category(), category::SECOND);
        assert_eq!(ERR102.category(), category::SECOND);
        assert_eq!(ERR103.category(), category::SECOND);
        assert_eq!(ERR104.category(), category::SECOND);
        assert_eq!(ERR105.category(), category::SECOND);
        assert_eq!(ERR106.category(), category::SECOND);
        assert_eq!(ERR107.category(), category::SECOND);
        assert_eq!(ERR108.category(), category::SECOND);
        assert_eq!(ERR109.category(), category::SECOND);
        assert_eq!(ERR110.category(), category::SECOND);
        assert_eq!(ERR111.category(), category::SECOND);
        assert_eq!(ERR112.category(), category::SECOND);
        assert_eq!(ERR113.category(), category::SECOND);
        assert_eq!(ERR114.category(), category::SECOND);
        assert_eq!(ERR115.category(), category::SECOND);
        assert_eq!(ERR116.category(), category::SECOND);
        assert_eq!(ERR117.category(), category::SECOND);
        assert_eq!(ERR118.category(), category::SECOND);
        assert_eq!(ERR119.category(), category::SECOND);
        assert_eq!(ERR120.category(), category::SECOND);
        assert_eq!(ERR121.category(), category::SECOND);
        assert_eq!(ERR122.category(), category::SECOND);
        assert_eq!(ERR123.category(), category::SECOND);
        assert_eq!(ERR124.category(), category::SECOND);
        assert_eq!(ERR125.category(), category::SECOND);
        assert_eq!(ERR126.category(), category::SECOND);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(ErrorKind::from_value(ONLY.value()).unwrap(), ONLY);
        assert_eq!(ErrorKind::from_value(ERR0.value()).unwrap(), ERR0);
        assert_eq!(ErrorKind::from_value(ERR1.value()).unwrap(), ERR1);
        assert_eq!(ErrorKind::from_value(ERR2.value()).unwrap(), ERR2);
        assert_eq!(ErrorKind::from_value(ERR3.value()).unwrap(), ERR3);
        assert_eq!(ErrorKind::from_value(ERR4.value()).unwrap(), ERR4);
        assert_eq!(ErrorKind::from_value(ERR5.value()).unwrap(), ERR5);
        assert_eq!(ErrorKind::from_value(ERR6.value()).unwrap(), ERR6);
        assert_eq!(ErrorKind::from_value(ERR7.value()).unwrap(), ERR7);
        assert_eq!(ErrorKind::from_value(ERR8.value()).unwrap(), ERR8);
        assert_eq!(ErrorKind::from_value(ERR9.value()).unwrap(), ERR9);
        assert_eq!(ErrorKind::from_value(ERR10.value()).unwrap(), ERR10);
        assert_eq!(ErrorKind::from_value(ERR11.value()).unwrap(), ERR11);
        assert_eq!(ErrorKind::from_value(ERR12.value()).unwrap(), ERR12);
        assert_eq!(ErrorKind::from_value(ERR13.value()).unwrap(), ERR13);
        assert_eq!(ErrorKind::from_value(ERR14.value()).unwrap(), ERR14);
        assert_eq!(ErrorKind::from_value(ERR15.value()).unwrap(), ERR15);
        assert_eq!(ErrorKind::from_value(ERR16.value()).unwrap(), ERR16);
        assert_eq!(ErrorKind::from_value(ERR17.value()).unwrap(), ERR17);
        assert_eq!(ErrorKind::from_value(ERR18.value()).unwrap(), ERR18);
        assert_eq!(ErrorKind::from_value(ERR19.value()).unwrap(), ERR19);
        assert_eq!(ErrorKind::from_value(ERR20.value()).unwrap(), ERR20);
        assert_eq!(ErrorKind::from_value(ERR21.value()).unwrap(), ERR21);
        assert_eq!(ErrorKind::from_value(ERR22.value()).unwrap(), ERR22);
        assert_eq!(ErrorKind::from_value(ERR23.value()).unwrap(), ERR23);
        assert_eq!(ErrorKind::from_value(ERR24.value()).unwrap(), ERR24);
        assert_eq!(ErrorKind::from_value(ERR25.value()).unwrap(), ERR25);
        assert_eq!(ErrorKind::from_value(ERR26.value()).unwrap(), ERR26);
        assert_eq!(ErrorKind::from_value(ERR27.value()).unwrap(), ERR27);
        assert_eq!(ErrorKind::from_value(ERR28.value()).unwrap(), ERR28);
        assert_eq!(ErrorKind::from_value(ERR29.value()).unwrap(), ERR29);
        assert_eq!(ErrorKind::from_value(ERR30.value()).unwrap(), ERR30);
        assert_eq!(ErrorKind::from_value(ERR31.value()).unwrap(), ERR31);
        assert_eq!(ErrorKind::from_value(ERR32.value()).unwrap(), ERR32);
        assert_eq!(ErrorKind::from_value(ERR33.value()).unwrap(), ERR33);
        assert_eq!(ErrorKind::from_value(ERR34.value()).unwrap(), ERR34);
        assert_eq!(ErrorKind::from_value(ERR35.value()).unwrap(), ERR35);
        assert_eq!(ErrorKind::from_value(ERR36.value()).unwrap(), ERR36);
        assert_eq!(ErrorKind::from_value(ERR37.value()).unwrap(), ERR37);
        assert_eq!(ErrorKind::from_value(ERR38.value()).unwrap(), ERR38);
        assert_eq!(ErrorKind::from_value(ERR39.value()).unwrap(), ERR39);
        assert_eq!(ErrorKind::from_value(ERR40.value()).unwrap(), ERR40);
        assert_eq!(ErrorKind::from_value(ERR41.value()).unwrap(), ERR41);
        assert_eq!(ErrorKind::from_value(ERR42.value()).unwrap(), ERR42);
        assert_eq!(ErrorKind::from_value(ERR43.value()).unwrap(), ERR43);
        assert_eq!(ErrorKind::from_value(ERR44.value()).unwrap(), ERR44);
        assert_eq!(ErrorKind::from_value(ERR45.value()).unwrap(), ERR45);
        assert_eq!(ErrorKind::from_value(ERR46.value()).unwrap(), ERR46);
        assert_eq!(ErrorKind::from_value(ERR47.value()).unwrap(), ERR47);
        assert_eq!(ErrorKind::from_value(ERR48.value()).unwrap(), ERR48);
        assert_eq!(ErrorKind::from_value(ERR49.value()).unwrap(), ERR49);
        assert_eq!(ErrorKind::from_value(ERR50.value()).unwrap(), ERR50);
        assert_eq!(ErrorKind::from_value(ERR51.value()).unwrap(), ERR51);
        assert_eq!(ErrorKind::from_value(ERR52.value()).unwrap(), ERR52);
        assert_eq!(ErrorKind::from_value(ERR53.value()).unwrap(), ERR53);
        assert_eq!(ErrorKind::from_value(ERR54.value()).unwrap(), ERR54);
        assert_eq!(ErrorKind::from_value(ERR55.value()).unwrap(), ERR55);
        assert_eq!(ErrorKind::from_value(ERR56.value()).unwrap(), ERR56);
        assert_eq!(ErrorKind::from_value(ERR57.value()).unwrap(), ERR57);
        assert_eq!(ErrorKind::from_value(ERR58.value()).unwrap(), ERR58);
        assert_eq!(ErrorKind::from_value(ERR59.value()).unwrap(), ERR59);
        assert_eq!(ErrorKind::from_value(ERR60.value()).unwrap(), ERR60);
        assert_eq!(ErrorKind::from_value(ERR61.value()).unwrap(), ERR61);
        assert_eq!(ErrorKind::from_value(ERR62.value()).unwrap(), ERR62);
        assert_eq!(ErrorKind::from_value(ERR63.value()).unwrap(), ERR63);
        assert_eq!(ErrorKind::from_value(ERR64.value()).unwrap(), ERR64);
        assert_eq!(ErrorKind::from_value(ERR65.value()).unwrap(), ERR65);
        assert_eq!(ErrorKind::from_value(ERR66.value()).unwrap(), ERR66);
        assert_eq!(ErrorKind::from_value(ERR67.value()).unwrap(), ERR67);
        assert_eq!(ErrorKind::from_value(ERR68.value()).unwrap(), ERR68);
        assert_eq!(ErrorKind::from_value(ERR69.value()).unwrap(), ERR69);
        assert_eq!(ErrorKind::from_value(ERR70.value()).unwrap(), ERR70);
        assert_eq!(ErrorKind::from_value(ERR71.value()).unwrap(), ERR71);
        assert_eq!(ErrorKind::from_value(ERR72.value()).unwrap(), ERR72);
        assert_eq!(ErrorKind::from_value(ERR73.value()).unwrap(), ERR73);
        assert_eq!(ErrorKind::from_value(ERR74.value()).unwrap(), ERR74);
        assert_eq!(ErrorKind::from_value(ERR75.value()).unwrap(), ERR75);
        assert_eq!(ErrorKind::from_value(ERR76.value()).unwrap(), ERR76);
        assert_eq!(ErrorKind::from_value(ERR77.value()).unwrap(), ERR77);
        assert_eq!(ErrorKind::from_value(ERR78.value()).unwrap(), ERR78);
        assert_eq!(ErrorKind::from_value(ERR79.value()).unwrap(), ERR79);
        assert_eq!(ErrorKind::from_value(ERR80.value()).unwrap(), ERR80);
        assert_eq!(ErrorKind::from_value(ERR81.value()).unwrap(), ERR81);
        assert_eq!(ErrorKind::from_value(ERR82.value()).unwrap(), ERR82);
        assert_eq!(ErrorKind::from_value(ERR83.value()).unwrap(), ERR83);
        assert_eq!(ErrorKind::from_value(ERR84.value()).unwrap(), ERR84);
        assert_eq!(ErrorKind::from_value(ERR85.value()).unwrap(), ERR85);
        assert_eq!(ErrorKind::from_value(ERR86.value()).unwrap(), ERR86);
        assert_eq!(ErrorKind::from_value(ERR87.value()).unwrap(), ERR87);
        assert_eq!(ErrorKind::from_value(ERR88.value()).unwrap(), ERR88);
        assert_eq!(ErrorKind::from_value(ERR89.value()).unwrap(), ERR89);
        assert_eq!(ErrorKind::from_value(ERR90.value()).unwrap(), ERR90);
        assert_eq!(ErrorKind::from_value(ERR91.value()).unwrap(), ERR91);
        assert_eq!(ErrorKind::from_value(ERR92.value()).unwrap(), ERR92);
        assert_eq!(ErrorKind::from_value(ERR93.value()).unwrap(), ERR93);
        assert_eq!(ErrorKind::from_value(ERR94.value()).unwrap(), ERR94);
        assert_eq!(ErrorKind::from_value(ERR95.value()).unwrap(), ERR95);
        assert_eq!(ErrorKind::from_value(ERR96.value()).unwrap(), ERR96);
        assert_eq!(ErrorKind::from_value(ERR97.value()).unwrap(), ERR97);
        assert_eq!(ErrorKind::from_value(ERR98.value()).unwrap(), ERR98);
        assert_eq!(ErrorKind::from_value(ERR99.value()).unwrap(), ERR99);
        assert_eq!(ErrorKind::from_value(ERR100.value()).unwrap(), ERR100);
        assert_eq!(ErrorKind::from_value(ERR101.value()).unwrap(), ERR101);
        assert_eq!(ErrorKind::from_value(ERR102.value()).unwrap(), ERR102);
        assert_eq!(ErrorKind::from_value(ERR103.value()).unwrap(), ERR103);
        assert_eq!(ErrorKind::from_value(ERR104.value()).unwrap(), ERR104);
        assert_eq!(ErrorKind::from_value(ERR105.value()).unwrap(), ERR105);
        assert_eq!(ErrorKind::from_value(ERR106.value()).unwrap(), ERR106);
        assert_eq!(ErrorKind::from_value(ERR107.value()).unwrap(), ERR107);
        assert_eq!(ErrorKind::from_value(ERR108.value()).unwrap(), ERR108);
        assert_eq!(ErrorKind::from_value(ERR109.value()).unwrap(), ERR109);
        assert_eq!(ErrorKind::from_value(ERR110.value()).unwrap(), ERR110);
        assert_eq!(ErrorKind::from_value(ERR111.value()).unwrap(), ERR111);
        assert_eq!(ErrorKind::from_value(ERR112.value()).unwrap(), ERR112);
        assert_eq!(ErrorKind::from_value(ERR113.value()).unwrap(), ERR113);
        assert_eq!(ErrorKind::from_value(ERR114.value()).unwrap(), ERR114);
        assert_eq!(ErrorKind::from_value(ERR115.value()).unwrap(), ERR115);
        assert_eq!(ErrorKind::from_value(ERR116.value()).unwrap(), ERR116);
        assert_eq!(ErrorKind::from_value(ERR117.value()).unwrap(), ERR117);
        assert_eq!(ErrorKind::from_value(ERR118.value()).unwrap(), ERR118);
        assert_eq!(ErrorKind::from_value(ERR119.value()).unwrap(), ERR119);
        assert_eq!(ErrorKind::from_value(ERR120.value()).unwrap(), ERR120);
        assert_eq!(ErrorKind::from_value(ERR121.value()).unwrap(), ERR121);
        assert_eq!(ErrorKind::from_value(ERR122.value()).unwrap(), ERR122);
        assert_eq!(ErrorKind::from_value(ERR123.value()).unwrap(), ERR123);
        assert_eq!(ErrorKind::from_value(ERR124.value()).unwrap(), ERR124);
        assert_eq!(ErrorKind::from_value(ERR125.value()).unwrap(), ERR125);
        assert_eq!(ErrorKind::from_value(ERR126.value()).unwrap(), ERR126);
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(format!("{}", Error::from(ONLY)), "ONLY");
        assert_eq!(format!("{}", Error::from(ERR0)), "ERR0");
        assert_eq!(format!("{}", Error::from(ERR1)), "ERR1");
        assert_eq!(format!("{}", Error::from(ERR2)), "ERR2");
        assert_eq!(format!("{}", Error::from(ERR3)), "ERR3");
        assert_eq!(format!("{}", Error::from(ERR4)), "ERR4");
        assert_eq!(format!("{}", Error::from(ERR5)), "ERR5");
        assert_eq!(format!("{}", Error::from(ERR6)), "ERR6");
        assert_eq!(format!("{}", Error::from(ERR7)), "ERR7");
        assert_eq!(format!("{}", Error::from(ERR8)), "ERR8");
        assert_eq!(format!("{}", Error::from(ERR9)), "ERR9");
        assert_eq!(format!("{}", Error::from(ERR10)), "ERR10");
        assert_eq!(format!("{}", Error::from(ERR11)), "ERR11");
        assert_eq!(format!("{}", Error::from(ERR12)), "ERR12");
        assert_eq!(format!("{}", Error::from(ERR13)), "ERR13");
        assert_eq!(format!("{}", Error::from(ERR14)), "ERR14");
        assert_eq!(format!("{}", Error::from(ERR15)), "ERR15");
        assert_eq!(format!("{}", Error::from(ERR16)), "ERR16");
        assert_eq!(format!("{}", Error::from(ERR17)), "ERR17");
        assert_eq!(format!("{}", Error::from(ERR18)), "ERR18");
        assert_eq!(format!("{}", Error::from(ERR19)), "ERR19");
        assert_eq!(format!("{}", Error::from(ERR20)), "ERR20");
        assert_eq!(format!("{}", Error::from(ERR21)), "ERR21");
        assert_eq!(format!("{}", Error::from(ERR22)), "ERR22");
        assert_eq!(format!("{}", Error::from(ERR23)), "ERR23");
        assert_eq!(format!("{}", Error::from(ERR24)), "ERR24");
        assert_eq!(format!("{}", Error::from(ERR25)), "ERR25");
        assert_eq!(format!("{}", Error::from(ERR26)), "ERR26");
        assert_eq!(format!("{}", Error::from(ERR27)), "ERR27");
        assert_eq!(format!("{}", Error::from(ERR28)), "ERR28");
        assert_eq!(format!("{}", Error::from(ERR29)), "ERR29");
        assert_eq!(format!("{}", Error::from(ERR30)), "ERR30");
        assert_eq!(format!("{}", Error::from(ERR31)), "ERR31");
        assert_eq!(format!("{}", Error::from(ERR32)), "ERR32");
        assert_eq!(format!("{}", Error::from(ERR33)), "ERR33");
        assert_eq!(format!("{}", Error::from(ERR34)), "ERR34");
        assert_eq!(format!("{}", Error::from(ERR35)), "ERR35");
        assert_eq!(format!("{}", Error::from(ERR36)), "ERR36");
        assert_eq!(format!("{}", Error::from(ERR37)), "ERR37");
        assert_eq!(format!("{}", Error::from(ERR38)), "ERR38");
        assert_eq!(format!("{}", Error::from(ERR39)), "ERR39");
        assert_eq!(format!("{}", Error::from(ERR40)), "ERR40");
        assert_eq!(format!("{}", Error::from(ERR41)), "ERR41");
        assert_eq!(format!("{}", Error::from(ERR42)), "ERR42");
        assert_eq!(format!("{}", Error::from(ERR43)), "ERR43");
        assert_eq!(format!("{}", Error::from(ERR44)), "ERR44");
        assert_eq!(format!("{}", Error::from(ERR45)), "ERR45");
        assert_eq!(format!("{}", Error::from(ERR46)), "ERR46");
        assert_eq!(format!("{}", Error::from(ERR47)), "ERR47");
        assert_eq!(format!("{}", Error::from(ERR48)), "ERR48");
        assert_eq!(format!("{}", Error::from(ERR49)), "ERR49");
        assert_eq!(format!("{}", Error::from(ERR50)), "ERR50");
        assert_eq!(format!("{}", Error::from(ERR51)), "ERR51");
        assert_eq!(format!("{}", Error::from(ERR52)), "ERR52");
        assert_eq!(format!("{}", Error::from(ERR53)), "ERR53");
        assert_eq!(format!("{}", Error::from(ERR54)), "ERR54");
        assert_eq!(format!("{}", Error::from(ERR55)), "ERR55");
        assert_eq!(format!("{}", Error::from(ERR56)), "ERR56");
        assert_eq!(format!("{}", Error::from(ERR57)), "ERR57");
        assert_eq!(format!("{}", Error::from(ERR58)), "ERR58");
        assert_eq!(format!("{}", Error::from(ERR59)), "ERR59");
        assert_eq!(format!("{}", Error::from(ERR60)), "ERR60");
        assert_eq!(format!("{}", Error::from(ERR61)), "ERR61");
        assert_eq!(format!("{}", Error::from(ERR62)), "ERR62");
        assert_eq!(format!("{}", Error::from(ERR63)), "ERR63");
        assert_eq!(format!("{}", Error::from(ERR64)), "ERR64");
        assert_eq!(format!("{}", Error::from(ERR65)), "ERR65");
        assert_eq!(format!("{}", Error::from(ERR66)), "ERR66");
        assert_eq!(format!("{}", Error::from(ERR67)), "ERR67");
        assert_eq!(format!("{}", Error::from(ERR68)), "ERR68");
        assert_eq!(format!("{}", Error::from(ERR69)), "ERR69");
        assert_eq!(format!("{}", Error::from(ERR70)), "ERR70");
        assert_eq!(format!("{}", Error::from(ERR71)), "ERR71");
        assert_eq!(format!("{}", Error::from(ERR72)), "ERR72");
        assert_eq!(format!("{}", Error::from(ERR73)), "ERR73");
        assert_eq!(format!("{}", Error::from(ERR74)), "ERR74");
        assert_eq!(format!("{}", Error::from(ERR75)), "ERR75");
        assert_eq!(format!("{}", Error::from(ERR76)), "ERR76");
        assert_eq!(format!("{}", Error::from(ERR77)), "ERR77");
        assert_eq!(format!("{}", Error::from(ERR78)), "ERR78");
        assert_eq!(format!("{}", Error::from(ERR79)), "ERR79");
        assert_eq!(format!("{}", Error::from(ERR80)), "ERR80");
        assert_eq!(format!("{}", Error::from(ERR81)), "ERR81");
        assert_eq!(format!("{}", Error::from(ERR82)), "ERR82");
        assert_eq!(format!("{}", Error::from(ERR83)), "ERR83");
        assert_eq!(format!("{}", Error::from(ERR84)), "ERR84");
        assert_eq!(format!("{}", Error::from(ERR85)), "ERR85");
        assert_eq!(format!("{}", Error::from(ERR86)), "ERR86");
        assert_eq!(format!("{}", Error::from(ERR87)), "ERR87");
        assert_eq!(format!("{}", Error::from(ERR88)), "ERR88");
        assert_eq!(format!("{}", Error::from(ERR89)), "ERR89");
        assert_eq!(format!("{}", Error::from(ERR90)), "ERR90");
        assert_eq!(format!("{}", Error::from(ERR91)), "ERR91");
        assert_eq!(format!("{}", Error::from(ERR92)), "ERR92");
        assert_eq!(format!("{}", Error::from(ERR93)), "ERR93");
        assert_eq!(format!("{}", Error::from(ERR94)), "ERR94");
        assert_eq!(format!("{}", Error::from(ERR95)), "ERR95");
        assert_eq!(format!("{}", Error::from(ERR96)), "ERR96");
        assert_eq!(format!("{}", Error::from(ERR97)), "ERR97");
        assert_eq!(format!("{}", Error::from(ERR98)), "ERR98");
        assert_eq!(format!("{}", Error::from(ERR99)), "ERR99");
        assert_eq!(format!("{}", Error::from(ERR100)), "ERR100");
        assert_eq!(format!("{}", Error::from(ERR101)), "ERR101");
        assert_eq!(format!("{}", Error::from(ERR102)), "ERR102");
        assert_eq!(format!("{}", Error::from(ERR103)), "ERR103");
        assert_eq!(format!("{}", Error::from(ERR104)), "ERR104");
        assert_eq!(format!("{}", Error::from(ERR105)), "ERR105");
        assert_eq!(format!("{}", Error::from(ERR106)), "ERR106");
        assert_eq!(format!("{}", Error::from(ERR107)), "ERR107");
        assert_eq!(format!("{}", Error::from(ERR108)), "ERR108");
        assert_eq!(format!("{}", Error::from(ERR109)), "ERR109");
        assert_eq!(format!("{}", Error::from(ERR110)), "ERR110");
        assert_eq!(format!("{}", Error::from(ERR111)), "ERR111");
        assert_eq!(format!("{}", Error::from(ERR112)), "ERR112");
        assert_eq!(format!("{}", Error::from(ERR113)), "ERR113");
        assert_eq!(format!("{}", Error::from(ERR114)), "ERR114");
        assert_eq!(format!("{}", Error::from(ERR115)), "ERR115");
        assert_eq!(format!("{}", Error::from(ERR116)), "ERR116");
        assert_eq!(format!("{}", Error::from(ERR117)), "ERR117");
        assert_eq!(format!("{}", Error::from(ERR118)), "ERR118");
        assert_eq!(format!("{}", Error::from(ERR119)), "ERR119");
        assert_eq!(format!("{}", Error::from(ERR120)), "ERR120");
        assert_eq!(format!("{}", Error::from(ERR121)), "ERR121");
        assert_eq!(format!("{}", Error::from(ERR122)), "ERR122");
        assert_eq!(format!("{}", Error::from(ERR123)), "ERR123");
        assert_eq!(format!("{}", Error::from(ERR124)), "ERR124");
        assert_eq!(format!("{}", Error::from(ERR125)), "ERR125");
        assert_eq!(format!("{}", Error::from(ERR126)), "ERR126");
    }

}

