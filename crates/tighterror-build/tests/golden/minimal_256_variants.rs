
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
        pub(crate) const ERR127: &str = "ERR127";
        pub(crate) const ERR128: &str = "ERR128";
        pub(crate) const ERR129: &str = "ERR129";
        pub(crate) const ERR130: &str = "ERR130";
        pub(crate) const ERR131: &str = "ERR131";
        pub(crate) const ERR132: &str = "ERR132";
        pub(crate) const ERR133: &str = "ERR133";
        pub(crate) const ERR134: &str = "ERR134";
        pub(crate) const ERR135: &str = "ERR135";
        pub(crate) const ERR136: &str = "ERR136";
        pub(crate) const ERR137: &str = "ERR137";
        pub(crate) const ERR138: &str = "ERR138";
        pub(crate) const ERR139: &str = "ERR139";
        pub(crate) const ERR140: &str = "ERR140";
        pub(crate) const ERR141: &str = "ERR141";
        pub(crate) const ERR142: &str = "ERR142";
        pub(crate) const ERR143: &str = "ERR143";
        pub(crate) const ERR144: &str = "ERR144";
        pub(crate) const ERR145: &str = "ERR145";
        pub(crate) const ERR146: &str = "ERR146";
        pub(crate) const ERR147: &str = "ERR147";
        pub(crate) const ERR148: &str = "ERR148";
        pub(crate) const ERR149: &str = "ERR149";
        pub(crate) const ERR150: &str = "ERR150";
        pub(crate) const ERR151: &str = "ERR151";
        pub(crate) const ERR152: &str = "ERR152";
        pub(crate) const ERR153: &str = "ERR153";
        pub(crate) const ERR154: &str = "ERR154";
        pub(crate) const ERR155: &str = "ERR155";
        pub(crate) const ERR156: &str = "ERR156";
        pub(crate) const ERR157: &str = "ERR157";
        pub(crate) const ERR158: &str = "ERR158";
        pub(crate) const ERR159: &str = "ERR159";
        pub(crate) const ERR160: &str = "ERR160";
        pub(crate) const ERR161: &str = "ERR161";
        pub(crate) const ERR162: &str = "ERR162";
        pub(crate) const ERR163: &str = "ERR163";
        pub(crate) const ERR164: &str = "ERR164";
        pub(crate) const ERR165: &str = "ERR165";
        pub(crate) const ERR166: &str = "ERR166";
        pub(crate) const ERR167: &str = "ERR167";
        pub(crate) const ERR168: &str = "ERR168";
        pub(crate) const ERR169: &str = "ERR169";
        pub(crate) const ERR170: &str = "ERR170";
        pub(crate) const ERR171: &str = "ERR171";
        pub(crate) const ERR172: &str = "ERR172";
        pub(crate) const ERR173: &str = "ERR173";
        pub(crate) const ERR174: &str = "ERR174";
        pub(crate) const ERR175: &str = "ERR175";
        pub(crate) const ERR176: &str = "ERR176";
        pub(crate) const ERR177: &str = "ERR177";
        pub(crate) const ERR178: &str = "ERR178";
        pub(crate) const ERR179: &str = "ERR179";
        pub(crate) const ERR180: &str = "ERR180";
        pub(crate) const ERR181: &str = "ERR181";
        pub(crate) const ERR182: &str = "ERR182";
        pub(crate) const ERR183: &str = "ERR183";
        pub(crate) const ERR184: &str = "ERR184";
        pub(crate) const ERR185: &str = "ERR185";
        pub(crate) const ERR186: &str = "ERR186";
        pub(crate) const ERR187: &str = "ERR187";
        pub(crate) const ERR188: &str = "ERR188";
        pub(crate) const ERR189: &str = "ERR189";
        pub(crate) const ERR190: &str = "ERR190";
        pub(crate) const ERR191: &str = "ERR191";
        pub(crate) const ERR192: &str = "ERR192";
        pub(crate) const ERR193: &str = "ERR193";
        pub(crate) const ERR194: &str = "ERR194";
        pub(crate) const ERR195: &str = "ERR195";
        pub(crate) const ERR196: &str = "ERR196";
        pub(crate) const ERR197: &str = "ERR197";
        pub(crate) const ERR198: &str = "ERR198";
        pub(crate) const ERR199: &str = "ERR199";
        pub(crate) const ERR200: &str = "ERR200";
        pub(crate) const ERR201: &str = "ERR201";
        pub(crate) const ERR202: &str = "ERR202";
        pub(crate) const ERR203: &str = "ERR203";
        pub(crate) const ERR204: &str = "ERR204";
        pub(crate) const ERR205: &str = "ERR205";
        pub(crate) const ERR206: &str = "ERR206";
        pub(crate) const ERR207: &str = "ERR207";
        pub(crate) const ERR208: &str = "ERR208";
        pub(crate) const ERR209: &str = "ERR209";
        pub(crate) const ERR210: &str = "ERR210";
        pub(crate) const ERR211: &str = "ERR211";
        pub(crate) const ERR212: &str = "ERR212";
        pub(crate) const ERR213: &str = "ERR213";
        pub(crate) const ERR214: &str = "ERR214";
        pub(crate) const ERR215: &str = "ERR215";
        pub(crate) const ERR216: &str = "ERR216";
        pub(crate) const ERR217: &str = "ERR217";
        pub(crate) const ERR218: &str = "ERR218";
        pub(crate) const ERR219: &str = "ERR219";
        pub(crate) const ERR220: &str = "ERR220";
        pub(crate) const ERR221: &str = "ERR221";
        pub(crate) const ERR222: &str = "ERR222";
        pub(crate) const ERR223: &str = "ERR223";
        pub(crate) const ERR224: &str = "ERR224";
        pub(crate) const ERR225: &str = "ERR225";
        pub(crate) const ERR226: &str = "ERR226";
        pub(crate) const ERR227: &str = "ERR227";
        pub(crate) const ERR228: &str = "ERR228";
        pub(crate) const ERR229: &str = "ERR229";
        pub(crate) const ERR230: &str = "ERR230";
        pub(crate) const ERR231: &str = "ERR231";
        pub(crate) const ERR232: &str = "ERR232";
        pub(crate) const ERR233: &str = "ERR233";
        pub(crate) const ERR234: &str = "ERR234";
        pub(crate) const ERR235: &str = "ERR235";
        pub(crate) const ERR236: &str = "ERR236";
        pub(crate) const ERR237: &str = "ERR237";
        pub(crate) const ERR238: &str = "ERR238";
        pub(crate) const ERR239: &str = "ERR239";
        pub(crate) const ERR240: &str = "ERR240";
        pub(crate) const ERR241: &str = "ERR241";
        pub(crate) const ERR242: &str = "ERR242";
        pub(crate) const ERR243: &str = "ERR243";
        pub(crate) const ERR244: &str = "ERR244";
        pub(crate) const ERR245: &str = "ERR245";
        pub(crate) const ERR246: &str = "ERR246";
        pub(crate) const ERR247: &str = "ERR247";
        pub(crate) const ERR248: &str = "ERR248";
        pub(crate) const ERR249: &str = "ERR249";
        pub(crate) const ERR250: &str = "ERR250";
        pub(crate) const ERR251: &str = "ERR251";
        pub(crate) const ERR252: &str = "ERR252";
        pub(crate) const ERR253: &str = "ERR253";
        pub(crate) const ERR254: &str = "ERR254";
        pub(crate) const ERR255: &str = "ERR255";
        pub static A: [&str; 256] = [
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
            ERR127,
            ERR128,
            ERR129,
            ERR130,
            ERR131,
            ERR132,
            ERR133,
            ERR134,
            ERR135,
            ERR136,
            ERR137,
            ERR138,
            ERR139,
            ERR140,
            ERR141,
            ERR142,
            ERR143,
            ERR144,
            ERR145,
            ERR146,
            ERR147,
            ERR148,
            ERR149,
            ERR150,
            ERR151,
            ERR152,
            ERR153,
            ERR154,
            ERR155,
            ERR156,
            ERR157,
            ERR158,
            ERR159,
            ERR160,
            ERR161,
            ERR162,
            ERR163,
            ERR164,
            ERR165,
            ERR166,
            ERR167,
            ERR168,
            ERR169,
            ERR170,
            ERR171,
            ERR172,
            ERR173,
            ERR174,
            ERR175,
            ERR176,
            ERR177,
            ERR178,
            ERR179,
            ERR180,
            ERR181,
            ERR182,
            ERR183,
            ERR184,
            ERR185,
            ERR186,
            ERR187,
            ERR188,
            ERR189,
            ERR190,
            ERR191,
            ERR192,
            ERR193,
            ERR194,
            ERR195,
            ERR196,
            ERR197,
            ERR198,
            ERR199,
            ERR200,
            ERR201,
            ERR202,
            ERR203,
            ERR204,
            ERR205,
            ERR206,
            ERR207,
            ERR208,
            ERR209,
            ERR210,
            ERR211,
            ERR212,
            ERR213,
            ERR214,
            ERR215,
            ERR216,
            ERR217,
            ERR218,
            ERR219,
            ERR220,
            ERR221,
            ERR222,
            ERR223,
            ERR224,
            ERR225,
            ERR226,
            ERR227,
            ERR228,
            ERR229,
            ERR230,
            ERR231,
            ERR232,
            ERR233,
            ERR234,
            ERR235,
            ERR236,
            ERR237,
            ERR238,
            ERR239,
            ERR240,
            ERR241,
            ERR242,
            ERR243,
            ERR244,
            ERR245,
            ERR246,
            ERR247,
            ERR248,
            ERR249,
            ERR250,
            ERR251,
            ERR252,
            ERR253,
            ERR254,
            ERR255,
        ];

    }

    pub static A: [&[&str]; 1] = [&general::A];
}

mod _d {
    pub(crate) mod general {
        use super::super::_n;
        const ERR0: &str = _n::general::ERR0;
        const ERR1: &str = _n::general::ERR1;
        const ERR2: &str = _n::general::ERR2;
        const ERR3: &str = _n::general::ERR3;
        const ERR4: &str = _n::general::ERR4;
        const ERR5: &str = _n::general::ERR5;
        const ERR6: &str = _n::general::ERR6;
        const ERR7: &str = _n::general::ERR7;
        const ERR8: &str = _n::general::ERR8;
        const ERR9: &str = _n::general::ERR9;
        const ERR10: &str = _n::general::ERR10;
        const ERR11: &str = _n::general::ERR11;
        const ERR12: &str = _n::general::ERR12;
        const ERR13: &str = _n::general::ERR13;
        const ERR14: &str = _n::general::ERR14;
        const ERR15: &str = _n::general::ERR15;
        const ERR16: &str = _n::general::ERR16;
        const ERR17: &str = _n::general::ERR17;
        const ERR18: &str = _n::general::ERR18;
        const ERR19: &str = _n::general::ERR19;
        const ERR20: &str = _n::general::ERR20;
        const ERR21: &str = _n::general::ERR21;
        const ERR22: &str = _n::general::ERR22;
        const ERR23: &str = _n::general::ERR23;
        const ERR24: &str = _n::general::ERR24;
        const ERR25: &str = _n::general::ERR25;
        const ERR26: &str = _n::general::ERR26;
        const ERR27: &str = _n::general::ERR27;
        const ERR28: &str = _n::general::ERR28;
        const ERR29: &str = _n::general::ERR29;
        const ERR30: &str = _n::general::ERR30;
        const ERR31: &str = _n::general::ERR31;
        const ERR32: &str = _n::general::ERR32;
        const ERR33: &str = _n::general::ERR33;
        const ERR34: &str = _n::general::ERR34;
        const ERR35: &str = _n::general::ERR35;
        const ERR36: &str = _n::general::ERR36;
        const ERR37: &str = _n::general::ERR37;
        const ERR38: &str = _n::general::ERR38;
        const ERR39: &str = _n::general::ERR39;
        const ERR40: &str = _n::general::ERR40;
        const ERR41: &str = _n::general::ERR41;
        const ERR42: &str = _n::general::ERR42;
        const ERR43: &str = _n::general::ERR43;
        const ERR44: &str = _n::general::ERR44;
        const ERR45: &str = _n::general::ERR45;
        const ERR46: &str = _n::general::ERR46;
        const ERR47: &str = _n::general::ERR47;
        const ERR48: &str = _n::general::ERR48;
        const ERR49: &str = _n::general::ERR49;
        const ERR50: &str = _n::general::ERR50;
        const ERR51: &str = _n::general::ERR51;
        const ERR52: &str = _n::general::ERR52;
        const ERR53: &str = _n::general::ERR53;
        const ERR54: &str = _n::general::ERR54;
        const ERR55: &str = _n::general::ERR55;
        const ERR56: &str = _n::general::ERR56;
        const ERR57: &str = _n::general::ERR57;
        const ERR58: &str = _n::general::ERR58;
        const ERR59: &str = _n::general::ERR59;
        const ERR60: &str = _n::general::ERR60;
        const ERR61: &str = _n::general::ERR61;
        const ERR62: &str = _n::general::ERR62;
        const ERR63: &str = _n::general::ERR63;
        const ERR64: &str = _n::general::ERR64;
        const ERR65: &str = _n::general::ERR65;
        const ERR66: &str = _n::general::ERR66;
        const ERR67: &str = _n::general::ERR67;
        const ERR68: &str = _n::general::ERR68;
        const ERR69: &str = _n::general::ERR69;
        const ERR70: &str = _n::general::ERR70;
        const ERR71: &str = _n::general::ERR71;
        const ERR72: &str = _n::general::ERR72;
        const ERR73: &str = _n::general::ERR73;
        const ERR74: &str = _n::general::ERR74;
        const ERR75: &str = _n::general::ERR75;
        const ERR76: &str = _n::general::ERR76;
        const ERR77: &str = _n::general::ERR77;
        const ERR78: &str = _n::general::ERR78;
        const ERR79: &str = _n::general::ERR79;
        const ERR80: &str = _n::general::ERR80;
        const ERR81: &str = _n::general::ERR81;
        const ERR82: &str = _n::general::ERR82;
        const ERR83: &str = _n::general::ERR83;
        const ERR84: &str = _n::general::ERR84;
        const ERR85: &str = _n::general::ERR85;
        const ERR86: &str = _n::general::ERR86;
        const ERR87: &str = _n::general::ERR87;
        const ERR88: &str = _n::general::ERR88;
        const ERR89: &str = _n::general::ERR89;
        const ERR90: &str = _n::general::ERR90;
        const ERR91: &str = _n::general::ERR91;
        const ERR92: &str = _n::general::ERR92;
        const ERR93: &str = _n::general::ERR93;
        const ERR94: &str = _n::general::ERR94;
        const ERR95: &str = _n::general::ERR95;
        const ERR96: &str = _n::general::ERR96;
        const ERR97: &str = _n::general::ERR97;
        const ERR98: &str = _n::general::ERR98;
        const ERR99: &str = _n::general::ERR99;
        const ERR100: &str = _n::general::ERR100;
        const ERR101: &str = _n::general::ERR101;
        const ERR102: &str = _n::general::ERR102;
        const ERR103: &str = _n::general::ERR103;
        const ERR104: &str = _n::general::ERR104;
        const ERR105: &str = _n::general::ERR105;
        const ERR106: &str = _n::general::ERR106;
        const ERR107: &str = _n::general::ERR107;
        const ERR108: &str = _n::general::ERR108;
        const ERR109: &str = _n::general::ERR109;
        const ERR110: &str = _n::general::ERR110;
        const ERR111: &str = _n::general::ERR111;
        const ERR112: &str = _n::general::ERR112;
        const ERR113: &str = _n::general::ERR113;
        const ERR114: &str = _n::general::ERR114;
        const ERR115: &str = _n::general::ERR115;
        const ERR116: &str = _n::general::ERR116;
        const ERR117: &str = _n::general::ERR117;
        const ERR118: &str = _n::general::ERR118;
        const ERR119: &str = _n::general::ERR119;
        const ERR120: &str = _n::general::ERR120;
        const ERR121: &str = _n::general::ERR121;
        const ERR122: &str = _n::general::ERR122;
        const ERR123: &str = _n::general::ERR123;
        const ERR124: &str = _n::general::ERR124;
        const ERR125: &str = _n::general::ERR125;
        const ERR126: &str = _n::general::ERR126;
        const ERR127: &str = _n::general::ERR127;
        const ERR128: &str = _n::general::ERR128;
        const ERR129: &str = _n::general::ERR129;
        const ERR130: &str = _n::general::ERR130;
        const ERR131: &str = _n::general::ERR131;
        const ERR132: &str = _n::general::ERR132;
        const ERR133: &str = _n::general::ERR133;
        const ERR134: &str = _n::general::ERR134;
        const ERR135: &str = _n::general::ERR135;
        const ERR136: &str = _n::general::ERR136;
        const ERR137: &str = _n::general::ERR137;
        const ERR138: &str = _n::general::ERR138;
        const ERR139: &str = _n::general::ERR139;
        const ERR140: &str = _n::general::ERR140;
        const ERR141: &str = _n::general::ERR141;
        const ERR142: &str = _n::general::ERR142;
        const ERR143: &str = _n::general::ERR143;
        const ERR144: &str = _n::general::ERR144;
        const ERR145: &str = _n::general::ERR145;
        const ERR146: &str = _n::general::ERR146;
        const ERR147: &str = _n::general::ERR147;
        const ERR148: &str = _n::general::ERR148;
        const ERR149: &str = _n::general::ERR149;
        const ERR150: &str = _n::general::ERR150;
        const ERR151: &str = _n::general::ERR151;
        const ERR152: &str = _n::general::ERR152;
        const ERR153: &str = _n::general::ERR153;
        const ERR154: &str = _n::general::ERR154;
        const ERR155: &str = _n::general::ERR155;
        const ERR156: &str = _n::general::ERR156;
        const ERR157: &str = _n::general::ERR157;
        const ERR158: &str = _n::general::ERR158;
        const ERR159: &str = _n::general::ERR159;
        const ERR160: &str = _n::general::ERR160;
        const ERR161: &str = _n::general::ERR161;
        const ERR162: &str = _n::general::ERR162;
        const ERR163: &str = _n::general::ERR163;
        const ERR164: &str = _n::general::ERR164;
        const ERR165: &str = _n::general::ERR165;
        const ERR166: &str = _n::general::ERR166;
        const ERR167: &str = _n::general::ERR167;
        const ERR168: &str = _n::general::ERR168;
        const ERR169: &str = _n::general::ERR169;
        const ERR170: &str = _n::general::ERR170;
        const ERR171: &str = _n::general::ERR171;
        const ERR172: &str = _n::general::ERR172;
        const ERR173: &str = _n::general::ERR173;
        const ERR174: &str = _n::general::ERR174;
        const ERR175: &str = _n::general::ERR175;
        const ERR176: &str = _n::general::ERR176;
        const ERR177: &str = _n::general::ERR177;
        const ERR178: &str = _n::general::ERR178;
        const ERR179: &str = _n::general::ERR179;
        const ERR180: &str = _n::general::ERR180;
        const ERR181: &str = _n::general::ERR181;
        const ERR182: &str = _n::general::ERR182;
        const ERR183: &str = _n::general::ERR183;
        const ERR184: &str = _n::general::ERR184;
        const ERR185: &str = _n::general::ERR185;
        const ERR186: &str = _n::general::ERR186;
        const ERR187: &str = _n::general::ERR187;
        const ERR188: &str = _n::general::ERR188;
        const ERR189: &str = _n::general::ERR189;
        const ERR190: &str = _n::general::ERR190;
        const ERR191: &str = _n::general::ERR191;
        const ERR192: &str = _n::general::ERR192;
        const ERR193: &str = _n::general::ERR193;
        const ERR194: &str = _n::general::ERR194;
        const ERR195: &str = _n::general::ERR195;
        const ERR196: &str = _n::general::ERR196;
        const ERR197: &str = _n::general::ERR197;
        const ERR198: &str = _n::general::ERR198;
        const ERR199: &str = _n::general::ERR199;
        const ERR200: &str = _n::general::ERR200;
        const ERR201: &str = _n::general::ERR201;
        const ERR202: &str = _n::general::ERR202;
        const ERR203: &str = _n::general::ERR203;
        const ERR204: &str = _n::general::ERR204;
        const ERR205: &str = _n::general::ERR205;
        const ERR206: &str = _n::general::ERR206;
        const ERR207: &str = _n::general::ERR207;
        const ERR208: &str = _n::general::ERR208;
        const ERR209: &str = _n::general::ERR209;
        const ERR210: &str = _n::general::ERR210;
        const ERR211: &str = _n::general::ERR211;
        const ERR212: &str = _n::general::ERR212;
        const ERR213: &str = _n::general::ERR213;
        const ERR214: &str = _n::general::ERR214;
        const ERR215: &str = _n::general::ERR215;
        const ERR216: &str = _n::general::ERR216;
        const ERR217: &str = _n::general::ERR217;
        const ERR218: &str = _n::general::ERR218;
        const ERR219: &str = _n::general::ERR219;
        const ERR220: &str = _n::general::ERR220;
        const ERR221: &str = _n::general::ERR221;
        const ERR222: &str = _n::general::ERR222;
        const ERR223: &str = _n::general::ERR223;
        const ERR224: &str = _n::general::ERR224;
        const ERR225: &str = _n::general::ERR225;
        const ERR226: &str = _n::general::ERR226;
        const ERR227: &str = _n::general::ERR227;
        const ERR228: &str = _n::general::ERR228;
        const ERR229: &str = _n::general::ERR229;
        const ERR230: &str = _n::general::ERR230;
        const ERR231: &str = _n::general::ERR231;
        const ERR232: &str = _n::general::ERR232;
        const ERR233: &str = _n::general::ERR233;
        const ERR234: &str = _n::general::ERR234;
        const ERR235: &str = _n::general::ERR235;
        const ERR236: &str = _n::general::ERR236;
        const ERR237: &str = _n::general::ERR237;
        const ERR238: &str = _n::general::ERR238;
        const ERR239: &str = _n::general::ERR239;
        const ERR240: &str = _n::general::ERR240;
        const ERR241: &str = _n::general::ERR241;
        const ERR242: &str = _n::general::ERR242;
        const ERR243: &str = _n::general::ERR243;
        const ERR244: &str = _n::general::ERR244;
        const ERR245: &str = _n::general::ERR245;
        const ERR246: &str = _n::general::ERR246;
        const ERR247: &str = _n::general::ERR247;
        const ERR248: &str = _n::general::ERR248;
        const ERR249: &str = _n::general::ERR249;
        const ERR250: &str = _n::general::ERR250;
        const ERR251: &str = _n::general::ERR251;
        const ERR252: &str = _n::general::ERR252;
        const ERR253: &str = _n::general::ERR253;
        const ERR254: &str = _n::general::ERR254;
        const ERR255: &str = _n::general::ERR255;
        pub static A: [&str; 256] = [
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
            ERR127,
            ERR128,
            ERR129,
            ERR130,
            ERR131,
            ERR132,
            ERR133,
            ERR134,
            ERR135,
            ERR136,
            ERR137,
            ERR138,
            ERR139,
            ERR140,
            ERR141,
            ERR142,
            ERR143,
            ERR144,
            ERR145,
            ERR146,
            ERR147,
            ERR148,
            ERR149,
            ERR150,
            ERR151,
            ERR152,
            ERR153,
            ERR154,
            ERR155,
            ERR156,
            ERR157,
            ERR158,
            ERR159,
            ERR160,
            ERR161,
            ERR162,
            ERR163,
            ERR164,
            ERR165,
            ERR166,
            ERR167,
            ERR168,
            ERR169,
            ERR170,
            ERR171,
            ERR172,
            ERR173,
            ERR174,
            ERR175,
            ERR176,
            ERR177,
            ERR178,
            ERR179,
            ERR180,
            ERR181,
            ERR182,
            ERR183,
            ERR184,
            ERR185,
            ERR186,
            ERR187,
            ERR188,
            ERR189,
            ERR190,
            ERR191,
            ERR192,
            ERR193,
            ERR194,
            ERR195,
            ERR196,
            ERR197,
            ERR198,
            ERR199,
            ERR200,
            ERR201,
            ERR202,
            ERR203,
            ERR204,
            ERR205,
            ERR206,
            ERR207,
            ERR208,
            ERR209,
            ERR210,
            ERR211,
            ERR212,
            ERR213,
            ERR214,
            ERR215,
            ERR216,
            ERR217,
            ERR218,
            ERR219,
            ERR220,
            ERR221,
            ERR222,
            ERR223,
            ERR224,
            ERR225,
            ERR226,
            ERR227,
            ERR228,
            ERR229,
            ERR230,
            ERR231,
            ERR232,
            ERR233,
            ERR234,
            ERR235,
            ERR236,
            ERR237,
            ERR238,
            ERR239,
            ERR240,
            ERR241,
            ERR242,
            ERR243,
            ERR244,
            ERR245,
            ERR246,
            ERR247,
            ERR248,
            ERR249,
            ERR250,
            ERR251,
            ERR252,
            ERR253,
            ERR254,
            ERR255,
        ];

    }

    pub static A: [&[&str]; 1] = [&general::A];
}

mod _p {
    pub const CAT_BITS: usize = 0;
    pub const VAR_BITS: usize = 8;
    pub const KIND_BITS: usize = CAT_BITS + VAR_BITS;
    pub type R = u8;
    pub const CAT_MAX: R = 0;
    pub const VAR_MASK: R = R::MAX >> (R::BITS as usize - VAR_BITS);
    pub static VAR_MAXES: [R; 1] = [255];
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
    pub const ERR0: EK = EK::new(c::GENERAL, 0);
    pub const ERR1: EK = EK::new(c::GENERAL, 1);
    pub const ERR2: EK = EK::new(c::GENERAL, 2);
    pub const ERR3: EK = EK::new(c::GENERAL, 3);
    pub const ERR4: EK = EK::new(c::GENERAL, 4);
    pub const ERR5: EK = EK::new(c::GENERAL, 5);
    pub const ERR6: EK = EK::new(c::GENERAL, 6);
    pub const ERR7: EK = EK::new(c::GENERAL, 7);
    pub const ERR8: EK = EK::new(c::GENERAL, 8);
    pub const ERR9: EK = EK::new(c::GENERAL, 9);
    pub const ERR10: EK = EK::new(c::GENERAL, 10);
    pub const ERR11: EK = EK::new(c::GENERAL, 11);
    pub const ERR12: EK = EK::new(c::GENERAL, 12);
    pub const ERR13: EK = EK::new(c::GENERAL, 13);
    pub const ERR14: EK = EK::new(c::GENERAL, 14);
    pub const ERR15: EK = EK::new(c::GENERAL, 15);
    pub const ERR16: EK = EK::new(c::GENERAL, 16);
    pub const ERR17: EK = EK::new(c::GENERAL, 17);
    pub const ERR18: EK = EK::new(c::GENERAL, 18);
    pub const ERR19: EK = EK::new(c::GENERAL, 19);
    pub const ERR20: EK = EK::new(c::GENERAL, 20);
    pub const ERR21: EK = EK::new(c::GENERAL, 21);
    pub const ERR22: EK = EK::new(c::GENERAL, 22);
    pub const ERR23: EK = EK::new(c::GENERAL, 23);
    pub const ERR24: EK = EK::new(c::GENERAL, 24);
    pub const ERR25: EK = EK::new(c::GENERAL, 25);
    pub const ERR26: EK = EK::new(c::GENERAL, 26);
    pub const ERR27: EK = EK::new(c::GENERAL, 27);
    pub const ERR28: EK = EK::new(c::GENERAL, 28);
    pub const ERR29: EK = EK::new(c::GENERAL, 29);
    pub const ERR30: EK = EK::new(c::GENERAL, 30);
    pub const ERR31: EK = EK::new(c::GENERAL, 31);
    pub const ERR32: EK = EK::new(c::GENERAL, 32);
    pub const ERR33: EK = EK::new(c::GENERAL, 33);
    pub const ERR34: EK = EK::new(c::GENERAL, 34);
    pub const ERR35: EK = EK::new(c::GENERAL, 35);
    pub const ERR36: EK = EK::new(c::GENERAL, 36);
    pub const ERR37: EK = EK::new(c::GENERAL, 37);
    pub const ERR38: EK = EK::new(c::GENERAL, 38);
    pub const ERR39: EK = EK::new(c::GENERAL, 39);
    pub const ERR40: EK = EK::new(c::GENERAL, 40);
    pub const ERR41: EK = EK::new(c::GENERAL, 41);
    pub const ERR42: EK = EK::new(c::GENERAL, 42);
    pub const ERR43: EK = EK::new(c::GENERAL, 43);
    pub const ERR44: EK = EK::new(c::GENERAL, 44);
    pub const ERR45: EK = EK::new(c::GENERAL, 45);
    pub const ERR46: EK = EK::new(c::GENERAL, 46);
    pub const ERR47: EK = EK::new(c::GENERAL, 47);
    pub const ERR48: EK = EK::new(c::GENERAL, 48);
    pub const ERR49: EK = EK::new(c::GENERAL, 49);
    pub const ERR50: EK = EK::new(c::GENERAL, 50);
    pub const ERR51: EK = EK::new(c::GENERAL, 51);
    pub const ERR52: EK = EK::new(c::GENERAL, 52);
    pub const ERR53: EK = EK::new(c::GENERAL, 53);
    pub const ERR54: EK = EK::new(c::GENERAL, 54);
    pub const ERR55: EK = EK::new(c::GENERAL, 55);
    pub const ERR56: EK = EK::new(c::GENERAL, 56);
    pub const ERR57: EK = EK::new(c::GENERAL, 57);
    pub const ERR58: EK = EK::new(c::GENERAL, 58);
    pub const ERR59: EK = EK::new(c::GENERAL, 59);
    pub const ERR60: EK = EK::new(c::GENERAL, 60);
    pub const ERR61: EK = EK::new(c::GENERAL, 61);
    pub const ERR62: EK = EK::new(c::GENERAL, 62);
    pub const ERR63: EK = EK::new(c::GENERAL, 63);
    pub const ERR64: EK = EK::new(c::GENERAL, 64);
    pub const ERR65: EK = EK::new(c::GENERAL, 65);
    pub const ERR66: EK = EK::new(c::GENERAL, 66);
    pub const ERR67: EK = EK::new(c::GENERAL, 67);
    pub const ERR68: EK = EK::new(c::GENERAL, 68);
    pub const ERR69: EK = EK::new(c::GENERAL, 69);
    pub const ERR70: EK = EK::new(c::GENERAL, 70);
    pub const ERR71: EK = EK::new(c::GENERAL, 71);
    pub const ERR72: EK = EK::new(c::GENERAL, 72);
    pub const ERR73: EK = EK::new(c::GENERAL, 73);
    pub const ERR74: EK = EK::new(c::GENERAL, 74);
    pub const ERR75: EK = EK::new(c::GENERAL, 75);
    pub const ERR76: EK = EK::new(c::GENERAL, 76);
    pub const ERR77: EK = EK::new(c::GENERAL, 77);
    pub const ERR78: EK = EK::new(c::GENERAL, 78);
    pub const ERR79: EK = EK::new(c::GENERAL, 79);
    pub const ERR80: EK = EK::new(c::GENERAL, 80);
    pub const ERR81: EK = EK::new(c::GENERAL, 81);
    pub const ERR82: EK = EK::new(c::GENERAL, 82);
    pub const ERR83: EK = EK::new(c::GENERAL, 83);
    pub const ERR84: EK = EK::new(c::GENERAL, 84);
    pub const ERR85: EK = EK::new(c::GENERAL, 85);
    pub const ERR86: EK = EK::new(c::GENERAL, 86);
    pub const ERR87: EK = EK::new(c::GENERAL, 87);
    pub const ERR88: EK = EK::new(c::GENERAL, 88);
    pub const ERR89: EK = EK::new(c::GENERAL, 89);
    pub const ERR90: EK = EK::new(c::GENERAL, 90);
    pub const ERR91: EK = EK::new(c::GENERAL, 91);
    pub const ERR92: EK = EK::new(c::GENERAL, 92);
    pub const ERR93: EK = EK::new(c::GENERAL, 93);
    pub const ERR94: EK = EK::new(c::GENERAL, 94);
    pub const ERR95: EK = EK::new(c::GENERAL, 95);
    pub const ERR96: EK = EK::new(c::GENERAL, 96);
    pub const ERR97: EK = EK::new(c::GENERAL, 97);
    pub const ERR98: EK = EK::new(c::GENERAL, 98);
    pub const ERR99: EK = EK::new(c::GENERAL, 99);
    pub const ERR100: EK = EK::new(c::GENERAL, 100);
    pub const ERR101: EK = EK::new(c::GENERAL, 101);
    pub const ERR102: EK = EK::new(c::GENERAL, 102);
    pub const ERR103: EK = EK::new(c::GENERAL, 103);
    pub const ERR104: EK = EK::new(c::GENERAL, 104);
    pub const ERR105: EK = EK::new(c::GENERAL, 105);
    pub const ERR106: EK = EK::new(c::GENERAL, 106);
    pub const ERR107: EK = EK::new(c::GENERAL, 107);
    pub const ERR108: EK = EK::new(c::GENERAL, 108);
    pub const ERR109: EK = EK::new(c::GENERAL, 109);
    pub const ERR110: EK = EK::new(c::GENERAL, 110);
    pub const ERR111: EK = EK::new(c::GENERAL, 111);
    pub const ERR112: EK = EK::new(c::GENERAL, 112);
    pub const ERR113: EK = EK::new(c::GENERAL, 113);
    pub const ERR114: EK = EK::new(c::GENERAL, 114);
    pub const ERR115: EK = EK::new(c::GENERAL, 115);
    pub const ERR116: EK = EK::new(c::GENERAL, 116);
    pub const ERR117: EK = EK::new(c::GENERAL, 117);
    pub const ERR118: EK = EK::new(c::GENERAL, 118);
    pub const ERR119: EK = EK::new(c::GENERAL, 119);
    pub const ERR120: EK = EK::new(c::GENERAL, 120);
    pub const ERR121: EK = EK::new(c::GENERAL, 121);
    pub const ERR122: EK = EK::new(c::GENERAL, 122);
    pub const ERR123: EK = EK::new(c::GENERAL, 123);
    pub const ERR124: EK = EK::new(c::GENERAL, 124);
    pub const ERR125: EK = EK::new(c::GENERAL, 125);
    pub const ERR126: EK = EK::new(c::GENERAL, 126);
    pub const ERR127: EK = EK::new(c::GENERAL, 127);
    pub const ERR128: EK = EK::new(c::GENERAL, 128);
    pub const ERR129: EK = EK::new(c::GENERAL, 129);
    pub const ERR130: EK = EK::new(c::GENERAL, 130);
    pub const ERR131: EK = EK::new(c::GENERAL, 131);
    pub const ERR132: EK = EK::new(c::GENERAL, 132);
    pub const ERR133: EK = EK::new(c::GENERAL, 133);
    pub const ERR134: EK = EK::new(c::GENERAL, 134);
    pub const ERR135: EK = EK::new(c::GENERAL, 135);
    pub const ERR136: EK = EK::new(c::GENERAL, 136);
    pub const ERR137: EK = EK::new(c::GENERAL, 137);
    pub const ERR138: EK = EK::new(c::GENERAL, 138);
    pub const ERR139: EK = EK::new(c::GENERAL, 139);
    pub const ERR140: EK = EK::new(c::GENERAL, 140);
    pub const ERR141: EK = EK::new(c::GENERAL, 141);
    pub const ERR142: EK = EK::new(c::GENERAL, 142);
    pub const ERR143: EK = EK::new(c::GENERAL, 143);
    pub const ERR144: EK = EK::new(c::GENERAL, 144);
    pub const ERR145: EK = EK::new(c::GENERAL, 145);
    pub const ERR146: EK = EK::new(c::GENERAL, 146);
    pub const ERR147: EK = EK::new(c::GENERAL, 147);
    pub const ERR148: EK = EK::new(c::GENERAL, 148);
    pub const ERR149: EK = EK::new(c::GENERAL, 149);
    pub const ERR150: EK = EK::new(c::GENERAL, 150);
    pub const ERR151: EK = EK::new(c::GENERAL, 151);
    pub const ERR152: EK = EK::new(c::GENERAL, 152);
    pub const ERR153: EK = EK::new(c::GENERAL, 153);
    pub const ERR154: EK = EK::new(c::GENERAL, 154);
    pub const ERR155: EK = EK::new(c::GENERAL, 155);
    pub const ERR156: EK = EK::new(c::GENERAL, 156);
    pub const ERR157: EK = EK::new(c::GENERAL, 157);
    pub const ERR158: EK = EK::new(c::GENERAL, 158);
    pub const ERR159: EK = EK::new(c::GENERAL, 159);
    pub const ERR160: EK = EK::new(c::GENERAL, 160);
    pub const ERR161: EK = EK::new(c::GENERAL, 161);
    pub const ERR162: EK = EK::new(c::GENERAL, 162);
    pub const ERR163: EK = EK::new(c::GENERAL, 163);
    pub const ERR164: EK = EK::new(c::GENERAL, 164);
    pub const ERR165: EK = EK::new(c::GENERAL, 165);
    pub const ERR166: EK = EK::new(c::GENERAL, 166);
    pub const ERR167: EK = EK::new(c::GENERAL, 167);
    pub const ERR168: EK = EK::new(c::GENERAL, 168);
    pub const ERR169: EK = EK::new(c::GENERAL, 169);
    pub const ERR170: EK = EK::new(c::GENERAL, 170);
    pub const ERR171: EK = EK::new(c::GENERAL, 171);
    pub const ERR172: EK = EK::new(c::GENERAL, 172);
    pub const ERR173: EK = EK::new(c::GENERAL, 173);
    pub const ERR174: EK = EK::new(c::GENERAL, 174);
    pub const ERR175: EK = EK::new(c::GENERAL, 175);
    pub const ERR176: EK = EK::new(c::GENERAL, 176);
    pub const ERR177: EK = EK::new(c::GENERAL, 177);
    pub const ERR178: EK = EK::new(c::GENERAL, 178);
    pub const ERR179: EK = EK::new(c::GENERAL, 179);
    pub const ERR180: EK = EK::new(c::GENERAL, 180);
    pub const ERR181: EK = EK::new(c::GENERAL, 181);
    pub const ERR182: EK = EK::new(c::GENERAL, 182);
    pub const ERR183: EK = EK::new(c::GENERAL, 183);
    pub const ERR184: EK = EK::new(c::GENERAL, 184);
    pub const ERR185: EK = EK::new(c::GENERAL, 185);
    pub const ERR186: EK = EK::new(c::GENERAL, 186);
    pub const ERR187: EK = EK::new(c::GENERAL, 187);
    pub const ERR188: EK = EK::new(c::GENERAL, 188);
    pub const ERR189: EK = EK::new(c::GENERAL, 189);
    pub const ERR190: EK = EK::new(c::GENERAL, 190);
    pub const ERR191: EK = EK::new(c::GENERAL, 191);
    pub const ERR192: EK = EK::new(c::GENERAL, 192);
    pub const ERR193: EK = EK::new(c::GENERAL, 193);
    pub const ERR194: EK = EK::new(c::GENERAL, 194);
    pub const ERR195: EK = EK::new(c::GENERAL, 195);
    pub const ERR196: EK = EK::new(c::GENERAL, 196);
    pub const ERR197: EK = EK::new(c::GENERAL, 197);
    pub const ERR198: EK = EK::new(c::GENERAL, 198);
    pub const ERR199: EK = EK::new(c::GENERAL, 199);
    pub const ERR200: EK = EK::new(c::GENERAL, 200);
    pub const ERR201: EK = EK::new(c::GENERAL, 201);
    pub const ERR202: EK = EK::new(c::GENERAL, 202);
    pub const ERR203: EK = EK::new(c::GENERAL, 203);
    pub const ERR204: EK = EK::new(c::GENERAL, 204);
    pub const ERR205: EK = EK::new(c::GENERAL, 205);
    pub const ERR206: EK = EK::new(c::GENERAL, 206);
    pub const ERR207: EK = EK::new(c::GENERAL, 207);
    pub const ERR208: EK = EK::new(c::GENERAL, 208);
    pub const ERR209: EK = EK::new(c::GENERAL, 209);
    pub const ERR210: EK = EK::new(c::GENERAL, 210);
    pub const ERR211: EK = EK::new(c::GENERAL, 211);
    pub const ERR212: EK = EK::new(c::GENERAL, 212);
    pub const ERR213: EK = EK::new(c::GENERAL, 213);
    pub const ERR214: EK = EK::new(c::GENERAL, 214);
    pub const ERR215: EK = EK::new(c::GENERAL, 215);
    pub const ERR216: EK = EK::new(c::GENERAL, 216);
    pub const ERR217: EK = EK::new(c::GENERAL, 217);
    pub const ERR218: EK = EK::new(c::GENERAL, 218);
    pub const ERR219: EK = EK::new(c::GENERAL, 219);
    pub const ERR220: EK = EK::new(c::GENERAL, 220);
    pub const ERR221: EK = EK::new(c::GENERAL, 221);
    pub const ERR222: EK = EK::new(c::GENERAL, 222);
    pub const ERR223: EK = EK::new(c::GENERAL, 223);
    pub const ERR224: EK = EK::new(c::GENERAL, 224);
    pub const ERR225: EK = EK::new(c::GENERAL, 225);
    pub const ERR226: EK = EK::new(c::GENERAL, 226);
    pub const ERR227: EK = EK::new(c::GENERAL, 227);
    pub const ERR228: EK = EK::new(c::GENERAL, 228);
    pub const ERR229: EK = EK::new(c::GENERAL, 229);
    pub const ERR230: EK = EK::new(c::GENERAL, 230);
    pub const ERR231: EK = EK::new(c::GENERAL, 231);
    pub const ERR232: EK = EK::new(c::GENERAL, 232);
    pub const ERR233: EK = EK::new(c::GENERAL, 233);
    pub const ERR234: EK = EK::new(c::GENERAL, 234);
    pub const ERR235: EK = EK::new(c::GENERAL, 235);
    pub const ERR236: EK = EK::new(c::GENERAL, 236);
    pub const ERR237: EK = EK::new(c::GENERAL, 237);
    pub const ERR238: EK = EK::new(c::GENERAL, 238);
    pub const ERR239: EK = EK::new(c::GENERAL, 239);
    pub const ERR240: EK = EK::new(c::GENERAL, 240);
    pub const ERR241: EK = EK::new(c::GENERAL, 241);
    pub const ERR242: EK = EK::new(c::GENERAL, 242);
    pub const ERR243: EK = EK::new(c::GENERAL, 243);
    pub const ERR244: EK = EK::new(c::GENERAL, 244);
    pub const ERR245: EK = EK::new(c::GENERAL, 245);
    pub const ERR246: EK = EK::new(c::GENERAL, 246);
    pub const ERR247: EK = EK::new(c::GENERAL, 247);
    pub const ERR248: EK = EK::new(c::GENERAL, 248);
    pub const ERR249: EK = EK::new(c::GENERAL, 249);
    pub const ERR250: EK = EK::new(c::GENERAL, 250);
    pub const ERR251: EK = EK::new(c::GENERAL, 251);
    pub const ERR252: EK = EK::new(c::GENERAL, 252);
    pub const ERR253: EK = EK::new(c::GENERAL, 253);
    pub const ERR254: EK = EK::new(c::GENERAL, 254);
    pub const ERR255: EK = EK::new(c::GENERAL, 255);
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
        assert_eq!(ERR127.name(), "ERR127");
        assert_eq!(tighterror::Kind::name(& ERR127), "ERR127");
        assert_eq!(ERR128.name(), "ERR128");
        assert_eq!(tighterror::Kind::name(& ERR128), "ERR128");
        assert_eq!(ERR129.name(), "ERR129");
        assert_eq!(tighterror::Kind::name(& ERR129), "ERR129");
        assert_eq!(ERR130.name(), "ERR130");
        assert_eq!(tighterror::Kind::name(& ERR130), "ERR130");
        assert_eq!(ERR131.name(), "ERR131");
        assert_eq!(tighterror::Kind::name(& ERR131), "ERR131");
        assert_eq!(ERR132.name(), "ERR132");
        assert_eq!(tighterror::Kind::name(& ERR132), "ERR132");
        assert_eq!(ERR133.name(), "ERR133");
        assert_eq!(tighterror::Kind::name(& ERR133), "ERR133");
        assert_eq!(ERR134.name(), "ERR134");
        assert_eq!(tighterror::Kind::name(& ERR134), "ERR134");
        assert_eq!(ERR135.name(), "ERR135");
        assert_eq!(tighterror::Kind::name(& ERR135), "ERR135");
        assert_eq!(ERR136.name(), "ERR136");
        assert_eq!(tighterror::Kind::name(& ERR136), "ERR136");
        assert_eq!(ERR137.name(), "ERR137");
        assert_eq!(tighterror::Kind::name(& ERR137), "ERR137");
        assert_eq!(ERR138.name(), "ERR138");
        assert_eq!(tighterror::Kind::name(& ERR138), "ERR138");
        assert_eq!(ERR139.name(), "ERR139");
        assert_eq!(tighterror::Kind::name(& ERR139), "ERR139");
        assert_eq!(ERR140.name(), "ERR140");
        assert_eq!(tighterror::Kind::name(& ERR140), "ERR140");
        assert_eq!(ERR141.name(), "ERR141");
        assert_eq!(tighterror::Kind::name(& ERR141), "ERR141");
        assert_eq!(ERR142.name(), "ERR142");
        assert_eq!(tighterror::Kind::name(& ERR142), "ERR142");
        assert_eq!(ERR143.name(), "ERR143");
        assert_eq!(tighterror::Kind::name(& ERR143), "ERR143");
        assert_eq!(ERR144.name(), "ERR144");
        assert_eq!(tighterror::Kind::name(& ERR144), "ERR144");
        assert_eq!(ERR145.name(), "ERR145");
        assert_eq!(tighterror::Kind::name(& ERR145), "ERR145");
        assert_eq!(ERR146.name(), "ERR146");
        assert_eq!(tighterror::Kind::name(& ERR146), "ERR146");
        assert_eq!(ERR147.name(), "ERR147");
        assert_eq!(tighterror::Kind::name(& ERR147), "ERR147");
        assert_eq!(ERR148.name(), "ERR148");
        assert_eq!(tighterror::Kind::name(& ERR148), "ERR148");
        assert_eq!(ERR149.name(), "ERR149");
        assert_eq!(tighterror::Kind::name(& ERR149), "ERR149");
        assert_eq!(ERR150.name(), "ERR150");
        assert_eq!(tighterror::Kind::name(& ERR150), "ERR150");
        assert_eq!(ERR151.name(), "ERR151");
        assert_eq!(tighterror::Kind::name(& ERR151), "ERR151");
        assert_eq!(ERR152.name(), "ERR152");
        assert_eq!(tighterror::Kind::name(& ERR152), "ERR152");
        assert_eq!(ERR153.name(), "ERR153");
        assert_eq!(tighterror::Kind::name(& ERR153), "ERR153");
        assert_eq!(ERR154.name(), "ERR154");
        assert_eq!(tighterror::Kind::name(& ERR154), "ERR154");
        assert_eq!(ERR155.name(), "ERR155");
        assert_eq!(tighterror::Kind::name(& ERR155), "ERR155");
        assert_eq!(ERR156.name(), "ERR156");
        assert_eq!(tighterror::Kind::name(& ERR156), "ERR156");
        assert_eq!(ERR157.name(), "ERR157");
        assert_eq!(tighterror::Kind::name(& ERR157), "ERR157");
        assert_eq!(ERR158.name(), "ERR158");
        assert_eq!(tighterror::Kind::name(& ERR158), "ERR158");
        assert_eq!(ERR159.name(), "ERR159");
        assert_eq!(tighterror::Kind::name(& ERR159), "ERR159");
        assert_eq!(ERR160.name(), "ERR160");
        assert_eq!(tighterror::Kind::name(& ERR160), "ERR160");
        assert_eq!(ERR161.name(), "ERR161");
        assert_eq!(tighterror::Kind::name(& ERR161), "ERR161");
        assert_eq!(ERR162.name(), "ERR162");
        assert_eq!(tighterror::Kind::name(& ERR162), "ERR162");
        assert_eq!(ERR163.name(), "ERR163");
        assert_eq!(tighterror::Kind::name(& ERR163), "ERR163");
        assert_eq!(ERR164.name(), "ERR164");
        assert_eq!(tighterror::Kind::name(& ERR164), "ERR164");
        assert_eq!(ERR165.name(), "ERR165");
        assert_eq!(tighterror::Kind::name(& ERR165), "ERR165");
        assert_eq!(ERR166.name(), "ERR166");
        assert_eq!(tighterror::Kind::name(& ERR166), "ERR166");
        assert_eq!(ERR167.name(), "ERR167");
        assert_eq!(tighterror::Kind::name(& ERR167), "ERR167");
        assert_eq!(ERR168.name(), "ERR168");
        assert_eq!(tighterror::Kind::name(& ERR168), "ERR168");
        assert_eq!(ERR169.name(), "ERR169");
        assert_eq!(tighterror::Kind::name(& ERR169), "ERR169");
        assert_eq!(ERR170.name(), "ERR170");
        assert_eq!(tighterror::Kind::name(& ERR170), "ERR170");
        assert_eq!(ERR171.name(), "ERR171");
        assert_eq!(tighterror::Kind::name(& ERR171), "ERR171");
        assert_eq!(ERR172.name(), "ERR172");
        assert_eq!(tighterror::Kind::name(& ERR172), "ERR172");
        assert_eq!(ERR173.name(), "ERR173");
        assert_eq!(tighterror::Kind::name(& ERR173), "ERR173");
        assert_eq!(ERR174.name(), "ERR174");
        assert_eq!(tighterror::Kind::name(& ERR174), "ERR174");
        assert_eq!(ERR175.name(), "ERR175");
        assert_eq!(tighterror::Kind::name(& ERR175), "ERR175");
        assert_eq!(ERR176.name(), "ERR176");
        assert_eq!(tighterror::Kind::name(& ERR176), "ERR176");
        assert_eq!(ERR177.name(), "ERR177");
        assert_eq!(tighterror::Kind::name(& ERR177), "ERR177");
        assert_eq!(ERR178.name(), "ERR178");
        assert_eq!(tighterror::Kind::name(& ERR178), "ERR178");
        assert_eq!(ERR179.name(), "ERR179");
        assert_eq!(tighterror::Kind::name(& ERR179), "ERR179");
        assert_eq!(ERR180.name(), "ERR180");
        assert_eq!(tighterror::Kind::name(& ERR180), "ERR180");
        assert_eq!(ERR181.name(), "ERR181");
        assert_eq!(tighterror::Kind::name(& ERR181), "ERR181");
        assert_eq!(ERR182.name(), "ERR182");
        assert_eq!(tighterror::Kind::name(& ERR182), "ERR182");
        assert_eq!(ERR183.name(), "ERR183");
        assert_eq!(tighterror::Kind::name(& ERR183), "ERR183");
        assert_eq!(ERR184.name(), "ERR184");
        assert_eq!(tighterror::Kind::name(& ERR184), "ERR184");
        assert_eq!(ERR185.name(), "ERR185");
        assert_eq!(tighterror::Kind::name(& ERR185), "ERR185");
        assert_eq!(ERR186.name(), "ERR186");
        assert_eq!(tighterror::Kind::name(& ERR186), "ERR186");
        assert_eq!(ERR187.name(), "ERR187");
        assert_eq!(tighterror::Kind::name(& ERR187), "ERR187");
        assert_eq!(ERR188.name(), "ERR188");
        assert_eq!(tighterror::Kind::name(& ERR188), "ERR188");
        assert_eq!(ERR189.name(), "ERR189");
        assert_eq!(tighterror::Kind::name(& ERR189), "ERR189");
        assert_eq!(ERR190.name(), "ERR190");
        assert_eq!(tighterror::Kind::name(& ERR190), "ERR190");
        assert_eq!(ERR191.name(), "ERR191");
        assert_eq!(tighterror::Kind::name(& ERR191), "ERR191");
        assert_eq!(ERR192.name(), "ERR192");
        assert_eq!(tighterror::Kind::name(& ERR192), "ERR192");
        assert_eq!(ERR193.name(), "ERR193");
        assert_eq!(tighterror::Kind::name(& ERR193), "ERR193");
        assert_eq!(ERR194.name(), "ERR194");
        assert_eq!(tighterror::Kind::name(& ERR194), "ERR194");
        assert_eq!(ERR195.name(), "ERR195");
        assert_eq!(tighterror::Kind::name(& ERR195), "ERR195");
        assert_eq!(ERR196.name(), "ERR196");
        assert_eq!(tighterror::Kind::name(& ERR196), "ERR196");
        assert_eq!(ERR197.name(), "ERR197");
        assert_eq!(tighterror::Kind::name(& ERR197), "ERR197");
        assert_eq!(ERR198.name(), "ERR198");
        assert_eq!(tighterror::Kind::name(& ERR198), "ERR198");
        assert_eq!(ERR199.name(), "ERR199");
        assert_eq!(tighterror::Kind::name(& ERR199), "ERR199");
        assert_eq!(ERR200.name(), "ERR200");
        assert_eq!(tighterror::Kind::name(& ERR200), "ERR200");
        assert_eq!(ERR201.name(), "ERR201");
        assert_eq!(tighterror::Kind::name(& ERR201), "ERR201");
        assert_eq!(ERR202.name(), "ERR202");
        assert_eq!(tighterror::Kind::name(& ERR202), "ERR202");
        assert_eq!(ERR203.name(), "ERR203");
        assert_eq!(tighterror::Kind::name(& ERR203), "ERR203");
        assert_eq!(ERR204.name(), "ERR204");
        assert_eq!(tighterror::Kind::name(& ERR204), "ERR204");
        assert_eq!(ERR205.name(), "ERR205");
        assert_eq!(tighterror::Kind::name(& ERR205), "ERR205");
        assert_eq!(ERR206.name(), "ERR206");
        assert_eq!(tighterror::Kind::name(& ERR206), "ERR206");
        assert_eq!(ERR207.name(), "ERR207");
        assert_eq!(tighterror::Kind::name(& ERR207), "ERR207");
        assert_eq!(ERR208.name(), "ERR208");
        assert_eq!(tighterror::Kind::name(& ERR208), "ERR208");
        assert_eq!(ERR209.name(), "ERR209");
        assert_eq!(tighterror::Kind::name(& ERR209), "ERR209");
        assert_eq!(ERR210.name(), "ERR210");
        assert_eq!(tighterror::Kind::name(& ERR210), "ERR210");
        assert_eq!(ERR211.name(), "ERR211");
        assert_eq!(tighterror::Kind::name(& ERR211), "ERR211");
        assert_eq!(ERR212.name(), "ERR212");
        assert_eq!(tighterror::Kind::name(& ERR212), "ERR212");
        assert_eq!(ERR213.name(), "ERR213");
        assert_eq!(tighterror::Kind::name(& ERR213), "ERR213");
        assert_eq!(ERR214.name(), "ERR214");
        assert_eq!(tighterror::Kind::name(& ERR214), "ERR214");
        assert_eq!(ERR215.name(), "ERR215");
        assert_eq!(tighterror::Kind::name(& ERR215), "ERR215");
        assert_eq!(ERR216.name(), "ERR216");
        assert_eq!(tighterror::Kind::name(& ERR216), "ERR216");
        assert_eq!(ERR217.name(), "ERR217");
        assert_eq!(tighterror::Kind::name(& ERR217), "ERR217");
        assert_eq!(ERR218.name(), "ERR218");
        assert_eq!(tighterror::Kind::name(& ERR218), "ERR218");
        assert_eq!(ERR219.name(), "ERR219");
        assert_eq!(tighterror::Kind::name(& ERR219), "ERR219");
        assert_eq!(ERR220.name(), "ERR220");
        assert_eq!(tighterror::Kind::name(& ERR220), "ERR220");
        assert_eq!(ERR221.name(), "ERR221");
        assert_eq!(tighterror::Kind::name(& ERR221), "ERR221");
        assert_eq!(ERR222.name(), "ERR222");
        assert_eq!(tighterror::Kind::name(& ERR222), "ERR222");
        assert_eq!(ERR223.name(), "ERR223");
        assert_eq!(tighterror::Kind::name(& ERR223), "ERR223");
        assert_eq!(ERR224.name(), "ERR224");
        assert_eq!(tighterror::Kind::name(& ERR224), "ERR224");
        assert_eq!(ERR225.name(), "ERR225");
        assert_eq!(tighterror::Kind::name(& ERR225), "ERR225");
        assert_eq!(ERR226.name(), "ERR226");
        assert_eq!(tighterror::Kind::name(& ERR226), "ERR226");
        assert_eq!(ERR227.name(), "ERR227");
        assert_eq!(tighterror::Kind::name(& ERR227), "ERR227");
        assert_eq!(ERR228.name(), "ERR228");
        assert_eq!(tighterror::Kind::name(& ERR228), "ERR228");
        assert_eq!(ERR229.name(), "ERR229");
        assert_eq!(tighterror::Kind::name(& ERR229), "ERR229");
        assert_eq!(ERR230.name(), "ERR230");
        assert_eq!(tighterror::Kind::name(& ERR230), "ERR230");
        assert_eq!(ERR231.name(), "ERR231");
        assert_eq!(tighterror::Kind::name(& ERR231), "ERR231");
        assert_eq!(ERR232.name(), "ERR232");
        assert_eq!(tighterror::Kind::name(& ERR232), "ERR232");
        assert_eq!(ERR233.name(), "ERR233");
        assert_eq!(tighterror::Kind::name(& ERR233), "ERR233");
        assert_eq!(ERR234.name(), "ERR234");
        assert_eq!(tighterror::Kind::name(& ERR234), "ERR234");
        assert_eq!(ERR235.name(), "ERR235");
        assert_eq!(tighterror::Kind::name(& ERR235), "ERR235");
        assert_eq!(ERR236.name(), "ERR236");
        assert_eq!(tighterror::Kind::name(& ERR236), "ERR236");
        assert_eq!(ERR237.name(), "ERR237");
        assert_eq!(tighterror::Kind::name(& ERR237), "ERR237");
        assert_eq!(ERR238.name(), "ERR238");
        assert_eq!(tighterror::Kind::name(& ERR238), "ERR238");
        assert_eq!(ERR239.name(), "ERR239");
        assert_eq!(tighterror::Kind::name(& ERR239), "ERR239");
        assert_eq!(ERR240.name(), "ERR240");
        assert_eq!(tighterror::Kind::name(& ERR240), "ERR240");
        assert_eq!(ERR241.name(), "ERR241");
        assert_eq!(tighterror::Kind::name(& ERR241), "ERR241");
        assert_eq!(ERR242.name(), "ERR242");
        assert_eq!(tighterror::Kind::name(& ERR242), "ERR242");
        assert_eq!(ERR243.name(), "ERR243");
        assert_eq!(tighterror::Kind::name(& ERR243), "ERR243");
        assert_eq!(ERR244.name(), "ERR244");
        assert_eq!(tighterror::Kind::name(& ERR244), "ERR244");
        assert_eq!(ERR245.name(), "ERR245");
        assert_eq!(tighterror::Kind::name(& ERR245), "ERR245");
        assert_eq!(ERR246.name(), "ERR246");
        assert_eq!(tighterror::Kind::name(& ERR246), "ERR246");
        assert_eq!(ERR247.name(), "ERR247");
        assert_eq!(tighterror::Kind::name(& ERR247), "ERR247");
        assert_eq!(ERR248.name(), "ERR248");
        assert_eq!(tighterror::Kind::name(& ERR248), "ERR248");
        assert_eq!(ERR249.name(), "ERR249");
        assert_eq!(tighterror::Kind::name(& ERR249), "ERR249");
        assert_eq!(ERR250.name(), "ERR250");
        assert_eq!(tighterror::Kind::name(& ERR250), "ERR250");
        assert_eq!(ERR251.name(), "ERR251");
        assert_eq!(tighterror::Kind::name(& ERR251), "ERR251");
        assert_eq!(ERR252.name(), "ERR252");
        assert_eq!(tighterror::Kind::name(& ERR252), "ERR252");
        assert_eq!(ERR253.name(), "ERR253");
        assert_eq!(tighterror::Kind::name(& ERR253), "ERR253");
        assert_eq!(ERR254.name(), "ERR254");
        assert_eq!(tighterror::Kind::name(& ERR254), "ERR254");
        assert_eq!(ERR255.name(), "ERR255");
        assert_eq!(tighterror::Kind::name(& ERR255), "ERR255");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
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
        assert_eq!(format!("{}", ERR127), "ERR127");
        assert_eq!(format!("{}", ERR128), "ERR128");
        assert_eq!(format!("{}", ERR129), "ERR129");
        assert_eq!(format!("{}", ERR130), "ERR130");
        assert_eq!(format!("{}", ERR131), "ERR131");
        assert_eq!(format!("{}", ERR132), "ERR132");
        assert_eq!(format!("{}", ERR133), "ERR133");
        assert_eq!(format!("{}", ERR134), "ERR134");
        assert_eq!(format!("{}", ERR135), "ERR135");
        assert_eq!(format!("{}", ERR136), "ERR136");
        assert_eq!(format!("{}", ERR137), "ERR137");
        assert_eq!(format!("{}", ERR138), "ERR138");
        assert_eq!(format!("{}", ERR139), "ERR139");
        assert_eq!(format!("{}", ERR140), "ERR140");
        assert_eq!(format!("{}", ERR141), "ERR141");
        assert_eq!(format!("{}", ERR142), "ERR142");
        assert_eq!(format!("{}", ERR143), "ERR143");
        assert_eq!(format!("{}", ERR144), "ERR144");
        assert_eq!(format!("{}", ERR145), "ERR145");
        assert_eq!(format!("{}", ERR146), "ERR146");
        assert_eq!(format!("{}", ERR147), "ERR147");
        assert_eq!(format!("{}", ERR148), "ERR148");
        assert_eq!(format!("{}", ERR149), "ERR149");
        assert_eq!(format!("{}", ERR150), "ERR150");
        assert_eq!(format!("{}", ERR151), "ERR151");
        assert_eq!(format!("{}", ERR152), "ERR152");
        assert_eq!(format!("{}", ERR153), "ERR153");
        assert_eq!(format!("{}", ERR154), "ERR154");
        assert_eq!(format!("{}", ERR155), "ERR155");
        assert_eq!(format!("{}", ERR156), "ERR156");
        assert_eq!(format!("{}", ERR157), "ERR157");
        assert_eq!(format!("{}", ERR158), "ERR158");
        assert_eq!(format!("{}", ERR159), "ERR159");
        assert_eq!(format!("{}", ERR160), "ERR160");
        assert_eq!(format!("{}", ERR161), "ERR161");
        assert_eq!(format!("{}", ERR162), "ERR162");
        assert_eq!(format!("{}", ERR163), "ERR163");
        assert_eq!(format!("{}", ERR164), "ERR164");
        assert_eq!(format!("{}", ERR165), "ERR165");
        assert_eq!(format!("{}", ERR166), "ERR166");
        assert_eq!(format!("{}", ERR167), "ERR167");
        assert_eq!(format!("{}", ERR168), "ERR168");
        assert_eq!(format!("{}", ERR169), "ERR169");
        assert_eq!(format!("{}", ERR170), "ERR170");
        assert_eq!(format!("{}", ERR171), "ERR171");
        assert_eq!(format!("{}", ERR172), "ERR172");
        assert_eq!(format!("{}", ERR173), "ERR173");
        assert_eq!(format!("{}", ERR174), "ERR174");
        assert_eq!(format!("{}", ERR175), "ERR175");
        assert_eq!(format!("{}", ERR176), "ERR176");
        assert_eq!(format!("{}", ERR177), "ERR177");
        assert_eq!(format!("{}", ERR178), "ERR178");
        assert_eq!(format!("{}", ERR179), "ERR179");
        assert_eq!(format!("{}", ERR180), "ERR180");
        assert_eq!(format!("{}", ERR181), "ERR181");
        assert_eq!(format!("{}", ERR182), "ERR182");
        assert_eq!(format!("{}", ERR183), "ERR183");
        assert_eq!(format!("{}", ERR184), "ERR184");
        assert_eq!(format!("{}", ERR185), "ERR185");
        assert_eq!(format!("{}", ERR186), "ERR186");
        assert_eq!(format!("{}", ERR187), "ERR187");
        assert_eq!(format!("{}", ERR188), "ERR188");
        assert_eq!(format!("{}", ERR189), "ERR189");
        assert_eq!(format!("{}", ERR190), "ERR190");
        assert_eq!(format!("{}", ERR191), "ERR191");
        assert_eq!(format!("{}", ERR192), "ERR192");
        assert_eq!(format!("{}", ERR193), "ERR193");
        assert_eq!(format!("{}", ERR194), "ERR194");
        assert_eq!(format!("{}", ERR195), "ERR195");
        assert_eq!(format!("{}", ERR196), "ERR196");
        assert_eq!(format!("{}", ERR197), "ERR197");
        assert_eq!(format!("{}", ERR198), "ERR198");
        assert_eq!(format!("{}", ERR199), "ERR199");
        assert_eq!(format!("{}", ERR200), "ERR200");
        assert_eq!(format!("{}", ERR201), "ERR201");
        assert_eq!(format!("{}", ERR202), "ERR202");
        assert_eq!(format!("{}", ERR203), "ERR203");
        assert_eq!(format!("{}", ERR204), "ERR204");
        assert_eq!(format!("{}", ERR205), "ERR205");
        assert_eq!(format!("{}", ERR206), "ERR206");
        assert_eq!(format!("{}", ERR207), "ERR207");
        assert_eq!(format!("{}", ERR208), "ERR208");
        assert_eq!(format!("{}", ERR209), "ERR209");
        assert_eq!(format!("{}", ERR210), "ERR210");
        assert_eq!(format!("{}", ERR211), "ERR211");
        assert_eq!(format!("{}", ERR212), "ERR212");
        assert_eq!(format!("{}", ERR213), "ERR213");
        assert_eq!(format!("{}", ERR214), "ERR214");
        assert_eq!(format!("{}", ERR215), "ERR215");
        assert_eq!(format!("{}", ERR216), "ERR216");
        assert_eq!(format!("{}", ERR217), "ERR217");
        assert_eq!(format!("{}", ERR218), "ERR218");
        assert_eq!(format!("{}", ERR219), "ERR219");
        assert_eq!(format!("{}", ERR220), "ERR220");
        assert_eq!(format!("{}", ERR221), "ERR221");
        assert_eq!(format!("{}", ERR222), "ERR222");
        assert_eq!(format!("{}", ERR223), "ERR223");
        assert_eq!(format!("{}", ERR224), "ERR224");
        assert_eq!(format!("{}", ERR225), "ERR225");
        assert_eq!(format!("{}", ERR226), "ERR226");
        assert_eq!(format!("{}", ERR227), "ERR227");
        assert_eq!(format!("{}", ERR228), "ERR228");
        assert_eq!(format!("{}", ERR229), "ERR229");
        assert_eq!(format!("{}", ERR230), "ERR230");
        assert_eq!(format!("{}", ERR231), "ERR231");
        assert_eq!(format!("{}", ERR232), "ERR232");
        assert_eq!(format!("{}", ERR233), "ERR233");
        assert_eq!(format!("{}", ERR234), "ERR234");
        assert_eq!(format!("{}", ERR235), "ERR235");
        assert_eq!(format!("{}", ERR236), "ERR236");
        assert_eq!(format!("{}", ERR237), "ERR237");
        assert_eq!(format!("{}", ERR238), "ERR238");
        assert_eq!(format!("{}", ERR239), "ERR239");
        assert_eq!(format!("{}", ERR240), "ERR240");
        assert_eq!(format!("{}", ERR241), "ERR241");
        assert_eq!(format!("{}", ERR242), "ERR242");
        assert_eq!(format!("{}", ERR243), "ERR243");
        assert_eq!(format!("{}", ERR244), "ERR244");
        assert_eq!(format!("{}", ERR245), "ERR245");
        assert_eq!(format!("{}", ERR246), "ERR246");
        assert_eq!(format!("{}", ERR247), "ERR247");
        assert_eq!(format!("{}", ERR248), "ERR248");
        assert_eq!(format!("{}", ERR249), "ERR249");
        assert_eq!(format!("{}", ERR250), "ERR250");
        assert_eq!(format!("{}", ERR251), "ERR251");
        assert_eq!(format!("{}", ERR252), "ERR252");
        assert_eq!(format!("{}", ERR253), "ERR253");
        assert_eq!(format!("{}", ERR254), "ERR254");
        assert_eq!(format!("{}", ERR255), "ERR255");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 256] = [
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
            ERR127,
            ERR128,
            ERR129,
            ERR130,
            ERR131,
            ERR132,
            ERR133,
            ERR134,
            ERR135,
            ERR136,
            ERR137,
            ERR138,
            ERR139,
            ERR140,
            ERR141,
            ERR142,
            ERR143,
            ERR144,
            ERR145,
            ERR146,
            ERR147,
            ERR148,
            ERR149,
            ERR150,
            ERR151,
            ERR152,
            ERR153,
            ERR154,
            ERR155,
            ERR156,
            ERR157,
            ERR158,
            ERR159,
            ERR160,
            ERR161,
            ERR162,
            ERR163,
            ERR164,
            ERR165,
            ERR166,
            ERR167,
            ERR168,
            ERR169,
            ERR170,
            ERR171,
            ERR172,
            ERR173,
            ERR174,
            ERR175,
            ERR176,
            ERR177,
            ERR178,
            ERR179,
            ERR180,
            ERR181,
            ERR182,
            ERR183,
            ERR184,
            ERR185,
            ERR186,
            ERR187,
            ERR188,
            ERR189,
            ERR190,
            ERR191,
            ERR192,
            ERR193,
            ERR194,
            ERR195,
            ERR196,
            ERR197,
            ERR198,
            ERR199,
            ERR200,
            ERR201,
            ERR202,
            ERR203,
            ERR204,
            ERR205,
            ERR206,
            ERR207,
            ERR208,
            ERR209,
            ERR210,
            ERR211,
            ERR212,
            ERR213,
            ERR214,
            ERR215,
            ERR216,
            ERR217,
            ERR218,
            ERR219,
            ERR220,
            ERR221,
            ERR222,
            ERR223,
            ERR224,
            ERR225,
            ERR226,
            ERR227,
            ERR228,
            ERR229,
            ERR230,
            ERR231,
            ERR232,
            ERR233,
            ERR234,
            ERR235,
            ERR236,
            ERR237,
            ERR238,
            ERR239,
            ERR240,
            ERR241,
            ERR242,
            ERR243,
            ERR244,
            ERR245,
            ERR246,
            ERR247,
            ERR248,
            ERR249,
            ERR250,
            ERR251,
            ERR252,
            ERR253,
            ERR254,
            ERR255,
        ];

        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 256);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 256] = [
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
            ERR127,
            ERR128,
            ERR129,
            ERR130,
            ERR131,
            ERR132,
            ERR133,
            ERR134,
            ERR135,
            ERR136,
            ERR137,
            ERR138,
            ERR139,
            ERR140,
            ERR141,
            ERR142,
            ERR143,
            ERR144,
            ERR145,
            ERR146,
            ERR147,
            ERR148,
            ERR149,
            ERR150,
            ERR151,
            ERR152,
            ERR153,
            ERR154,
            ERR155,
            ERR156,
            ERR157,
            ERR158,
            ERR159,
            ERR160,
            ERR161,
            ERR162,
            ERR163,
            ERR164,
            ERR165,
            ERR166,
            ERR167,
            ERR168,
            ERR169,
            ERR170,
            ERR171,
            ERR172,
            ERR173,
            ERR174,
            ERR175,
            ERR176,
            ERR177,
            ERR178,
            ERR179,
            ERR180,
            ERR181,
            ERR182,
            ERR183,
            ERR184,
            ERR185,
            ERR186,
            ERR187,
            ERR188,
            ERR189,
            ERR190,
            ERR191,
            ERR192,
            ERR193,
            ERR194,
            ERR195,
            ERR196,
            ERR197,
            ERR198,
            ERR199,
            ERR200,
            ERR201,
            ERR202,
            ERR203,
            ERR204,
            ERR205,
            ERR206,
            ERR207,
            ERR208,
            ERR209,
            ERR210,
            ERR211,
            ERR212,
            ERR213,
            ERR214,
            ERR215,
            ERR216,
            ERR217,
            ERR218,
            ERR219,
            ERR220,
            ERR221,
            ERR222,
            ERR223,
            ERR224,
            ERR225,
            ERR226,
            ERR227,
            ERR228,
            ERR229,
            ERR230,
            ERR231,
            ERR232,
            ERR233,
            ERR234,
            ERR235,
            ERR236,
            ERR237,
            ERR238,
            ERR239,
            ERR240,
            ERR241,
            ERR242,
            ERR243,
            ERR244,
            ERR245,
            ERR246,
            ERR247,
            ERR248,
            ERR249,
            ERR250,
            ERR251,
            ERR252,
            ERR253,
            ERR254,
            ERR255,
        ];

        let set = HashSet::<_p::R>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 256);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(ERR0.category(), category::GENERAL);
        assert_eq!(ERR1.category(), category::GENERAL);
        assert_eq!(ERR2.category(), category::GENERAL);
        assert_eq!(ERR3.category(), category::GENERAL);
        assert_eq!(ERR4.category(), category::GENERAL);
        assert_eq!(ERR5.category(), category::GENERAL);
        assert_eq!(ERR6.category(), category::GENERAL);
        assert_eq!(ERR7.category(), category::GENERAL);
        assert_eq!(ERR8.category(), category::GENERAL);
        assert_eq!(ERR9.category(), category::GENERAL);
        assert_eq!(ERR10.category(), category::GENERAL);
        assert_eq!(ERR11.category(), category::GENERAL);
        assert_eq!(ERR12.category(), category::GENERAL);
        assert_eq!(ERR13.category(), category::GENERAL);
        assert_eq!(ERR14.category(), category::GENERAL);
        assert_eq!(ERR15.category(), category::GENERAL);
        assert_eq!(ERR16.category(), category::GENERAL);
        assert_eq!(ERR17.category(), category::GENERAL);
        assert_eq!(ERR18.category(), category::GENERAL);
        assert_eq!(ERR19.category(), category::GENERAL);
        assert_eq!(ERR20.category(), category::GENERAL);
        assert_eq!(ERR21.category(), category::GENERAL);
        assert_eq!(ERR22.category(), category::GENERAL);
        assert_eq!(ERR23.category(), category::GENERAL);
        assert_eq!(ERR24.category(), category::GENERAL);
        assert_eq!(ERR25.category(), category::GENERAL);
        assert_eq!(ERR26.category(), category::GENERAL);
        assert_eq!(ERR27.category(), category::GENERAL);
        assert_eq!(ERR28.category(), category::GENERAL);
        assert_eq!(ERR29.category(), category::GENERAL);
        assert_eq!(ERR30.category(), category::GENERAL);
        assert_eq!(ERR31.category(), category::GENERAL);
        assert_eq!(ERR32.category(), category::GENERAL);
        assert_eq!(ERR33.category(), category::GENERAL);
        assert_eq!(ERR34.category(), category::GENERAL);
        assert_eq!(ERR35.category(), category::GENERAL);
        assert_eq!(ERR36.category(), category::GENERAL);
        assert_eq!(ERR37.category(), category::GENERAL);
        assert_eq!(ERR38.category(), category::GENERAL);
        assert_eq!(ERR39.category(), category::GENERAL);
        assert_eq!(ERR40.category(), category::GENERAL);
        assert_eq!(ERR41.category(), category::GENERAL);
        assert_eq!(ERR42.category(), category::GENERAL);
        assert_eq!(ERR43.category(), category::GENERAL);
        assert_eq!(ERR44.category(), category::GENERAL);
        assert_eq!(ERR45.category(), category::GENERAL);
        assert_eq!(ERR46.category(), category::GENERAL);
        assert_eq!(ERR47.category(), category::GENERAL);
        assert_eq!(ERR48.category(), category::GENERAL);
        assert_eq!(ERR49.category(), category::GENERAL);
        assert_eq!(ERR50.category(), category::GENERAL);
        assert_eq!(ERR51.category(), category::GENERAL);
        assert_eq!(ERR52.category(), category::GENERAL);
        assert_eq!(ERR53.category(), category::GENERAL);
        assert_eq!(ERR54.category(), category::GENERAL);
        assert_eq!(ERR55.category(), category::GENERAL);
        assert_eq!(ERR56.category(), category::GENERAL);
        assert_eq!(ERR57.category(), category::GENERAL);
        assert_eq!(ERR58.category(), category::GENERAL);
        assert_eq!(ERR59.category(), category::GENERAL);
        assert_eq!(ERR60.category(), category::GENERAL);
        assert_eq!(ERR61.category(), category::GENERAL);
        assert_eq!(ERR62.category(), category::GENERAL);
        assert_eq!(ERR63.category(), category::GENERAL);
        assert_eq!(ERR64.category(), category::GENERAL);
        assert_eq!(ERR65.category(), category::GENERAL);
        assert_eq!(ERR66.category(), category::GENERAL);
        assert_eq!(ERR67.category(), category::GENERAL);
        assert_eq!(ERR68.category(), category::GENERAL);
        assert_eq!(ERR69.category(), category::GENERAL);
        assert_eq!(ERR70.category(), category::GENERAL);
        assert_eq!(ERR71.category(), category::GENERAL);
        assert_eq!(ERR72.category(), category::GENERAL);
        assert_eq!(ERR73.category(), category::GENERAL);
        assert_eq!(ERR74.category(), category::GENERAL);
        assert_eq!(ERR75.category(), category::GENERAL);
        assert_eq!(ERR76.category(), category::GENERAL);
        assert_eq!(ERR77.category(), category::GENERAL);
        assert_eq!(ERR78.category(), category::GENERAL);
        assert_eq!(ERR79.category(), category::GENERAL);
        assert_eq!(ERR80.category(), category::GENERAL);
        assert_eq!(ERR81.category(), category::GENERAL);
        assert_eq!(ERR82.category(), category::GENERAL);
        assert_eq!(ERR83.category(), category::GENERAL);
        assert_eq!(ERR84.category(), category::GENERAL);
        assert_eq!(ERR85.category(), category::GENERAL);
        assert_eq!(ERR86.category(), category::GENERAL);
        assert_eq!(ERR87.category(), category::GENERAL);
        assert_eq!(ERR88.category(), category::GENERAL);
        assert_eq!(ERR89.category(), category::GENERAL);
        assert_eq!(ERR90.category(), category::GENERAL);
        assert_eq!(ERR91.category(), category::GENERAL);
        assert_eq!(ERR92.category(), category::GENERAL);
        assert_eq!(ERR93.category(), category::GENERAL);
        assert_eq!(ERR94.category(), category::GENERAL);
        assert_eq!(ERR95.category(), category::GENERAL);
        assert_eq!(ERR96.category(), category::GENERAL);
        assert_eq!(ERR97.category(), category::GENERAL);
        assert_eq!(ERR98.category(), category::GENERAL);
        assert_eq!(ERR99.category(), category::GENERAL);
        assert_eq!(ERR100.category(), category::GENERAL);
        assert_eq!(ERR101.category(), category::GENERAL);
        assert_eq!(ERR102.category(), category::GENERAL);
        assert_eq!(ERR103.category(), category::GENERAL);
        assert_eq!(ERR104.category(), category::GENERAL);
        assert_eq!(ERR105.category(), category::GENERAL);
        assert_eq!(ERR106.category(), category::GENERAL);
        assert_eq!(ERR107.category(), category::GENERAL);
        assert_eq!(ERR108.category(), category::GENERAL);
        assert_eq!(ERR109.category(), category::GENERAL);
        assert_eq!(ERR110.category(), category::GENERAL);
        assert_eq!(ERR111.category(), category::GENERAL);
        assert_eq!(ERR112.category(), category::GENERAL);
        assert_eq!(ERR113.category(), category::GENERAL);
        assert_eq!(ERR114.category(), category::GENERAL);
        assert_eq!(ERR115.category(), category::GENERAL);
        assert_eq!(ERR116.category(), category::GENERAL);
        assert_eq!(ERR117.category(), category::GENERAL);
        assert_eq!(ERR118.category(), category::GENERAL);
        assert_eq!(ERR119.category(), category::GENERAL);
        assert_eq!(ERR120.category(), category::GENERAL);
        assert_eq!(ERR121.category(), category::GENERAL);
        assert_eq!(ERR122.category(), category::GENERAL);
        assert_eq!(ERR123.category(), category::GENERAL);
        assert_eq!(ERR124.category(), category::GENERAL);
        assert_eq!(ERR125.category(), category::GENERAL);
        assert_eq!(ERR126.category(), category::GENERAL);
        assert_eq!(ERR127.category(), category::GENERAL);
        assert_eq!(ERR128.category(), category::GENERAL);
        assert_eq!(ERR129.category(), category::GENERAL);
        assert_eq!(ERR130.category(), category::GENERAL);
        assert_eq!(ERR131.category(), category::GENERAL);
        assert_eq!(ERR132.category(), category::GENERAL);
        assert_eq!(ERR133.category(), category::GENERAL);
        assert_eq!(ERR134.category(), category::GENERAL);
        assert_eq!(ERR135.category(), category::GENERAL);
        assert_eq!(ERR136.category(), category::GENERAL);
        assert_eq!(ERR137.category(), category::GENERAL);
        assert_eq!(ERR138.category(), category::GENERAL);
        assert_eq!(ERR139.category(), category::GENERAL);
        assert_eq!(ERR140.category(), category::GENERAL);
        assert_eq!(ERR141.category(), category::GENERAL);
        assert_eq!(ERR142.category(), category::GENERAL);
        assert_eq!(ERR143.category(), category::GENERAL);
        assert_eq!(ERR144.category(), category::GENERAL);
        assert_eq!(ERR145.category(), category::GENERAL);
        assert_eq!(ERR146.category(), category::GENERAL);
        assert_eq!(ERR147.category(), category::GENERAL);
        assert_eq!(ERR148.category(), category::GENERAL);
        assert_eq!(ERR149.category(), category::GENERAL);
        assert_eq!(ERR150.category(), category::GENERAL);
        assert_eq!(ERR151.category(), category::GENERAL);
        assert_eq!(ERR152.category(), category::GENERAL);
        assert_eq!(ERR153.category(), category::GENERAL);
        assert_eq!(ERR154.category(), category::GENERAL);
        assert_eq!(ERR155.category(), category::GENERAL);
        assert_eq!(ERR156.category(), category::GENERAL);
        assert_eq!(ERR157.category(), category::GENERAL);
        assert_eq!(ERR158.category(), category::GENERAL);
        assert_eq!(ERR159.category(), category::GENERAL);
        assert_eq!(ERR160.category(), category::GENERAL);
        assert_eq!(ERR161.category(), category::GENERAL);
        assert_eq!(ERR162.category(), category::GENERAL);
        assert_eq!(ERR163.category(), category::GENERAL);
        assert_eq!(ERR164.category(), category::GENERAL);
        assert_eq!(ERR165.category(), category::GENERAL);
        assert_eq!(ERR166.category(), category::GENERAL);
        assert_eq!(ERR167.category(), category::GENERAL);
        assert_eq!(ERR168.category(), category::GENERAL);
        assert_eq!(ERR169.category(), category::GENERAL);
        assert_eq!(ERR170.category(), category::GENERAL);
        assert_eq!(ERR171.category(), category::GENERAL);
        assert_eq!(ERR172.category(), category::GENERAL);
        assert_eq!(ERR173.category(), category::GENERAL);
        assert_eq!(ERR174.category(), category::GENERAL);
        assert_eq!(ERR175.category(), category::GENERAL);
        assert_eq!(ERR176.category(), category::GENERAL);
        assert_eq!(ERR177.category(), category::GENERAL);
        assert_eq!(ERR178.category(), category::GENERAL);
        assert_eq!(ERR179.category(), category::GENERAL);
        assert_eq!(ERR180.category(), category::GENERAL);
        assert_eq!(ERR181.category(), category::GENERAL);
        assert_eq!(ERR182.category(), category::GENERAL);
        assert_eq!(ERR183.category(), category::GENERAL);
        assert_eq!(ERR184.category(), category::GENERAL);
        assert_eq!(ERR185.category(), category::GENERAL);
        assert_eq!(ERR186.category(), category::GENERAL);
        assert_eq!(ERR187.category(), category::GENERAL);
        assert_eq!(ERR188.category(), category::GENERAL);
        assert_eq!(ERR189.category(), category::GENERAL);
        assert_eq!(ERR190.category(), category::GENERAL);
        assert_eq!(ERR191.category(), category::GENERAL);
        assert_eq!(ERR192.category(), category::GENERAL);
        assert_eq!(ERR193.category(), category::GENERAL);
        assert_eq!(ERR194.category(), category::GENERAL);
        assert_eq!(ERR195.category(), category::GENERAL);
        assert_eq!(ERR196.category(), category::GENERAL);
        assert_eq!(ERR197.category(), category::GENERAL);
        assert_eq!(ERR198.category(), category::GENERAL);
        assert_eq!(ERR199.category(), category::GENERAL);
        assert_eq!(ERR200.category(), category::GENERAL);
        assert_eq!(ERR201.category(), category::GENERAL);
        assert_eq!(ERR202.category(), category::GENERAL);
        assert_eq!(ERR203.category(), category::GENERAL);
        assert_eq!(ERR204.category(), category::GENERAL);
        assert_eq!(ERR205.category(), category::GENERAL);
        assert_eq!(ERR206.category(), category::GENERAL);
        assert_eq!(ERR207.category(), category::GENERAL);
        assert_eq!(ERR208.category(), category::GENERAL);
        assert_eq!(ERR209.category(), category::GENERAL);
        assert_eq!(ERR210.category(), category::GENERAL);
        assert_eq!(ERR211.category(), category::GENERAL);
        assert_eq!(ERR212.category(), category::GENERAL);
        assert_eq!(ERR213.category(), category::GENERAL);
        assert_eq!(ERR214.category(), category::GENERAL);
        assert_eq!(ERR215.category(), category::GENERAL);
        assert_eq!(ERR216.category(), category::GENERAL);
        assert_eq!(ERR217.category(), category::GENERAL);
        assert_eq!(ERR218.category(), category::GENERAL);
        assert_eq!(ERR219.category(), category::GENERAL);
        assert_eq!(ERR220.category(), category::GENERAL);
        assert_eq!(ERR221.category(), category::GENERAL);
        assert_eq!(ERR222.category(), category::GENERAL);
        assert_eq!(ERR223.category(), category::GENERAL);
        assert_eq!(ERR224.category(), category::GENERAL);
        assert_eq!(ERR225.category(), category::GENERAL);
        assert_eq!(ERR226.category(), category::GENERAL);
        assert_eq!(ERR227.category(), category::GENERAL);
        assert_eq!(ERR228.category(), category::GENERAL);
        assert_eq!(ERR229.category(), category::GENERAL);
        assert_eq!(ERR230.category(), category::GENERAL);
        assert_eq!(ERR231.category(), category::GENERAL);
        assert_eq!(ERR232.category(), category::GENERAL);
        assert_eq!(ERR233.category(), category::GENERAL);
        assert_eq!(ERR234.category(), category::GENERAL);
        assert_eq!(ERR235.category(), category::GENERAL);
        assert_eq!(ERR236.category(), category::GENERAL);
        assert_eq!(ERR237.category(), category::GENERAL);
        assert_eq!(ERR238.category(), category::GENERAL);
        assert_eq!(ERR239.category(), category::GENERAL);
        assert_eq!(ERR240.category(), category::GENERAL);
        assert_eq!(ERR241.category(), category::GENERAL);
        assert_eq!(ERR242.category(), category::GENERAL);
        assert_eq!(ERR243.category(), category::GENERAL);
        assert_eq!(ERR244.category(), category::GENERAL);
        assert_eq!(ERR245.category(), category::GENERAL);
        assert_eq!(ERR246.category(), category::GENERAL);
        assert_eq!(ERR247.category(), category::GENERAL);
        assert_eq!(ERR248.category(), category::GENERAL);
        assert_eq!(ERR249.category(), category::GENERAL);
        assert_eq!(ERR250.category(), category::GENERAL);
        assert_eq!(ERR251.category(), category::GENERAL);
        assert_eq!(ERR252.category(), category::GENERAL);
        assert_eq!(ERR253.category(), category::GENERAL);
        assert_eq!(ERR254.category(), category::GENERAL);
        assert_eq!(ERR255.category(), category::GENERAL);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
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
        assert_eq!(ErrorKind::from_value(ERR127.value()).unwrap(), ERR127);
        assert_eq!(ErrorKind::from_value(ERR128.value()).unwrap(), ERR128);
        assert_eq!(ErrorKind::from_value(ERR129.value()).unwrap(), ERR129);
        assert_eq!(ErrorKind::from_value(ERR130.value()).unwrap(), ERR130);
        assert_eq!(ErrorKind::from_value(ERR131.value()).unwrap(), ERR131);
        assert_eq!(ErrorKind::from_value(ERR132.value()).unwrap(), ERR132);
        assert_eq!(ErrorKind::from_value(ERR133.value()).unwrap(), ERR133);
        assert_eq!(ErrorKind::from_value(ERR134.value()).unwrap(), ERR134);
        assert_eq!(ErrorKind::from_value(ERR135.value()).unwrap(), ERR135);
        assert_eq!(ErrorKind::from_value(ERR136.value()).unwrap(), ERR136);
        assert_eq!(ErrorKind::from_value(ERR137.value()).unwrap(), ERR137);
        assert_eq!(ErrorKind::from_value(ERR138.value()).unwrap(), ERR138);
        assert_eq!(ErrorKind::from_value(ERR139.value()).unwrap(), ERR139);
        assert_eq!(ErrorKind::from_value(ERR140.value()).unwrap(), ERR140);
        assert_eq!(ErrorKind::from_value(ERR141.value()).unwrap(), ERR141);
        assert_eq!(ErrorKind::from_value(ERR142.value()).unwrap(), ERR142);
        assert_eq!(ErrorKind::from_value(ERR143.value()).unwrap(), ERR143);
        assert_eq!(ErrorKind::from_value(ERR144.value()).unwrap(), ERR144);
        assert_eq!(ErrorKind::from_value(ERR145.value()).unwrap(), ERR145);
        assert_eq!(ErrorKind::from_value(ERR146.value()).unwrap(), ERR146);
        assert_eq!(ErrorKind::from_value(ERR147.value()).unwrap(), ERR147);
        assert_eq!(ErrorKind::from_value(ERR148.value()).unwrap(), ERR148);
        assert_eq!(ErrorKind::from_value(ERR149.value()).unwrap(), ERR149);
        assert_eq!(ErrorKind::from_value(ERR150.value()).unwrap(), ERR150);
        assert_eq!(ErrorKind::from_value(ERR151.value()).unwrap(), ERR151);
        assert_eq!(ErrorKind::from_value(ERR152.value()).unwrap(), ERR152);
        assert_eq!(ErrorKind::from_value(ERR153.value()).unwrap(), ERR153);
        assert_eq!(ErrorKind::from_value(ERR154.value()).unwrap(), ERR154);
        assert_eq!(ErrorKind::from_value(ERR155.value()).unwrap(), ERR155);
        assert_eq!(ErrorKind::from_value(ERR156.value()).unwrap(), ERR156);
        assert_eq!(ErrorKind::from_value(ERR157.value()).unwrap(), ERR157);
        assert_eq!(ErrorKind::from_value(ERR158.value()).unwrap(), ERR158);
        assert_eq!(ErrorKind::from_value(ERR159.value()).unwrap(), ERR159);
        assert_eq!(ErrorKind::from_value(ERR160.value()).unwrap(), ERR160);
        assert_eq!(ErrorKind::from_value(ERR161.value()).unwrap(), ERR161);
        assert_eq!(ErrorKind::from_value(ERR162.value()).unwrap(), ERR162);
        assert_eq!(ErrorKind::from_value(ERR163.value()).unwrap(), ERR163);
        assert_eq!(ErrorKind::from_value(ERR164.value()).unwrap(), ERR164);
        assert_eq!(ErrorKind::from_value(ERR165.value()).unwrap(), ERR165);
        assert_eq!(ErrorKind::from_value(ERR166.value()).unwrap(), ERR166);
        assert_eq!(ErrorKind::from_value(ERR167.value()).unwrap(), ERR167);
        assert_eq!(ErrorKind::from_value(ERR168.value()).unwrap(), ERR168);
        assert_eq!(ErrorKind::from_value(ERR169.value()).unwrap(), ERR169);
        assert_eq!(ErrorKind::from_value(ERR170.value()).unwrap(), ERR170);
        assert_eq!(ErrorKind::from_value(ERR171.value()).unwrap(), ERR171);
        assert_eq!(ErrorKind::from_value(ERR172.value()).unwrap(), ERR172);
        assert_eq!(ErrorKind::from_value(ERR173.value()).unwrap(), ERR173);
        assert_eq!(ErrorKind::from_value(ERR174.value()).unwrap(), ERR174);
        assert_eq!(ErrorKind::from_value(ERR175.value()).unwrap(), ERR175);
        assert_eq!(ErrorKind::from_value(ERR176.value()).unwrap(), ERR176);
        assert_eq!(ErrorKind::from_value(ERR177.value()).unwrap(), ERR177);
        assert_eq!(ErrorKind::from_value(ERR178.value()).unwrap(), ERR178);
        assert_eq!(ErrorKind::from_value(ERR179.value()).unwrap(), ERR179);
        assert_eq!(ErrorKind::from_value(ERR180.value()).unwrap(), ERR180);
        assert_eq!(ErrorKind::from_value(ERR181.value()).unwrap(), ERR181);
        assert_eq!(ErrorKind::from_value(ERR182.value()).unwrap(), ERR182);
        assert_eq!(ErrorKind::from_value(ERR183.value()).unwrap(), ERR183);
        assert_eq!(ErrorKind::from_value(ERR184.value()).unwrap(), ERR184);
        assert_eq!(ErrorKind::from_value(ERR185.value()).unwrap(), ERR185);
        assert_eq!(ErrorKind::from_value(ERR186.value()).unwrap(), ERR186);
        assert_eq!(ErrorKind::from_value(ERR187.value()).unwrap(), ERR187);
        assert_eq!(ErrorKind::from_value(ERR188.value()).unwrap(), ERR188);
        assert_eq!(ErrorKind::from_value(ERR189.value()).unwrap(), ERR189);
        assert_eq!(ErrorKind::from_value(ERR190.value()).unwrap(), ERR190);
        assert_eq!(ErrorKind::from_value(ERR191.value()).unwrap(), ERR191);
        assert_eq!(ErrorKind::from_value(ERR192.value()).unwrap(), ERR192);
        assert_eq!(ErrorKind::from_value(ERR193.value()).unwrap(), ERR193);
        assert_eq!(ErrorKind::from_value(ERR194.value()).unwrap(), ERR194);
        assert_eq!(ErrorKind::from_value(ERR195.value()).unwrap(), ERR195);
        assert_eq!(ErrorKind::from_value(ERR196.value()).unwrap(), ERR196);
        assert_eq!(ErrorKind::from_value(ERR197.value()).unwrap(), ERR197);
        assert_eq!(ErrorKind::from_value(ERR198.value()).unwrap(), ERR198);
        assert_eq!(ErrorKind::from_value(ERR199.value()).unwrap(), ERR199);
        assert_eq!(ErrorKind::from_value(ERR200.value()).unwrap(), ERR200);
        assert_eq!(ErrorKind::from_value(ERR201.value()).unwrap(), ERR201);
        assert_eq!(ErrorKind::from_value(ERR202.value()).unwrap(), ERR202);
        assert_eq!(ErrorKind::from_value(ERR203.value()).unwrap(), ERR203);
        assert_eq!(ErrorKind::from_value(ERR204.value()).unwrap(), ERR204);
        assert_eq!(ErrorKind::from_value(ERR205.value()).unwrap(), ERR205);
        assert_eq!(ErrorKind::from_value(ERR206.value()).unwrap(), ERR206);
        assert_eq!(ErrorKind::from_value(ERR207.value()).unwrap(), ERR207);
        assert_eq!(ErrorKind::from_value(ERR208.value()).unwrap(), ERR208);
        assert_eq!(ErrorKind::from_value(ERR209.value()).unwrap(), ERR209);
        assert_eq!(ErrorKind::from_value(ERR210.value()).unwrap(), ERR210);
        assert_eq!(ErrorKind::from_value(ERR211.value()).unwrap(), ERR211);
        assert_eq!(ErrorKind::from_value(ERR212.value()).unwrap(), ERR212);
        assert_eq!(ErrorKind::from_value(ERR213.value()).unwrap(), ERR213);
        assert_eq!(ErrorKind::from_value(ERR214.value()).unwrap(), ERR214);
        assert_eq!(ErrorKind::from_value(ERR215.value()).unwrap(), ERR215);
        assert_eq!(ErrorKind::from_value(ERR216.value()).unwrap(), ERR216);
        assert_eq!(ErrorKind::from_value(ERR217.value()).unwrap(), ERR217);
        assert_eq!(ErrorKind::from_value(ERR218.value()).unwrap(), ERR218);
        assert_eq!(ErrorKind::from_value(ERR219.value()).unwrap(), ERR219);
        assert_eq!(ErrorKind::from_value(ERR220.value()).unwrap(), ERR220);
        assert_eq!(ErrorKind::from_value(ERR221.value()).unwrap(), ERR221);
        assert_eq!(ErrorKind::from_value(ERR222.value()).unwrap(), ERR222);
        assert_eq!(ErrorKind::from_value(ERR223.value()).unwrap(), ERR223);
        assert_eq!(ErrorKind::from_value(ERR224.value()).unwrap(), ERR224);
        assert_eq!(ErrorKind::from_value(ERR225.value()).unwrap(), ERR225);
        assert_eq!(ErrorKind::from_value(ERR226.value()).unwrap(), ERR226);
        assert_eq!(ErrorKind::from_value(ERR227.value()).unwrap(), ERR227);
        assert_eq!(ErrorKind::from_value(ERR228.value()).unwrap(), ERR228);
        assert_eq!(ErrorKind::from_value(ERR229.value()).unwrap(), ERR229);
        assert_eq!(ErrorKind::from_value(ERR230.value()).unwrap(), ERR230);
        assert_eq!(ErrorKind::from_value(ERR231.value()).unwrap(), ERR231);
        assert_eq!(ErrorKind::from_value(ERR232.value()).unwrap(), ERR232);
        assert_eq!(ErrorKind::from_value(ERR233.value()).unwrap(), ERR233);
        assert_eq!(ErrorKind::from_value(ERR234.value()).unwrap(), ERR234);
        assert_eq!(ErrorKind::from_value(ERR235.value()).unwrap(), ERR235);
        assert_eq!(ErrorKind::from_value(ERR236.value()).unwrap(), ERR236);
        assert_eq!(ErrorKind::from_value(ERR237.value()).unwrap(), ERR237);
        assert_eq!(ErrorKind::from_value(ERR238.value()).unwrap(), ERR238);
        assert_eq!(ErrorKind::from_value(ERR239.value()).unwrap(), ERR239);
        assert_eq!(ErrorKind::from_value(ERR240.value()).unwrap(), ERR240);
        assert_eq!(ErrorKind::from_value(ERR241.value()).unwrap(), ERR241);
        assert_eq!(ErrorKind::from_value(ERR242.value()).unwrap(), ERR242);
        assert_eq!(ErrorKind::from_value(ERR243.value()).unwrap(), ERR243);
        assert_eq!(ErrorKind::from_value(ERR244.value()).unwrap(), ERR244);
        assert_eq!(ErrorKind::from_value(ERR245.value()).unwrap(), ERR245);
        assert_eq!(ErrorKind::from_value(ERR246.value()).unwrap(), ERR246);
        assert_eq!(ErrorKind::from_value(ERR247.value()).unwrap(), ERR247);
        assert_eq!(ErrorKind::from_value(ERR248.value()).unwrap(), ERR248);
        assert_eq!(ErrorKind::from_value(ERR249.value()).unwrap(), ERR249);
        assert_eq!(ErrorKind::from_value(ERR250.value()).unwrap(), ERR250);
        assert_eq!(ErrorKind::from_value(ERR251.value()).unwrap(), ERR251);
        assert_eq!(ErrorKind::from_value(ERR252.value()).unwrap(), ERR252);
        assert_eq!(ErrorKind::from_value(ERR253.value()).unwrap(), ERR253);
        assert_eq!(ErrorKind::from_value(ERR254.value()).unwrap(), ERR254);
        assert_eq!(ErrorKind::from_value(ERR255.value()).unwrap(), ERR255);
    }


    #[test]
    fn test_err_display() {
        use kind::*;
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
        assert_eq!(format!("{}", Error::from(ERR127)), "ERR127");
        assert_eq!(format!("{}", Error::from(ERR128)), "ERR128");
        assert_eq!(format!("{}", Error::from(ERR129)), "ERR129");
        assert_eq!(format!("{}", Error::from(ERR130)), "ERR130");
        assert_eq!(format!("{}", Error::from(ERR131)), "ERR131");
        assert_eq!(format!("{}", Error::from(ERR132)), "ERR132");
        assert_eq!(format!("{}", Error::from(ERR133)), "ERR133");
        assert_eq!(format!("{}", Error::from(ERR134)), "ERR134");
        assert_eq!(format!("{}", Error::from(ERR135)), "ERR135");
        assert_eq!(format!("{}", Error::from(ERR136)), "ERR136");
        assert_eq!(format!("{}", Error::from(ERR137)), "ERR137");
        assert_eq!(format!("{}", Error::from(ERR138)), "ERR138");
        assert_eq!(format!("{}", Error::from(ERR139)), "ERR139");
        assert_eq!(format!("{}", Error::from(ERR140)), "ERR140");
        assert_eq!(format!("{}", Error::from(ERR141)), "ERR141");
        assert_eq!(format!("{}", Error::from(ERR142)), "ERR142");
        assert_eq!(format!("{}", Error::from(ERR143)), "ERR143");
        assert_eq!(format!("{}", Error::from(ERR144)), "ERR144");
        assert_eq!(format!("{}", Error::from(ERR145)), "ERR145");
        assert_eq!(format!("{}", Error::from(ERR146)), "ERR146");
        assert_eq!(format!("{}", Error::from(ERR147)), "ERR147");
        assert_eq!(format!("{}", Error::from(ERR148)), "ERR148");
        assert_eq!(format!("{}", Error::from(ERR149)), "ERR149");
        assert_eq!(format!("{}", Error::from(ERR150)), "ERR150");
        assert_eq!(format!("{}", Error::from(ERR151)), "ERR151");
        assert_eq!(format!("{}", Error::from(ERR152)), "ERR152");
        assert_eq!(format!("{}", Error::from(ERR153)), "ERR153");
        assert_eq!(format!("{}", Error::from(ERR154)), "ERR154");
        assert_eq!(format!("{}", Error::from(ERR155)), "ERR155");
        assert_eq!(format!("{}", Error::from(ERR156)), "ERR156");
        assert_eq!(format!("{}", Error::from(ERR157)), "ERR157");
        assert_eq!(format!("{}", Error::from(ERR158)), "ERR158");
        assert_eq!(format!("{}", Error::from(ERR159)), "ERR159");
        assert_eq!(format!("{}", Error::from(ERR160)), "ERR160");
        assert_eq!(format!("{}", Error::from(ERR161)), "ERR161");
        assert_eq!(format!("{}", Error::from(ERR162)), "ERR162");
        assert_eq!(format!("{}", Error::from(ERR163)), "ERR163");
        assert_eq!(format!("{}", Error::from(ERR164)), "ERR164");
        assert_eq!(format!("{}", Error::from(ERR165)), "ERR165");
        assert_eq!(format!("{}", Error::from(ERR166)), "ERR166");
        assert_eq!(format!("{}", Error::from(ERR167)), "ERR167");
        assert_eq!(format!("{}", Error::from(ERR168)), "ERR168");
        assert_eq!(format!("{}", Error::from(ERR169)), "ERR169");
        assert_eq!(format!("{}", Error::from(ERR170)), "ERR170");
        assert_eq!(format!("{}", Error::from(ERR171)), "ERR171");
        assert_eq!(format!("{}", Error::from(ERR172)), "ERR172");
        assert_eq!(format!("{}", Error::from(ERR173)), "ERR173");
        assert_eq!(format!("{}", Error::from(ERR174)), "ERR174");
        assert_eq!(format!("{}", Error::from(ERR175)), "ERR175");
        assert_eq!(format!("{}", Error::from(ERR176)), "ERR176");
        assert_eq!(format!("{}", Error::from(ERR177)), "ERR177");
        assert_eq!(format!("{}", Error::from(ERR178)), "ERR178");
        assert_eq!(format!("{}", Error::from(ERR179)), "ERR179");
        assert_eq!(format!("{}", Error::from(ERR180)), "ERR180");
        assert_eq!(format!("{}", Error::from(ERR181)), "ERR181");
        assert_eq!(format!("{}", Error::from(ERR182)), "ERR182");
        assert_eq!(format!("{}", Error::from(ERR183)), "ERR183");
        assert_eq!(format!("{}", Error::from(ERR184)), "ERR184");
        assert_eq!(format!("{}", Error::from(ERR185)), "ERR185");
        assert_eq!(format!("{}", Error::from(ERR186)), "ERR186");
        assert_eq!(format!("{}", Error::from(ERR187)), "ERR187");
        assert_eq!(format!("{}", Error::from(ERR188)), "ERR188");
        assert_eq!(format!("{}", Error::from(ERR189)), "ERR189");
        assert_eq!(format!("{}", Error::from(ERR190)), "ERR190");
        assert_eq!(format!("{}", Error::from(ERR191)), "ERR191");
        assert_eq!(format!("{}", Error::from(ERR192)), "ERR192");
        assert_eq!(format!("{}", Error::from(ERR193)), "ERR193");
        assert_eq!(format!("{}", Error::from(ERR194)), "ERR194");
        assert_eq!(format!("{}", Error::from(ERR195)), "ERR195");
        assert_eq!(format!("{}", Error::from(ERR196)), "ERR196");
        assert_eq!(format!("{}", Error::from(ERR197)), "ERR197");
        assert_eq!(format!("{}", Error::from(ERR198)), "ERR198");
        assert_eq!(format!("{}", Error::from(ERR199)), "ERR199");
        assert_eq!(format!("{}", Error::from(ERR200)), "ERR200");
        assert_eq!(format!("{}", Error::from(ERR201)), "ERR201");
        assert_eq!(format!("{}", Error::from(ERR202)), "ERR202");
        assert_eq!(format!("{}", Error::from(ERR203)), "ERR203");
        assert_eq!(format!("{}", Error::from(ERR204)), "ERR204");
        assert_eq!(format!("{}", Error::from(ERR205)), "ERR205");
        assert_eq!(format!("{}", Error::from(ERR206)), "ERR206");
        assert_eq!(format!("{}", Error::from(ERR207)), "ERR207");
        assert_eq!(format!("{}", Error::from(ERR208)), "ERR208");
        assert_eq!(format!("{}", Error::from(ERR209)), "ERR209");
        assert_eq!(format!("{}", Error::from(ERR210)), "ERR210");
        assert_eq!(format!("{}", Error::from(ERR211)), "ERR211");
        assert_eq!(format!("{}", Error::from(ERR212)), "ERR212");
        assert_eq!(format!("{}", Error::from(ERR213)), "ERR213");
        assert_eq!(format!("{}", Error::from(ERR214)), "ERR214");
        assert_eq!(format!("{}", Error::from(ERR215)), "ERR215");
        assert_eq!(format!("{}", Error::from(ERR216)), "ERR216");
        assert_eq!(format!("{}", Error::from(ERR217)), "ERR217");
        assert_eq!(format!("{}", Error::from(ERR218)), "ERR218");
        assert_eq!(format!("{}", Error::from(ERR219)), "ERR219");
        assert_eq!(format!("{}", Error::from(ERR220)), "ERR220");
        assert_eq!(format!("{}", Error::from(ERR221)), "ERR221");
        assert_eq!(format!("{}", Error::from(ERR222)), "ERR222");
        assert_eq!(format!("{}", Error::from(ERR223)), "ERR223");
        assert_eq!(format!("{}", Error::from(ERR224)), "ERR224");
        assert_eq!(format!("{}", Error::from(ERR225)), "ERR225");
        assert_eq!(format!("{}", Error::from(ERR226)), "ERR226");
        assert_eq!(format!("{}", Error::from(ERR227)), "ERR227");
        assert_eq!(format!("{}", Error::from(ERR228)), "ERR228");
        assert_eq!(format!("{}", Error::from(ERR229)), "ERR229");
        assert_eq!(format!("{}", Error::from(ERR230)), "ERR230");
        assert_eq!(format!("{}", Error::from(ERR231)), "ERR231");
        assert_eq!(format!("{}", Error::from(ERR232)), "ERR232");
        assert_eq!(format!("{}", Error::from(ERR233)), "ERR233");
        assert_eq!(format!("{}", Error::from(ERR234)), "ERR234");
        assert_eq!(format!("{}", Error::from(ERR235)), "ERR235");
        assert_eq!(format!("{}", Error::from(ERR236)), "ERR236");
        assert_eq!(format!("{}", Error::from(ERR237)), "ERR237");
        assert_eq!(format!("{}", Error::from(ERR238)), "ERR238");
        assert_eq!(format!("{}", Error::from(ERR239)), "ERR239");
        assert_eq!(format!("{}", Error::from(ERR240)), "ERR240");
        assert_eq!(format!("{}", Error::from(ERR241)), "ERR241");
        assert_eq!(format!("{}", Error::from(ERR242)), "ERR242");
        assert_eq!(format!("{}", Error::from(ERR243)), "ERR243");
        assert_eq!(format!("{}", Error::from(ERR244)), "ERR244");
        assert_eq!(format!("{}", Error::from(ERR245)), "ERR245");
        assert_eq!(format!("{}", Error::from(ERR246)), "ERR246");
        assert_eq!(format!("{}", Error::from(ERR247)), "ERR247");
        assert_eq!(format!("{}", Error::from(ERR248)), "ERR248");
        assert_eq!(format!("{}", Error::from(ERR249)), "ERR249");
        assert_eq!(format!("{}", Error::from(ERR250)), "ERR250");
        assert_eq!(format!("{}", Error::from(ERR251)), "ERR251");
        assert_eq!(format!("{}", Error::from(ERR252)), "ERR252");
        assert_eq!(format!("{}", Error::from(ERR253)), "ERR253");
        assert_eq!(format!("{}", Error::from(ERR254)), "ERR254");
        assert_eq!(format!("{}", Error::from(ERR255)), "ERR255");
    }

}

