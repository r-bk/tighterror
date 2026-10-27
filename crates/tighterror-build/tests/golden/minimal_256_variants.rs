
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
    pub type R = u8;
    pub const KIND_BITS: usize = 8;
    pub const CAT_BITS: usize = 0;
    pub const CAT_MAX: R = 0;
    pub const VAR_MASK: R = 255;
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

    /// General error category.
    pub const GENERAL: C = C::new(0);
}


/// Error kind constants.
pub mod kind {
    use super::ErrorKind as EK;
    use super::category as c;

    /// General category error kind constants.
    pub mod general {
        use super::c;
        use super::EK;
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
        assert_eq!(general::ERR0.name(), "ERR0");
        assert_eq!(tighterror::Kind::name(& general::ERR0), "ERR0");
        assert_eq!(general::ERR1.name(), "ERR1");
        assert_eq!(tighterror::Kind::name(& general::ERR1), "ERR1");
        assert_eq!(general::ERR2.name(), "ERR2");
        assert_eq!(tighterror::Kind::name(& general::ERR2), "ERR2");
        assert_eq!(general::ERR3.name(), "ERR3");
        assert_eq!(tighterror::Kind::name(& general::ERR3), "ERR3");
        assert_eq!(general::ERR4.name(), "ERR4");
        assert_eq!(tighterror::Kind::name(& general::ERR4), "ERR4");
        assert_eq!(general::ERR5.name(), "ERR5");
        assert_eq!(tighterror::Kind::name(& general::ERR5), "ERR5");
        assert_eq!(general::ERR6.name(), "ERR6");
        assert_eq!(tighterror::Kind::name(& general::ERR6), "ERR6");
        assert_eq!(general::ERR7.name(), "ERR7");
        assert_eq!(tighterror::Kind::name(& general::ERR7), "ERR7");
        assert_eq!(general::ERR8.name(), "ERR8");
        assert_eq!(tighterror::Kind::name(& general::ERR8), "ERR8");
        assert_eq!(general::ERR9.name(), "ERR9");
        assert_eq!(tighterror::Kind::name(& general::ERR9), "ERR9");
        assert_eq!(general::ERR10.name(), "ERR10");
        assert_eq!(tighterror::Kind::name(& general::ERR10), "ERR10");
        assert_eq!(general::ERR11.name(), "ERR11");
        assert_eq!(tighterror::Kind::name(& general::ERR11), "ERR11");
        assert_eq!(general::ERR12.name(), "ERR12");
        assert_eq!(tighterror::Kind::name(& general::ERR12), "ERR12");
        assert_eq!(general::ERR13.name(), "ERR13");
        assert_eq!(tighterror::Kind::name(& general::ERR13), "ERR13");
        assert_eq!(general::ERR14.name(), "ERR14");
        assert_eq!(tighterror::Kind::name(& general::ERR14), "ERR14");
        assert_eq!(general::ERR15.name(), "ERR15");
        assert_eq!(tighterror::Kind::name(& general::ERR15), "ERR15");
        assert_eq!(general::ERR16.name(), "ERR16");
        assert_eq!(tighterror::Kind::name(& general::ERR16), "ERR16");
        assert_eq!(general::ERR17.name(), "ERR17");
        assert_eq!(tighterror::Kind::name(& general::ERR17), "ERR17");
        assert_eq!(general::ERR18.name(), "ERR18");
        assert_eq!(tighterror::Kind::name(& general::ERR18), "ERR18");
        assert_eq!(general::ERR19.name(), "ERR19");
        assert_eq!(tighterror::Kind::name(& general::ERR19), "ERR19");
        assert_eq!(general::ERR20.name(), "ERR20");
        assert_eq!(tighterror::Kind::name(& general::ERR20), "ERR20");
        assert_eq!(general::ERR21.name(), "ERR21");
        assert_eq!(tighterror::Kind::name(& general::ERR21), "ERR21");
        assert_eq!(general::ERR22.name(), "ERR22");
        assert_eq!(tighterror::Kind::name(& general::ERR22), "ERR22");
        assert_eq!(general::ERR23.name(), "ERR23");
        assert_eq!(tighterror::Kind::name(& general::ERR23), "ERR23");
        assert_eq!(general::ERR24.name(), "ERR24");
        assert_eq!(tighterror::Kind::name(& general::ERR24), "ERR24");
        assert_eq!(general::ERR25.name(), "ERR25");
        assert_eq!(tighterror::Kind::name(& general::ERR25), "ERR25");
        assert_eq!(general::ERR26.name(), "ERR26");
        assert_eq!(tighterror::Kind::name(& general::ERR26), "ERR26");
        assert_eq!(general::ERR27.name(), "ERR27");
        assert_eq!(tighterror::Kind::name(& general::ERR27), "ERR27");
        assert_eq!(general::ERR28.name(), "ERR28");
        assert_eq!(tighterror::Kind::name(& general::ERR28), "ERR28");
        assert_eq!(general::ERR29.name(), "ERR29");
        assert_eq!(tighterror::Kind::name(& general::ERR29), "ERR29");
        assert_eq!(general::ERR30.name(), "ERR30");
        assert_eq!(tighterror::Kind::name(& general::ERR30), "ERR30");
        assert_eq!(general::ERR31.name(), "ERR31");
        assert_eq!(tighterror::Kind::name(& general::ERR31), "ERR31");
        assert_eq!(general::ERR32.name(), "ERR32");
        assert_eq!(tighterror::Kind::name(& general::ERR32), "ERR32");
        assert_eq!(general::ERR33.name(), "ERR33");
        assert_eq!(tighterror::Kind::name(& general::ERR33), "ERR33");
        assert_eq!(general::ERR34.name(), "ERR34");
        assert_eq!(tighterror::Kind::name(& general::ERR34), "ERR34");
        assert_eq!(general::ERR35.name(), "ERR35");
        assert_eq!(tighterror::Kind::name(& general::ERR35), "ERR35");
        assert_eq!(general::ERR36.name(), "ERR36");
        assert_eq!(tighterror::Kind::name(& general::ERR36), "ERR36");
        assert_eq!(general::ERR37.name(), "ERR37");
        assert_eq!(tighterror::Kind::name(& general::ERR37), "ERR37");
        assert_eq!(general::ERR38.name(), "ERR38");
        assert_eq!(tighterror::Kind::name(& general::ERR38), "ERR38");
        assert_eq!(general::ERR39.name(), "ERR39");
        assert_eq!(tighterror::Kind::name(& general::ERR39), "ERR39");
        assert_eq!(general::ERR40.name(), "ERR40");
        assert_eq!(tighterror::Kind::name(& general::ERR40), "ERR40");
        assert_eq!(general::ERR41.name(), "ERR41");
        assert_eq!(tighterror::Kind::name(& general::ERR41), "ERR41");
        assert_eq!(general::ERR42.name(), "ERR42");
        assert_eq!(tighterror::Kind::name(& general::ERR42), "ERR42");
        assert_eq!(general::ERR43.name(), "ERR43");
        assert_eq!(tighterror::Kind::name(& general::ERR43), "ERR43");
        assert_eq!(general::ERR44.name(), "ERR44");
        assert_eq!(tighterror::Kind::name(& general::ERR44), "ERR44");
        assert_eq!(general::ERR45.name(), "ERR45");
        assert_eq!(tighterror::Kind::name(& general::ERR45), "ERR45");
        assert_eq!(general::ERR46.name(), "ERR46");
        assert_eq!(tighterror::Kind::name(& general::ERR46), "ERR46");
        assert_eq!(general::ERR47.name(), "ERR47");
        assert_eq!(tighterror::Kind::name(& general::ERR47), "ERR47");
        assert_eq!(general::ERR48.name(), "ERR48");
        assert_eq!(tighterror::Kind::name(& general::ERR48), "ERR48");
        assert_eq!(general::ERR49.name(), "ERR49");
        assert_eq!(tighterror::Kind::name(& general::ERR49), "ERR49");
        assert_eq!(general::ERR50.name(), "ERR50");
        assert_eq!(tighterror::Kind::name(& general::ERR50), "ERR50");
        assert_eq!(general::ERR51.name(), "ERR51");
        assert_eq!(tighterror::Kind::name(& general::ERR51), "ERR51");
        assert_eq!(general::ERR52.name(), "ERR52");
        assert_eq!(tighterror::Kind::name(& general::ERR52), "ERR52");
        assert_eq!(general::ERR53.name(), "ERR53");
        assert_eq!(tighterror::Kind::name(& general::ERR53), "ERR53");
        assert_eq!(general::ERR54.name(), "ERR54");
        assert_eq!(tighterror::Kind::name(& general::ERR54), "ERR54");
        assert_eq!(general::ERR55.name(), "ERR55");
        assert_eq!(tighterror::Kind::name(& general::ERR55), "ERR55");
        assert_eq!(general::ERR56.name(), "ERR56");
        assert_eq!(tighterror::Kind::name(& general::ERR56), "ERR56");
        assert_eq!(general::ERR57.name(), "ERR57");
        assert_eq!(tighterror::Kind::name(& general::ERR57), "ERR57");
        assert_eq!(general::ERR58.name(), "ERR58");
        assert_eq!(tighterror::Kind::name(& general::ERR58), "ERR58");
        assert_eq!(general::ERR59.name(), "ERR59");
        assert_eq!(tighterror::Kind::name(& general::ERR59), "ERR59");
        assert_eq!(general::ERR60.name(), "ERR60");
        assert_eq!(tighterror::Kind::name(& general::ERR60), "ERR60");
        assert_eq!(general::ERR61.name(), "ERR61");
        assert_eq!(tighterror::Kind::name(& general::ERR61), "ERR61");
        assert_eq!(general::ERR62.name(), "ERR62");
        assert_eq!(tighterror::Kind::name(& general::ERR62), "ERR62");
        assert_eq!(general::ERR63.name(), "ERR63");
        assert_eq!(tighterror::Kind::name(& general::ERR63), "ERR63");
        assert_eq!(general::ERR64.name(), "ERR64");
        assert_eq!(tighterror::Kind::name(& general::ERR64), "ERR64");
        assert_eq!(general::ERR65.name(), "ERR65");
        assert_eq!(tighterror::Kind::name(& general::ERR65), "ERR65");
        assert_eq!(general::ERR66.name(), "ERR66");
        assert_eq!(tighterror::Kind::name(& general::ERR66), "ERR66");
        assert_eq!(general::ERR67.name(), "ERR67");
        assert_eq!(tighterror::Kind::name(& general::ERR67), "ERR67");
        assert_eq!(general::ERR68.name(), "ERR68");
        assert_eq!(tighterror::Kind::name(& general::ERR68), "ERR68");
        assert_eq!(general::ERR69.name(), "ERR69");
        assert_eq!(tighterror::Kind::name(& general::ERR69), "ERR69");
        assert_eq!(general::ERR70.name(), "ERR70");
        assert_eq!(tighterror::Kind::name(& general::ERR70), "ERR70");
        assert_eq!(general::ERR71.name(), "ERR71");
        assert_eq!(tighterror::Kind::name(& general::ERR71), "ERR71");
        assert_eq!(general::ERR72.name(), "ERR72");
        assert_eq!(tighterror::Kind::name(& general::ERR72), "ERR72");
        assert_eq!(general::ERR73.name(), "ERR73");
        assert_eq!(tighterror::Kind::name(& general::ERR73), "ERR73");
        assert_eq!(general::ERR74.name(), "ERR74");
        assert_eq!(tighterror::Kind::name(& general::ERR74), "ERR74");
        assert_eq!(general::ERR75.name(), "ERR75");
        assert_eq!(tighterror::Kind::name(& general::ERR75), "ERR75");
        assert_eq!(general::ERR76.name(), "ERR76");
        assert_eq!(tighterror::Kind::name(& general::ERR76), "ERR76");
        assert_eq!(general::ERR77.name(), "ERR77");
        assert_eq!(tighterror::Kind::name(& general::ERR77), "ERR77");
        assert_eq!(general::ERR78.name(), "ERR78");
        assert_eq!(tighterror::Kind::name(& general::ERR78), "ERR78");
        assert_eq!(general::ERR79.name(), "ERR79");
        assert_eq!(tighterror::Kind::name(& general::ERR79), "ERR79");
        assert_eq!(general::ERR80.name(), "ERR80");
        assert_eq!(tighterror::Kind::name(& general::ERR80), "ERR80");
        assert_eq!(general::ERR81.name(), "ERR81");
        assert_eq!(tighterror::Kind::name(& general::ERR81), "ERR81");
        assert_eq!(general::ERR82.name(), "ERR82");
        assert_eq!(tighterror::Kind::name(& general::ERR82), "ERR82");
        assert_eq!(general::ERR83.name(), "ERR83");
        assert_eq!(tighterror::Kind::name(& general::ERR83), "ERR83");
        assert_eq!(general::ERR84.name(), "ERR84");
        assert_eq!(tighterror::Kind::name(& general::ERR84), "ERR84");
        assert_eq!(general::ERR85.name(), "ERR85");
        assert_eq!(tighterror::Kind::name(& general::ERR85), "ERR85");
        assert_eq!(general::ERR86.name(), "ERR86");
        assert_eq!(tighterror::Kind::name(& general::ERR86), "ERR86");
        assert_eq!(general::ERR87.name(), "ERR87");
        assert_eq!(tighterror::Kind::name(& general::ERR87), "ERR87");
        assert_eq!(general::ERR88.name(), "ERR88");
        assert_eq!(tighterror::Kind::name(& general::ERR88), "ERR88");
        assert_eq!(general::ERR89.name(), "ERR89");
        assert_eq!(tighterror::Kind::name(& general::ERR89), "ERR89");
        assert_eq!(general::ERR90.name(), "ERR90");
        assert_eq!(tighterror::Kind::name(& general::ERR90), "ERR90");
        assert_eq!(general::ERR91.name(), "ERR91");
        assert_eq!(tighterror::Kind::name(& general::ERR91), "ERR91");
        assert_eq!(general::ERR92.name(), "ERR92");
        assert_eq!(tighterror::Kind::name(& general::ERR92), "ERR92");
        assert_eq!(general::ERR93.name(), "ERR93");
        assert_eq!(tighterror::Kind::name(& general::ERR93), "ERR93");
        assert_eq!(general::ERR94.name(), "ERR94");
        assert_eq!(tighterror::Kind::name(& general::ERR94), "ERR94");
        assert_eq!(general::ERR95.name(), "ERR95");
        assert_eq!(tighterror::Kind::name(& general::ERR95), "ERR95");
        assert_eq!(general::ERR96.name(), "ERR96");
        assert_eq!(tighterror::Kind::name(& general::ERR96), "ERR96");
        assert_eq!(general::ERR97.name(), "ERR97");
        assert_eq!(tighterror::Kind::name(& general::ERR97), "ERR97");
        assert_eq!(general::ERR98.name(), "ERR98");
        assert_eq!(tighterror::Kind::name(& general::ERR98), "ERR98");
        assert_eq!(general::ERR99.name(), "ERR99");
        assert_eq!(tighterror::Kind::name(& general::ERR99), "ERR99");
        assert_eq!(general::ERR100.name(), "ERR100");
        assert_eq!(tighterror::Kind::name(& general::ERR100), "ERR100");
        assert_eq!(general::ERR101.name(), "ERR101");
        assert_eq!(tighterror::Kind::name(& general::ERR101), "ERR101");
        assert_eq!(general::ERR102.name(), "ERR102");
        assert_eq!(tighterror::Kind::name(& general::ERR102), "ERR102");
        assert_eq!(general::ERR103.name(), "ERR103");
        assert_eq!(tighterror::Kind::name(& general::ERR103), "ERR103");
        assert_eq!(general::ERR104.name(), "ERR104");
        assert_eq!(tighterror::Kind::name(& general::ERR104), "ERR104");
        assert_eq!(general::ERR105.name(), "ERR105");
        assert_eq!(tighterror::Kind::name(& general::ERR105), "ERR105");
        assert_eq!(general::ERR106.name(), "ERR106");
        assert_eq!(tighterror::Kind::name(& general::ERR106), "ERR106");
        assert_eq!(general::ERR107.name(), "ERR107");
        assert_eq!(tighterror::Kind::name(& general::ERR107), "ERR107");
        assert_eq!(general::ERR108.name(), "ERR108");
        assert_eq!(tighterror::Kind::name(& general::ERR108), "ERR108");
        assert_eq!(general::ERR109.name(), "ERR109");
        assert_eq!(tighterror::Kind::name(& general::ERR109), "ERR109");
        assert_eq!(general::ERR110.name(), "ERR110");
        assert_eq!(tighterror::Kind::name(& general::ERR110), "ERR110");
        assert_eq!(general::ERR111.name(), "ERR111");
        assert_eq!(tighterror::Kind::name(& general::ERR111), "ERR111");
        assert_eq!(general::ERR112.name(), "ERR112");
        assert_eq!(tighterror::Kind::name(& general::ERR112), "ERR112");
        assert_eq!(general::ERR113.name(), "ERR113");
        assert_eq!(tighterror::Kind::name(& general::ERR113), "ERR113");
        assert_eq!(general::ERR114.name(), "ERR114");
        assert_eq!(tighterror::Kind::name(& general::ERR114), "ERR114");
        assert_eq!(general::ERR115.name(), "ERR115");
        assert_eq!(tighterror::Kind::name(& general::ERR115), "ERR115");
        assert_eq!(general::ERR116.name(), "ERR116");
        assert_eq!(tighterror::Kind::name(& general::ERR116), "ERR116");
        assert_eq!(general::ERR117.name(), "ERR117");
        assert_eq!(tighterror::Kind::name(& general::ERR117), "ERR117");
        assert_eq!(general::ERR118.name(), "ERR118");
        assert_eq!(tighterror::Kind::name(& general::ERR118), "ERR118");
        assert_eq!(general::ERR119.name(), "ERR119");
        assert_eq!(tighterror::Kind::name(& general::ERR119), "ERR119");
        assert_eq!(general::ERR120.name(), "ERR120");
        assert_eq!(tighterror::Kind::name(& general::ERR120), "ERR120");
        assert_eq!(general::ERR121.name(), "ERR121");
        assert_eq!(tighterror::Kind::name(& general::ERR121), "ERR121");
        assert_eq!(general::ERR122.name(), "ERR122");
        assert_eq!(tighterror::Kind::name(& general::ERR122), "ERR122");
        assert_eq!(general::ERR123.name(), "ERR123");
        assert_eq!(tighterror::Kind::name(& general::ERR123), "ERR123");
        assert_eq!(general::ERR124.name(), "ERR124");
        assert_eq!(tighterror::Kind::name(& general::ERR124), "ERR124");
        assert_eq!(general::ERR125.name(), "ERR125");
        assert_eq!(tighterror::Kind::name(& general::ERR125), "ERR125");
        assert_eq!(general::ERR126.name(), "ERR126");
        assert_eq!(tighterror::Kind::name(& general::ERR126), "ERR126");
        assert_eq!(general::ERR127.name(), "ERR127");
        assert_eq!(tighterror::Kind::name(& general::ERR127), "ERR127");
        assert_eq!(general::ERR128.name(), "ERR128");
        assert_eq!(tighterror::Kind::name(& general::ERR128), "ERR128");
        assert_eq!(general::ERR129.name(), "ERR129");
        assert_eq!(tighterror::Kind::name(& general::ERR129), "ERR129");
        assert_eq!(general::ERR130.name(), "ERR130");
        assert_eq!(tighterror::Kind::name(& general::ERR130), "ERR130");
        assert_eq!(general::ERR131.name(), "ERR131");
        assert_eq!(tighterror::Kind::name(& general::ERR131), "ERR131");
        assert_eq!(general::ERR132.name(), "ERR132");
        assert_eq!(tighterror::Kind::name(& general::ERR132), "ERR132");
        assert_eq!(general::ERR133.name(), "ERR133");
        assert_eq!(tighterror::Kind::name(& general::ERR133), "ERR133");
        assert_eq!(general::ERR134.name(), "ERR134");
        assert_eq!(tighterror::Kind::name(& general::ERR134), "ERR134");
        assert_eq!(general::ERR135.name(), "ERR135");
        assert_eq!(tighterror::Kind::name(& general::ERR135), "ERR135");
        assert_eq!(general::ERR136.name(), "ERR136");
        assert_eq!(tighterror::Kind::name(& general::ERR136), "ERR136");
        assert_eq!(general::ERR137.name(), "ERR137");
        assert_eq!(tighterror::Kind::name(& general::ERR137), "ERR137");
        assert_eq!(general::ERR138.name(), "ERR138");
        assert_eq!(tighterror::Kind::name(& general::ERR138), "ERR138");
        assert_eq!(general::ERR139.name(), "ERR139");
        assert_eq!(tighterror::Kind::name(& general::ERR139), "ERR139");
        assert_eq!(general::ERR140.name(), "ERR140");
        assert_eq!(tighterror::Kind::name(& general::ERR140), "ERR140");
        assert_eq!(general::ERR141.name(), "ERR141");
        assert_eq!(tighterror::Kind::name(& general::ERR141), "ERR141");
        assert_eq!(general::ERR142.name(), "ERR142");
        assert_eq!(tighterror::Kind::name(& general::ERR142), "ERR142");
        assert_eq!(general::ERR143.name(), "ERR143");
        assert_eq!(tighterror::Kind::name(& general::ERR143), "ERR143");
        assert_eq!(general::ERR144.name(), "ERR144");
        assert_eq!(tighterror::Kind::name(& general::ERR144), "ERR144");
        assert_eq!(general::ERR145.name(), "ERR145");
        assert_eq!(tighterror::Kind::name(& general::ERR145), "ERR145");
        assert_eq!(general::ERR146.name(), "ERR146");
        assert_eq!(tighterror::Kind::name(& general::ERR146), "ERR146");
        assert_eq!(general::ERR147.name(), "ERR147");
        assert_eq!(tighterror::Kind::name(& general::ERR147), "ERR147");
        assert_eq!(general::ERR148.name(), "ERR148");
        assert_eq!(tighterror::Kind::name(& general::ERR148), "ERR148");
        assert_eq!(general::ERR149.name(), "ERR149");
        assert_eq!(tighterror::Kind::name(& general::ERR149), "ERR149");
        assert_eq!(general::ERR150.name(), "ERR150");
        assert_eq!(tighterror::Kind::name(& general::ERR150), "ERR150");
        assert_eq!(general::ERR151.name(), "ERR151");
        assert_eq!(tighterror::Kind::name(& general::ERR151), "ERR151");
        assert_eq!(general::ERR152.name(), "ERR152");
        assert_eq!(tighterror::Kind::name(& general::ERR152), "ERR152");
        assert_eq!(general::ERR153.name(), "ERR153");
        assert_eq!(tighterror::Kind::name(& general::ERR153), "ERR153");
        assert_eq!(general::ERR154.name(), "ERR154");
        assert_eq!(tighterror::Kind::name(& general::ERR154), "ERR154");
        assert_eq!(general::ERR155.name(), "ERR155");
        assert_eq!(tighterror::Kind::name(& general::ERR155), "ERR155");
        assert_eq!(general::ERR156.name(), "ERR156");
        assert_eq!(tighterror::Kind::name(& general::ERR156), "ERR156");
        assert_eq!(general::ERR157.name(), "ERR157");
        assert_eq!(tighterror::Kind::name(& general::ERR157), "ERR157");
        assert_eq!(general::ERR158.name(), "ERR158");
        assert_eq!(tighterror::Kind::name(& general::ERR158), "ERR158");
        assert_eq!(general::ERR159.name(), "ERR159");
        assert_eq!(tighterror::Kind::name(& general::ERR159), "ERR159");
        assert_eq!(general::ERR160.name(), "ERR160");
        assert_eq!(tighterror::Kind::name(& general::ERR160), "ERR160");
        assert_eq!(general::ERR161.name(), "ERR161");
        assert_eq!(tighterror::Kind::name(& general::ERR161), "ERR161");
        assert_eq!(general::ERR162.name(), "ERR162");
        assert_eq!(tighterror::Kind::name(& general::ERR162), "ERR162");
        assert_eq!(general::ERR163.name(), "ERR163");
        assert_eq!(tighterror::Kind::name(& general::ERR163), "ERR163");
        assert_eq!(general::ERR164.name(), "ERR164");
        assert_eq!(tighterror::Kind::name(& general::ERR164), "ERR164");
        assert_eq!(general::ERR165.name(), "ERR165");
        assert_eq!(tighterror::Kind::name(& general::ERR165), "ERR165");
        assert_eq!(general::ERR166.name(), "ERR166");
        assert_eq!(tighterror::Kind::name(& general::ERR166), "ERR166");
        assert_eq!(general::ERR167.name(), "ERR167");
        assert_eq!(tighterror::Kind::name(& general::ERR167), "ERR167");
        assert_eq!(general::ERR168.name(), "ERR168");
        assert_eq!(tighterror::Kind::name(& general::ERR168), "ERR168");
        assert_eq!(general::ERR169.name(), "ERR169");
        assert_eq!(tighterror::Kind::name(& general::ERR169), "ERR169");
        assert_eq!(general::ERR170.name(), "ERR170");
        assert_eq!(tighterror::Kind::name(& general::ERR170), "ERR170");
        assert_eq!(general::ERR171.name(), "ERR171");
        assert_eq!(tighterror::Kind::name(& general::ERR171), "ERR171");
        assert_eq!(general::ERR172.name(), "ERR172");
        assert_eq!(tighterror::Kind::name(& general::ERR172), "ERR172");
        assert_eq!(general::ERR173.name(), "ERR173");
        assert_eq!(tighterror::Kind::name(& general::ERR173), "ERR173");
        assert_eq!(general::ERR174.name(), "ERR174");
        assert_eq!(tighterror::Kind::name(& general::ERR174), "ERR174");
        assert_eq!(general::ERR175.name(), "ERR175");
        assert_eq!(tighterror::Kind::name(& general::ERR175), "ERR175");
        assert_eq!(general::ERR176.name(), "ERR176");
        assert_eq!(tighterror::Kind::name(& general::ERR176), "ERR176");
        assert_eq!(general::ERR177.name(), "ERR177");
        assert_eq!(tighterror::Kind::name(& general::ERR177), "ERR177");
        assert_eq!(general::ERR178.name(), "ERR178");
        assert_eq!(tighterror::Kind::name(& general::ERR178), "ERR178");
        assert_eq!(general::ERR179.name(), "ERR179");
        assert_eq!(tighterror::Kind::name(& general::ERR179), "ERR179");
        assert_eq!(general::ERR180.name(), "ERR180");
        assert_eq!(tighterror::Kind::name(& general::ERR180), "ERR180");
        assert_eq!(general::ERR181.name(), "ERR181");
        assert_eq!(tighterror::Kind::name(& general::ERR181), "ERR181");
        assert_eq!(general::ERR182.name(), "ERR182");
        assert_eq!(tighterror::Kind::name(& general::ERR182), "ERR182");
        assert_eq!(general::ERR183.name(), "ERR183");
        assert_eq!(tighterror::Kind::name(& general::ERR183), "ERR183");
        assert_eq!(general::ERR184.name(), "ERR184");
        assert_eq!(tighterror::Kind::name(& general::ERR184), "ERR184");
        assert_eq!(general::ERR185.name(), "ERR185");
        assert_eq!(tighterror::Kind::name(& general::ERR185), "ERR185");
        assert_eq!(general::ERR186.name(), "ERR186");
        assert_eq!(tighterror::Kind::name(& general::ERR186), "ERR186");
        assert_eq!(general::ERR187.name(), "ERR187");
        assert_eq!(tighterror::Kind::name(& general::ERR187), "ERR187");
        assert_eq!(general::ERR188.name(), "ERR188");
        assert_eq!(tighterror::Kind::name(& general::ERR188), "ERR188");
        assert_eq!(general::ERR189.name(), "ERR189");
        assert_eq!(tighterror::Kind::name(& general::ERR189), "ERR189");
        assert_eq!(general::ERR190.name(), "ERR190");
        assert_eq!(tighterror::Kind::name(& general::ERR190), "ERR190");
        assert_eq!(general::ERR191.name(), "ERR191");
        assert_eq!(tighterror::Kind::name(& general::ERR191), "ERR191");
        assert_eq!(general::ERR192.name(), "ERR192");
        assert_eq!(tighterror::Kind::name(& general::ERR192), "ERR192");
        assert_eq!(general::ERR193.name(), "ERR193");
        assert_eq!(tighterror::Kind::name(& general::ERR193), "ERR193");
        assert_eq!(general::ERR194.name(), "ERR194");
        assert_eq!(tighterror::Kind::name(& general::ERR194), "ERR194");
        assert_eq!(general::ERR195.name(), "ERR195");
        assert_eq!(tighterror::Kind::name(& general::ERR195), "ERR195");
        assert_eq!(general::ERR196.name(), "ERR196");
        assert_eq!(tighterror::Kind::name(& general::ERR196), "ERR196");
        assert_eq!(general::ERR197.name(), "ERR197");
        assert_eq!(tighterror::Kind::name(& general::ERR197), "ERR197");
        assert_eq!(general::ERR198.name(), "ERR198");
        assert_eq!(tighterror::Kind::name(& general::ERR198), "ERR198");
        assert_eq!(general::ERR199.name(), "ERR199");
        assert_eq!(tighterror::Kind::name(& general::ERR199), "ERR199");
        assert_eq!(general::ERR200.name(), "ERR200");
        assert_eq!(tighterror::Kind::name(& general::ERR200), "ERR200");
        assert_eq!(general::ERR201.name(), "ERR201");
        assert_eq!(tighterror::Kind::name(& general::ERR201), "ERR201");
        assert_eq!(general::ERR202.name(), "ERR202");
        assert_eq!(tighterror::Kind::name(& general::ERR202), "ERR202");
        assert_eq!(general::ERR203.name(), "ERR203");
        assert_eq!(tighterror::Kind::name(& general::ERR203), "ERR203");
        assert_eq!(general::ERR204.name(), "ERR204");
        assert_eq!(tighterror::Kind::name(& general::ERR204), "ERR204");
        assert_eq!(general::ERR205.name(), "ERR205");
        assert_eq!(tighterror::Kind::name(& general::ERR205), "ERR205");
        assert_eq!(general::ERR206.name(), "ERR206");
        assert_eq!(tighterror::Kind::name(& general::ERR206), "ERR206");
        assert_eq!(general::ERR207.name(), "ERR207");
        assert_eq!(tighterror::Kind::name(& general::ERR207), "ERR207");
        assert_eq!(general::ERR208.name(), "ERR208");
        assert_eq!(tighterror::Kind::name(& general::ERR208), "ERR208");
        assert_eq!(general::ERR209.name(), "ERR209");
        assert_eq!(tighterror::Kind::name(& general::ERR209), "ERR209");
        assert_eq!(general::ERR210.name(), "ERR210");
        assert_eq!(tighterror::Kind::name(& general::ERR210), "ERR210");
        assert_eq!(general::ERR211.name(), "ERR211");
        assert_eq!(tighterror::Kind::name(& general::ERR211), "ERR211");
        assert_eq!(general::ERR212.name(), "ERR212");
        assert_eq!(tighterror::Kind::name(& general::ERR212), "ERR212");
        assert_eq!(general::ERR213.name(), "ERR213");
        assert_eq!(tighterror::Kind::name(& general::ERR213), "ERR213");
        assert_eq!(general::ERR214.name(), "ERR214");
        assert_eq!(tighterror::Kind::name(& general::ERR214), "ERR214");
        assert_eq!(general::ERR215.name(), "ERR215");
        assert_eq!(tighterror::Kind::name(& general::ERR215), "ERR215");
        assert_eq!(general::ERR216.name(), "ERR216");
        assert_eq!(tighterror::Kind::name(& general::ERR216), "ERR216");
        assert_eq!(general::ERR217.name(), "ERR217");
        assert_eq!(tighterror::Kind::name(& general::ERR217), "ERR217");
        assert_eq!(general::ERR218.name(), "ERR218");
        assert_eq!(tighterror::Kind::name(& general::ERR218), "ERR218");
        assert_eq!(general::ERR219.name(), "ERR219");
        assert_eq!(tighterror::Kind::name(& general::ERR219), "ERR219");
        assert_eq!(general::ERR220.name(), "ERR220");
        assert_eq!(tighterror::Kind::name(& general::ERR220), "ERR220");
        assert_eq!(general::ERR221.name(), "ERR221");
        assert_eq!(tighterror::Kind::name(& general::ERR221), "ERR221");
        assert_eq!(general::ERR222.name(), "ERR222");
        assert_eq!(tighterror::Kind::name(& general::ERR222), "ERR222");
        assert_eq!(general::ERR223.name(), "ERR223");
        assert_eq!(tighterror::Kind::name(& general::ERR223), "ERR223");
        assert_eq!(general::ERR224.name(), "ERR224");
        assert_eq!(tighterror::Kind::name(& general::ERR224), "ERR224");
        assert_eq!(general::ERR225.name(), "ERR225");
        assert_eq!(tighterror::Kind::name(& general::ERR225), "ERR225");
        assert_eq!(general::ERR226.name(), "ERR226");
        assert_eq!(tighterror::Kind::name(& general::ERR226), "ERR226");
        assert_eq!(general::ERR227.name(), "ERR227");
        assert_eq!(tighterror::Kind::name(& general::ERR227), "ERR227");
        assert_eq!(general::ERR228.name(), "ERR228");
        assert_eq!(tighterror::Kind::name(& general::ERR228), "ERR228");
        assert_eq!(general::ERR229.name(), "ERR229");
        assert_eq!(tighterror::Kind::name(& general::ERR229), "ERR229");
        assert_eq!(general::ERR230.name(), "ERR230");
        assert_eq!(tighterror::Kind::name(& general::ERR230), "ERR230");
        assert_eq!(general::ERR231.name(), "ERR231");
        assert_eq!(tighterror::Kind::name(& general::ERR231), "ERR231");
        assert_eq!(general::ERR232.name(), "ERR232");
        assert_eq!(tighterror::Kind::name(& general::ERR232), "ERR232");
        assert_eq!(general::ERR233.name(), "ERR233");
        assert_eq!(tighterror::Kind::name(& general::ERR233), "ERR233");
        assert_eq!(general::ERR234.name(), "ERR234");
        assert_eq!(tighterror::Kind::name(& general::ERR234), "ERR234");
        assert_eq!(general::ERR235.name(), "ERR235");
        assert_eq!(tighterror::Kind::name(& general::ERR235), "ERR235");
        assert_eq!(general::ERR236.name(), "ERR236");
        assert_eq!(tighterror::Kind::name(& general::ERR236), "ERR236");
        assert_eq!(general::ERR237.name(), "ERR237");
        assert_eq!(tighterror::Kind::name(& general::ERR237), "ERR237");
        assert_eq!(general::ERR238.name(), "ERR238");
        assert_eq!(tighterror::Kind::name(& general::ERR238), "ERR238");
        assert_eq!(general::ERR239.name(), "ERR239");
        assert_eq!(tighterror::Kind::name(& general::ERR239), "ERR239");
        assert_eq!(general::ERR240.name(), "ERR240");
        assert_eq!(tighterror::Kind::name(& general::ERR240), "ERR240");
        assert_eq!(general::ERR241.name(), "ERR241");
        assert_eq!(tighterror::Kind::name(& general::ERR241), "ERR241");
        assert_eq!(general::ERR242.name(), "ERR242");
        assert_eq!(tighterror::Kind::name(& general::ERR242), "ERR242");
        assert_eq!(general::ERR243.name(), "ERR243");
        assert_eq!(tighterror::Kind::name(& general::ERR243), "ERR243");
        assert_eq!(general::ERR244.name(), "ERR244");
        assert_eq!(tighterror::Kind::name(& general::ERR244), "ERR244");
        assert_eq!(general::ERR245.name(), "ERR245");
        assert_eq!(tighterror::Kind::name(& general::ERR245), "ERR245");
        assert_eq!(general::ERR246.name(), "ERR246");
        assert_eq!(tighterror::Kind::name(& general::ERR246), "ERR246");
        assert_eq!(general::ERR247.name(), "ERR247");
        assert_eq!(tighterror::Kind::name(& general::ERR247), "ERR247");
        assert_eq!(general::ERR248.name(), "ERR248");
        assert_eq!(tighterror::Kind::name(& general::ERR248), "ERR248");
        assert_eq!(general::ERR249.name(), "ERR249");
        assert_eq!(tighterror::Kind::name(& general::ERR249), "ERR249");
        assert_eq!(general::ERR250.name(), "ERR250");
        assert_eq!(tighterror::Kind::name(& general::ERR250), "ERR250");
        assert_eq!(general::ERR251.name(), "ERR251");
        assert_eq!(tighterror::Kind::name(& general::ERR251), "ERR251");
        assert_eq!(general::ERR252.name(), "ERR252");
        assert_eq!(tighterror::Kind::name(& general::ERR252), "ERR252");
        assert_eq!(general::ERR253.name(), "ERR253");
        assert_eq!(tighterror::Kind::name(& general::ERR253), "ERR253");
        assert_eq!(general::ERR254.name(), "ERR254");
        assert_eq!(tighterror::Kind::name(& general::ERR254), "ERR254");
        assert_eq!(general::ERR255.name(), "ERR255");
        assert_eq!(tighterror::Kind::name(& general::ERR255), "ERR255");
    }


    #[test]
    fn test_err_kind_display() {
        use kind::*;
        assert_eq!(format!("{}", general::ERR0), "ERR0");
        assert_eq!(format!("{}", general::ERR1), "ERR1");
        assert_eq!(format!("{}", general::ERR2), "ERR2");
        assert_eq!(format!("{}", general::ERR3), "ERR3");
        assert_eq!(format!("{}", general::ERR4), "ERR4");
        assert_eq!(format!("{}", general::ERR5), "ERR5");
        assert_eq!(format!("{}", general::ERR6), "ERR6");
        assert_eq!(format!("{}", general::ERR7), "ERR7");
        assert_eq!(format!("{}", general::ERR8), "ERR8");
        assert_eq!(format!("{}", general::ERR9), "ERR9");
        assert_eq!(format!("{}", general::ERR10), "ERR10");
        assert_eq!(format!("{}", general::ERR11), "ERR11");
        assert_eq!(format!("{}", general::ERR12), "ERR12");
        assert_eq!(format!("{}", general::ERR13), "ERR13");
        assert_eq!(format!("{}", general::ERR14), "ERR14");
        assert_eq!(format!("{}", general::ERR15), "ERR15");
        assert_eq!(format!("{}", general::ERR16), "ERR16");
        assert_eq!(format!("{}", general::ERR17), "ERR17");
        assert_eq!(format!("{}", general::ERR18), "ERR18");
        assert_eq!(format!("{}", general::ERR19), "ERR19");
        assert_eq!(format!("{}", general::ERR20), "ERR20");
        assert_eq!(format!("{}", general::ERR21), "ERR21");
        assert_eq!(format!("{}", general::ERR22), "ERR22");
        assert_eq!(format!("{}", general::ERR23), "ERR23");
        assert_eq!(format!("{}", general::ERR24), "ERR24");
        assert_eq!(format!("{}", general::ERR25), "ERR25");
        assert_eq!(format!("{}", general::ERR26), "ERR26");
        assert_eq!(format!("{}", general::ERR27), "ERR27");
        assert_eq!(format!("{}", general::ERR28), "ERR28");
        assert_eq!(format!("{}", general::ERR29), "ERR29");
        assert_eq!(format!("{}", general::ERR30), "ERR30");
        assert_eq!(format!("{}", general::ERR31), "ERR31");
        assert_eq!(format!("{}", general::ERR32), "ERR32");
        assert_eq!(format!("{}", general::ERR33), "ERR33");
        assert_eq!(format!("{}", general::ERR34), "ERR34");
        assert_eq!(format!("{}", general::ERR35), "ERR35");
        assert_eq!(format!("{}", general::ERR36), "ERR36");
        assert_eq!(format!("{}", general::ERR37), "ERR37");
        assert_eq!(format!("{}", general::ERR38), "ERR38");
        assert_eq!(format!("{}", general::ERR39), "ERR39");
        assert_eq!(format!("{}", general::ERR40), "ERR40");
        assert_eq!(format!("{}", general::ERR41), "ERR41");
        assert_eq!(format!("{}", general::ERR42), "ERR42");
        assert_eq!(format!("{}", general::ERR43), "ERR43");
        assert_eq!(format!("{}", general::ERR44), "ERR44");
        assert_eq!(format!("{}", general::ERR45), "ERR45");
        assert_eq!(format!("{}", general::ERR46), "ERR46");
        assert_eq!(format!("{}", general::ERR47), "ERR47");
        assert_eq!(format!("{}", general::ERR48), "ERR48");
        assert_eq!(format!("{}", general::ERR49), "ERR49");
        assert_eq!(format!("{}", general::ERR50), "ERR50");
        assert_eq!(format!("{}", general::ERR51), "ERR51");
        assert_eq!(format!("{}", general::ERR52), "ERR52");
        assert_eq!(format!("{}", general::ERR53), "ERR53");
        assert_eq!(format!("{}", general::ERR54), "ERR54");
        assert_eq!(format!("{}", general::ERR55), "ERR55");
        assert_eq!(format!("{}", general::ERR56), "ERR56");
        assert_eq!(format!("{}", general::ERR57), "ERR57");
        assert_eq!(format!("{}", general::ERR58), "ERR58");
        assert_eq!(format!("{}", general::ERR59), "ERR59");
        assert_eq!(format!("{}", general::ERR60), "ERR60");
        assert_eq!(format!("{}", general::ERR61), "ERR61");
        assert_eq!(format!("{}", general::ERR62), "ERR62");
        assert_eq!(format!("{}", general::ERR63), "ERR63");
        assert_eq!(format!("{}", general::ERR64), "ERR64");
        assert_eq!(format!("{}", general::ERR65), "ERR65");
        assert_eq!(format!("{}", general::ERR66), "ERR66");
        assert_eq!(format!("{}", general::ERR67), "ERR67");
        assert_eq!(format!("{}", general::ERR68), "ERR68");
        assert_eq!(format!("{}", general::ERR69), "ERR69");
        assert_eq!(format!("{}", general::ERR70), "ERR70");
        assert_eq!(format!("{}", general::ERR71), "ERR71");
        assert_eq!(format!("{}", general::ERR72), "ERR72");
        assert_eq!(format!("{}", general::ERR73), "ERR73");
        assert_eq!(format!("{}", general::ERR74), "ERR74");
        assert_eq!(format!("{}", general::ERR75), "ERR75");
        assert_eq!(format!("{}", general::ERR76), "ERR76");
        assert_eq!(format!("{}", general::ERR77), "ERR77");
        assert_eq!(format!("{}", general::ERR78), "ERR78");
        assert_eq!(format!("{}", general::ERR79), "ERR79");
        assert_eq!(format!("{}", general::ERR80), "ERR80");
        assert_eq!(format!("{}", general::ERR81), "ERR81");
        assert_eq!(format!("{}", general::ERR82), "ERR82");
        assert_eq!(format!("{}", general::ERR83), "ERR83");
        assert_eq!(format!("{}", general::ERR84), "ERR84");
        assert_eq!(format!("{}", general::ERR85), "ERR85");
        assert_eq!(format!("{}", general::ERR86), "ERR86");
        assert_eq!(format!("{}", general::ERR87), "ERR87");
        assert_eq!(format!("{}", general::ERR88), "ERR88");
        assert_eq!(format!("{}", general::ERR89), "ERR89");
        assert_eq!(format!("{}", general::ERR90), "ERR90");
        assert_eq!(format!("{}", general::ERR91), "ERR91");
        assert_eq!(format!("{}", general::ERR92), "ERR92");
        assert_eq!(format!("{}", general::ERR93), "ERR93");
        assert_eq!(format!("{}", general::ERR94), "ERR94");
        assert_eq!(format!("{}", general::ERR95), "ERR95");
        assert_eq!(format!("{}", general::ERR96), "ERR96");
        assert_eq!(format!("{}", general::ERR97), "ERR97");
        assert_eq!(format!("{}", general::ERR98), "ERR98");
        assert_eq!(format!("{}", general::ERR99), "ERR99");
        assert_eq!(format!("{}", general::ERR100), "ERR100");
        assert_eq!(format!("{}", general::ERR101), "ERR101");
        assert_eq!(format!("{}", general::ERR102), "ERR102");
        assert_eq!(format!("{}", general::ERR103), "ERR103");
        assert_eq!(format!("{}", general::ERR104), "ERR104");
        assert_eq!(format!("{}", general::ERR105), "ERR105");
        assert_eq!(format!("{}", general::ERR106), "ERR106");
        assert_eq!(format!("{}", general::ERR107), "ERR107");
        assert_eq!(format!("{}", general::ERR108), "ERR108");
        assert_eq!(format!("{}", general::ERR109), "ERR109");
        assert_eq!(format!("{}", general::ERR110), "ERR110");
        assert_eq!(format!("{}", general::ERR111), "ERR111");
        assert_eq!(format!("{}", general::ERR112), "ERR112");
        assert_eq!(format!("{}", general::ERR113), "ERR113");
        assert_eq!(format!("{}", general::ERR114), "ERR114");
        assert_eq!(format!("{}", general::ERR115), "ERR115");
        assert_eq!(format!("{}", general::ERR116), "ERR116");
        assert_eq!(format!("{}", general::ERR117), "ERR117");
        assert_eq!(format!("{}", general::ERR118), "ERR118");
        assert_eq!(format!("{}", general::ERR119), "ERR119");
        assert_eq!(format!("{}", general::ERR120), "ERR120");
        assert_eq!(format!("{}", general::ERR121), "ERR121");
        assert_eq!(format!("{}", general::ERR122), "ERR122");
        assert_eq!(format!("{}", general::ERR123), "ERR123");
        assert_eq!(format!("{}", general::ERR124), "ERR124");
        assert_eq!(format!("{}", general::ERR125), "ERR125");
        assert_eq!(format!("{}", general::ERR126), "ERR126");
        assert_eq!(format!("{}", general::ERR127), "ERR127");
        assert_eq!(format!("{}", general::ERR128), "ERR128");
        assert_eq!(format!("{}", general::ERR129), "ERR129");
        assert_eq!(format!("{}", general::ERR130), "ERR130");
        assert_eq!(format!("{}", general::ERR131), "ERR131");
        assert_eq!(format!("{}", general::ERR132), "ERR132");
        assert_eq!(format!("{}", general::ERR133), "ERR133");
        assert_eq!(format!("{}", general::ERR134), "ERR134");
        assert_eq!(format!("{}", general::ERR135), "ERR135");
        assert_eq!(format!("{}", general::ERR136), "ERR136");
        assert_eq!(format!("{}", general::ERR137), "ERR137");
        assert_eq!(format!("{}", general::ERR138), "ERR138");
        assert_eq!(format!("{}", general::ERR139), "ERR139");
        assert_eq!(format!("{}", general::ERR140), "ERR140");
        assert_eq!(format!("{}", general::ERR141), "ERR141");
        assert_eq!(format!("{}", general::ERR142), "ERR142");
        assert_eq!(format!("{}", general::ERR143), "ERR143");
        assert_eq!(format!("{}", general::ERR144), "ERR144");
        assert_eq!(format!("{}", general::ERR145), "ERR145");
        assert_eq!(format!("{}", general::ERR146), "ERR146");
        assert_eq!(format!("{}", general::ERR147), "ERR147");
        assert_eq!(format!("{}", general::ERR148), "ERR148");
        assert_eq!(format!("{}", general::ERR149), "ERR149");
        assert_eq!(format!("{}", general::ERR150), "ERR150");
        assert_eq!(format!("{}", general::ERR151), "ERR151");
        assert_eq!(format!("{}", general::ERR152), "ERR152");
        assert_eq!(format!("{}", general::ERR153), "ERR153");
        assert_eq!(format!("{}", general::ERR154), "ERR154");
        assert_eq!(format!("{}", general::ERR155), "ERR155");
        assert_eq!(format!("{}", general::ERR156), "ERR156");
        assert_eq!(format!("{}", general::ERR157), "ERR157");
        assert_eq!(format!("{}", general::ERR158), "ERR158");
        assert_eq!(format!("{}", general::ERR159), "ERR159");
        assert_eq!(format!("{}", general::ERR160), "ERR160");
        assert_eq!(format!("{}", general::ERR161), "ERR161");
        assert_eq!(format!("{}", general::ERR162), "ERR162");
        assert_eq!(format!("{}", general::ERR163), "ERR163");
        assert_eq!(format!("{}", general::ERR164), "ERR164");
        assert_eq!(format!("{}", general::ERR165), "ERR165");
        assert_eq!(format!("{}", general::ERR166), "ERR166");
        assert_eq!(format!("{}", general::ERR167), "ERR167");
        assert_eq!(format!("{}", general::ERR168), "ERR168");
        assert_eq!(format!("{}", general::ERR169), "ERR169");
        assert_eq!(format!("{}", general::ERR170), "ERR170");
        assert_eq!(format!("{}", general::ERR171), "ERR171");
        assert_eq!(format!("{}", general::ERR172), "ERR172");
        assert_eq!(format!("{}", general::ERR173), "ERR173");
        assert_eq!(format!("{}", general::ERR174), "ERR174");
        assert_eq!(format!("{}", general::ERR175), "ERR175");
        assert_eq!(format!("{}", general::ERR176), "ERR176");
        assert_eq!(format!("{}", general::ERR177), "ERR177");
        assert_eq!(format!("{}", general::ERR178), "ERR178");
        assert_eq!(format!("{}", general::ERR179), "ERR179");
        assert_eq!(format!("{}", general::ERR180), "ERR180");
        assert_eq!(format!("{}", general::ERR181), "ERR181");
        assert_eq!(format!("{}", general::ERR182), "ERR182");
        assert_eq!(format!("{}", general::ERR183), "ERR183");
        assert_eq!(format!("{}", general::ERR184), "ERR184");
        assert_eq!(format!("{}", general::ERR185), "ERR185");
        assert_eq!(format!("{}", general::ERR186), "ERR186");
        assert_eq!(format!("{}", general::ERR187), "ERR187");
        assert_eq!(format!("{}", general::ERR188), "ERR188");
        assert_eq!(format!("{}", general::ERR189), "ERR189");
        assert_eq!(format!("{}", general::ERR190), "ERR190");
        assert_eq!(format!("{}", general::ERR191), "ERR191");
        assert_eq!(format!("{}", general::ERR192), "ERR192");
        assert_eq!(format!("{}", general::ERR193), "ERR193");
        assert_eq!(format!("{}", general::ERR194), "ERR194");
        assert_eq!(format!("{}", general::ERR195), "ERR195");
        assert_eq!(format!("{}", general::ERR196), "ERR196");
        assert_eq!(format!("{}", general::ERR197), "ERR197");
        assert_eq!(format!("{}", general::ERR198), "ERR198");
        assert_eq!(format!("{}", general::ERR199), "ERR199");
        assert_eq!(format!("{}", general::ERR200), "ERR200");
        assert_eq!(format!("{}", general::ERR201), "ERR201");
        assert_eq!(format!("{}", general::ERR202), "ERR202");
        assert_eq!(format!("{}", general::ERR203), "ERR203");
        assert_eq!(format!("{}", general::ERR204), "ERR204");
        assert_eq!(format!("{}", general::ERR205), "ERR205");
        assert_eq!(format!("{}", general::ERR206), "ERR206");
        assert_eq!(format!("{}", general::ERR207), "ERR207");
        assert_eq!(format!("{}", general::ERR208), "ERR208");
        assert_eq!(format!("{}", general::ERR209), "ERR209");
        assert_eq!(format!("{}", general::ERR210), "ERR210");
        assert_eq!(format!("{}", general::ERR211), "ERR211");
        assert_eq!(format!("{}", general::ERR212), "ERR212");
        assert_eq!(format!("{}", general::ERR213), "ERR213");
        assert_eq!(format!("{}", general::ERR214), "ERR214");
        assert_eq!(format!("{}", general::ERR215), "ERR215");
        assert_eq!(format!("{}", general::ERR216), "ERR216");
        assert_eq!(format!("{}", general::ERR217), "ERR217");
        assert_eq!(format!("{}", general::ERR218), "ERR218");
        assert_eq!(format!("{}", general::ERR219), "ERR219");
        assert_eq!(format!("{}", general::ERR220), "ERR220");
        assert_eq!(format!("{}", general::ERR221), "ERR221");
        assert_eq!(format!("{}", general::ERR222), "ERR222");
        assert_eq!(format!("{}", general::ERR223), "ERR223");
        assert_eq!(format!("{}", general::ERR224), "ERR224");
        assert_eq!(format!("{}", general::ERR225), "ERR225");
        assert_eq!(format!("{}", general::ERR226), "ERR226");
        assert_eq!(format!("{}", general::ERR227), "ERR227");
        assert_eq!(format!("{}", general::ERR228), "ERR228");
        assert_eq!(format!("{}", general::ERR229), "ERR229");
        assert_eq!(format!("{}", general::ERR230), "ERR230");
        assert_eq!(format!("{}", general::ERR231), "ERR231");
        assert_eq!(format!("{}", general::ERR232), "ERR232");
        assert_eq!(format!("{}", general::ERR233), "ERR233");
        assert_eq!(format!("{}", general::ERR234), "ERR234");
        assert_eq!(format!("{}", general::ERR235), "ERR235");
        assert_eq!(format!("{}", general::ERR236), "ERR236");
        assert_eq!(format!("{}", general::ERR237), "ERR237");
        assert_eq!(format!("{}", general::ERR238), "ERR238");
        assert_eq!(format!("{}", general::ERR239), "ERR239");
        assert_eq!(format!("{}", general::ERR240), "ERR240");
        assert_eq!(format!("{}", general::ERR241), "ERR241");
        assert_eq!(format!("{}", general::ERR242), "ERR242");
        assert_eq!(format!("{}", general::ERR243), "ERR243");
        assert_eq!(format!("{}", general::ERR244), "ERR244");
        assert_eq!(format!("{}", general::ERR245), "ERR245");
        assert_eq!(format!("{}", general::ERR246), "ERR246");
        assert_eq!(format!("{}", general::ERR247), "ERR247");
        assert_eq!(format!("{}", general::ERR248), "ERR248");
        assert_eq!(format!("{}", general::ERR249), "ERR249");
        assert_eq!(format!("{}", general::ERR250), "ERR250");
        assert_eq!(format!("{}", general::ERR251), "ERR251");
        assert_eq!(format!("{}", general::ERR252), "ERR252");
        assert_eq!(format!("{}", general::ERR253), "ERR253");
        assert_eq!(format!("{}", general::ERR254), "ERR254");
        assert_eq!(format!("{}", general::ERR255), "ERR255");
    }


    #[test]
    fn test_err_kind_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 256] = [
            general::ERR0,
            general::ERR1,
            general::ERR2,
            general::ERR3,
            general::ERR4,
            general::ERR5,
            general::ERR6,
            general::ERR7,
            general::ERR8,
            general::ERR9,
            general::ERR10,
            general::ERR11,
            general::ERR12,
            general::ERR13,
            general::ERR14,
            general::ERR15,
            general::ERR16,
            general::ERR17,
            general::ERR18,
            general::ERR19,
            general::ERR20,
            general::ERR21,
            general::ERR22,
            general::ERR23,
            general::ERR24,
            general::ERR25,
            general::ERR26,
            general::ERR27,
            general::ERR28,
            general::ERR29,
            general::ERR30,
            general::ERR31,
            general::ERR32,
            general::ERR33,
            general::ERR34,
            general::ERR35,
            general::ERR36,
            general::ERR37,
            general::ERR38,
            general::ERR39,
            general::ERR40,
            general::ERR41,
            general::ERR42,
            general::ERR43,
            general::ERR44,
            general::ERR45,
            general::ERR46,
            general::ERR47,
            general::ERR48,
            general::ERR49,
            general::ERR50,
            general::ERR51,
            general::ERR52,
            general::ERR53,
            general::ERR54,
            general::ERR55,
            general::ERR56,
            general::ERR57,
            general::ERR58,
            general::ERR59,
            general::ERR60,
            general::ERR61,
            general::ERR62,
            general::ERR63,
            general::ERR64,
            general::ERR65,
            general::ERR66,
            general::ERR67,
            general::ERR68,
            general::ERR69,
            general::ERR70,
            general::ERR71,
            general::ERR72,
            general::ERR73,
            general::ERR74,
            general::ERR75,
            general::ERR76,
            general::ERR77,
            general::ERR78,
            general::ERR79,
            general::ERR80,
            general::ERR81,
            general::ERR82,
            general::ERR83,
            general::ERR84,
            general::ERR85,
            general::ERR86,
            general::ERR87,
            general::ERR88,
            general::ERR89,
            general::ERR90,
            general::ERR91,
            general::ERR92,
            general::ERR93,
            general::ERR94,
            general::ERR95,
            general::ERR96,
            general::ERR97,
            general::ERR98,
            general::ERR99,
            general::ERR100,
            general::ERR101,
            general::ERR102,
            general::ERR103,
            general::ERR104,
            general::ERR105,
            general::ERR106,
            general::ERR107,
            general::ERR108,
            general::ERR109,
            general::ERR110,
            general::ERR111,
            general::ERR112,
            general::ERR113,
            general::ERR114,
            general::ERR115,
            general::ERR116,
            general::ERR117,
            general::ERR118,
            general::ERR119,
            general::ERR120,
            general::ERR121,
            general::ERR122,
            general::ERR123,
            general::ERR124,
            general::ERR125,
            general::ERR126,
            general::ERR127,
            general::ERR128,
            general::ERR129,
            general::ERR130,
            general::ERR131,
            general::ERR132,
            general::ERR133,
            general::ERR134,
            general::ERR135,
            general::ERR136,
            general::ERR137,
            general::ERR138,
            general::ERR139,
            general::ERR140,
            general::ERR141,
            general::ERR142,
            general::ERR143,
            general::ERR144,
            general::ERR145,
            general::ERR146,
            general::ERR147,
            general::ERR148,
            general::ERR149,
            general::ERR150,
            general::ERR151,
            general::ERR152,
            general::ERR153,
            general::ERR154,
            general::ERR155,
            general::ERR156,
            general::ERR157,
            general::ERR158,
            general::ERR159,
            general::ERR160,
            general::ERR161,
            general::ERR162,
            general::ERR163,
            general::ERR164,
            general::ERR165,
            general::ERR166,
            general::ERR167,
            general::ERR168,
            general::ERR169,
            general::ERR170,
            general::ERR171,
            general::ERR172,
            general::ERR173,
            general::ERR174,
            general::ERR175,
            general::ERR176,
            general::ERR177,
            general::ERR178,
            general::ERR179,
            general::ERR180,
            general::ERR181,
            general::ERR182,
            general::ERR183,
            general::ERR184,
            general::ERR185,
            general::ERR186,
            general::ERR187,
            general::ERR188,
            general::ERR189,
            general::ERR190,
            general::ERR191,
            general::ERR192,
            general::ERR193,
            general::ERR194,
            general::ERR195,
            general::ERR196,
            general::ERR197,
            general::ERR198,
            general::ERR199,
            general::ERR200,
            general::ERR201,
            general::ERR202,
            general::ERR203,
            general::ERR204,
            general::ERR205,
            general::ERR206,
            general::ERR207,
            general::ERR208,
            general::ERR209,
            general::ERR210,
            general::ERR211,
            general::ERR212,
            general::ERR213,
            general::ERR214,
            general::ERR215,
            general::ERR216,
            general::ERR217,
            general::ERR218,
            general::ERR219,
            general::ERR220,
            general::ERR221,
            general::ERR222,
            general::ERR223,
            general::ERR224,
            general::ERR225,
            general::ERR226,
            general::ERR227,
            general::ERR228,
            general::ERR229,
            general::ERR230,
            general::ERR231,
            general::ERR232,
            general::ERR233,
            general::ERR234,
            general::ERR235,
            general::ERR236,
            general::ERR237,
            general::ERR238,
            general::ERR239,
            general::ERR240,
            general::ERR241,
            general::ERR242,
            general::ERR243,
            general::ERR244,
            general::ERR245,
            general::ERR246,
            general::ERR247,
            general::ERR248,
            general::ERR249,
            general::ERR250,
            general::ERR251,
            general::ERR252,
            general::ERR253,
            general::ERR254,
            general::ERR255,
        ];

        let set = HashSet::<ErrorKind>::from_iter(errs);
        assert_eq!(set.len(), 256);
    }


    #[test]
    fn test_err_kind_value_uniqueness() {
        use kind::*;
        use std::collections::HashSet;
        let errs: [ErrorKind; 256] = [
            general::ERR0,
            general::ERR1,
            general::ERR2,
            general::ERR3,
            general::ERR4,
            general::ERR5,
            general::ERR6,
            general::ERR7,
            general::ERR8,
            general::ERR9,
            general::ERR10,
            general::ERR11,
            general::ERR12,
            general::ERR13,
            general::ERR14,
            general::ERR15,
            general::ERR16,
            general::ERR17,
            general::ERR18,
            general::ERR19,
            general::ERR20,
            general::ERR21,
            general::ERR22,
            general::ERR23,
            general::ERR24,
            general::ERR25,
            general::ERR26,
            general::ERR27,
            general::ERR28,
            general::ERR29,
            general::ERR30,
            general::ERR31,
            general::ERR32,
            general::ERR33,
            general::ERR34,
            general::ERR35,
            general::ERR36,
            general::ERR37,
            general::ERR38,
            general::ERR39,
            general::ERR40,
            general::ERR41,
            general::ERR42,
            general::ERR43,
            general::ERR44,
            general::ERR45,
            general::ERR46,
            general::ERR47,
            general::ERR48,
            general::ERR49,
            general::ERR50,
            general::ERR51,
            general::ERR52,
            general::ERR53,
            general::ERR54,
            general::ERR55,
            general::ERR56,
            general::ERR57,
            general::ERR58,
            general::ERR59,
            general::ERR60,
            general::ERR61,
            general::ERR62,
            general::ERR63,
            general::ERR64,
            general::ERR65,
            general::ERR66,
            general::ERR67,
            general::ERR68,
            general::ERR69,
            general::ERR70,
            general::ERR71,
            general::ERR72,
            general::ERR73,
            general::ERR74,
            general::ERR75,
            general::ERR76,
            general::ERR77,
            general::ERR78,
            general::ERR79,
            general::ERR80,
            general::ERR81,
            general::ERR82,
            general::ERR83,
            general::ERR84,
            general::ERR85,
            general::ERR86,
            general::ERR87,
            general::ERR88,
            general::ERR89,
            general::ERR90,
            general::ERR91,
            general::ERR92,
            general::ERR93,
            general::ERR94,
            general::ERR95,
            general::ERR96,
            general::ERR97,
            general::ERR98,
            general::ERR99,
            general::ERR100,
            general::ERR101,
            general::ERR102,
            general::ERR103,
            general::ERR104,
            general::ERR105,
            general::ERR106,
            general::ERR107,
            general::ERR108,
            general::ERR109,
            general::ERR110,
            general::ERR111,
            general::ERR112,
            general::ERR113,
            general::ERR114,
            general::ERR115,
            general::ERR116,
            general::ERR117,
            general::ERR118,
            general::ERR119,
            general::ERR120,
            general::ERR121,
            general::ERR122,
            general::ERR123,
            general::ERR124,
            general::ERR125,
            general::ERR126,
            general::ERR127,
            general::ERR128,
            general::ERR129,
            general::ERR130,
            general::ERR131,
            general::ERR132,
            general::ERR133,
            general::ERR134,
            general::ERR135,
            general::ERR136,
            general::ERR137,
            general::ERR138,
            general::ERR139,
            general::ERR140,
            general::ERR141,
            general::ERR142,
            general::ERR143,
            general::ERR144,
            general::ERR145,
            general::ERR146,
            general::ERR147,
            general::ERR148,
            general::ERR149,
            general::ERR150,
            general::ERR151,
            general::ERR152,
            general::ERR153,
            general::ERR154,
            general::ERR155,
            general::ERR156,
            general::ERR157,
            general::ERR158,
            general::ERR159,
            general::ERR160,
            general::ERR161,
            general::ERR162,
            general::ERR163,
            general::ERR164,
            general::ERR165,
            general::ERR166,
            general::ERR167,
            general::ERR168,
            general::ERR169,
            general::ERR170,
            general::ERR171,
            general::ERR172,
            general::ERR173,
            general::ERR174,
            general::ERR175,
            general::ERR176,
            general::ERR177,
            general::ERR178,
            general::ERR179,
            general::ERR180,
            general::ERR181,
            general::ERR182,
            general::ERR183,
            general::ERR184,
            general::ERR185,
            general::ERR186,
            general::ERR187,
            general::ERR188,
            general::ERR189,
            general::ERR190,
            general::ERR191,
            general::ERR192,
            general::ERR193,
            general::ERR194,
            general::ERR195,
            general::ERR196,
            general::ERR197,
            general::ERR198,
            general::ERR199,
            general::ERR200,
            general::ERR201,
            general::ERR202,
            general::ERR203,
            general::ERR204,
            general::ERR205,
            general::ERR206,
            general::ERR207,
            general::ERR208,
            general::ERR209,
            general::ERR210,
            general::ERR211,
            general::ERR212,
            general::ERR213,
            general::ERR214,
            general::ERR215,
            general::ERR216,
            general::ERR217,
            general::ERR218,
            general::ERR219,
            general::ERR220,
            general::ERR221,
            general::ERR222,
            general::ERR223,
            general::ERR224,
            general::ERR225,
            general::ERR226,
            general::ERR227,
            general::ERR228,
            general::ERR229,
            general::ERR230,
            general::ERR231,
            general::ERR232,
            general::ERR233,
            general::ERR234,
            general::ERR235,
            general::ERR236,
            general::ERR237,
            general::ERR238,
            general::ERR239,
            general::ERR240,
            general::ERR241,
            general::ERR242,
            general::ERR243,
            general::ERR244,
            general::ERR245,
            general::ERR246,
            general::ERR247,
            general::ERR248,
            general::ERR249,
            general::ERR250,
            general::ERR251,
            general::ERR252,
            general::ERR253,
            general::ERR254,
            general::ERR255,
        ];

        let set = HashSet::<u8>::from_iter(errs.iter().map(|ec| ec.value()));
        assert_eq!(set.len(), 256);
    }


    #[test]
    fn test_err_kind_category() {
        use kind::*;
        assert_eq!(general::ERR0.category(), category::GENERAL);
        assert_eq!(general::ERR1.category(), category::GENERAL);
        assert_eq!(general::ERR2.category(), category::GENERAL);
        assert_eq!(general::ERR3.category(), category::GENERAL);
        assert_eq!(general::ERR4.category(), category::GENERAL);
        assert_eq!(general::ERR5.category(), category::GENERAL);
        assert_eq!(general::ERR6.category(), category::GENERAL);
        assert_eq!(general::ERR7.category(), category::GENERAL);
        assert_eq!(general::ERR8.category(), category::GENERAL);
        assert_eq!(general::ERR9.category(), category::GENERAL);
        assert_eq!(general::ERR10.category(), category::GENERAL);
        assert_eq!(general::ERR11.category(), category::GENERAL);
        assert_eq!(general::ERR12.category(), category::GENERAL);
        assert_eq!(general::ERR13.category(), category::GENERAL);
        assert_eq!(general::ERR14.category(), category::GENERAL);
        assert_eq!(general::ERR15.category(), category::GENERAL);
        assert_eq!(general::ERR16.category(), category::GENERAL);
        assert_eq!(general::ERR17.category(), category::GENERAL);
        assert_eq!(general::ERR18.category(), category::GENERAL);
        assert_eq!(general::ERR19.category(), category::GENERAL);
        assert_eq!(general::ERR20.category(), category::GENERAL);
        assert_eq!(general::ERR21.category(), category::GENERAL);
        assert_eq!(general::ERR22.category(), category::GENERAL);
        assert_eq!(general::ERR23.category(), category::GENERAL);
        assert_eq!(general::ERR24.category(), category::GENERAL);
        assert_eq!(general::ERR25.category(), category::GENERAL);
        assert_eq!(general::ERR26.category(), category::GENERAL);
        assert_eq!(general::ERR27.category(), category::GENERAL);
        assert_eq!(general::ERR28.category(), category::GENERAL);
        assert_eq!(general::ERR29.category(), category::GENERAL);
        assert_eq!(general::ERR30.category(), category::GENERAL);
        assert_eq!(general::ERR31.category(), category::GENERAL);
        assert_eq!(general::ERR32.category(), category::GENERAL);
        assert_eq!(general::ERR33.category(), category::GENERAL);
        assert_eq!(general::ERR34.category(), category::GENERAL);
        assert_eq!(general::ERR35.category(), category::GENERAL);
        assert_eq!(general::ERR36.category(), category::GENERAL);
        assert_eq!(general::ERR37.category(), category::GENERAL);
        assert_eq!(general::ERR38.category(), category::GENERAL);
        assert_eq!(general::ERR39.category(), category::GENERAL);
        assert_eq!(general::ERR40.category(), category::GENERAL);
        assert_eq!(general::ERR41.category(), category::GENERAL);
        assert_eq!(general::ERR42.category(), category::GENERAL);
        assert_eq!(general::ERR43.category(), category::GENERAL);
        assert_eq!(general::ERR44.category(), category::GENERAL);
        assert_eq!(general::ERR45.category(), category::GENERAL);
        assert_eq!(general::ERR46.category(), category::GENERAL);
        assert_eq!(general::ERR47.category(), category::GENERAL);
        assert_eq!(general::ERR48.category(), category::GENERAL);
        assert_eq!(general::ERR49.category(), category::GENERAL);
        assert_eq!(general::ERR50.category(), category::GENERAL);
        assert_eq!(general::ERR51.category(), category::GENERAL);
        assert_eq!(general::ERR52.category(), category::GENERAL);
        assert_eq!(general::ERR53.category(), category::GENERAL);
        assert_eq!(general::ERR54.category(), category::GENERAL);
        assert_eq!(general::ERR55.category(), category::GENERAL);
        assert_eq!(general::ERR56.category(), category::GENERAL);
        assert_eq!(general::ERR57.category(), category::GENERAL);
        assert_eq!(general::ERR58.category(), category::GENERAL);
        assert_eq!(general::ERR59.category(), category::GENERAL);
        assert_eq!(general::ERR60.category(), category::GENERAL);
        assert_eq!(general::ERR61.category(), category::GENERAL);
        assert_eq!(general::ERR62.category(), category::GENERAL);
        assert_eq!(general::ERR63.category(), category::GENERAL);
        assert_eq!(general::ERR64.category(), category::GENERAL);
        assert_eq!(general::ERR65.category(), category::GENERAL);
        assert_eq!(general::ERR66.category(), category::GENERAL);
        assert_eq!(general::ERR67.category(), category::GENERAL);
        assert_eq!(general::ERR68.category(), category::GENERAL);
        assert_eq!(general::ERR69.category(), category::GENERAL);
        assert_eq!(general::ERR70.category(), category::GENERAL);
        assert_eq!(general::ERR71.category(), category::GENERAL);
        assert_eq!(general::ERR72.category(), category::GENERAL);
        assert_eq!(general::ERR73.category(), category::GENERAL);
        assert_eq!(general::ERR74.category(), category::GENERAL);
        assert_eq!(general::ERR75.category(), category::GENERAL);
        assert_eq!(general::ERR76.category(), category::GENERAL);
        assert_eq!(general::ERR77.category(), category::GENERAL);
        assert_eq!(general::ERR78.category(), category::GENERAL);
        assert_eq!(general::ERR79.category(), category::GENERAL);
        assert_eq!(general::ERR80.category(), category::GENERAL);
        assert_eq!(general::ERR81.category(), category::GENERAL);
        assert_eq!(general::ERR82.category(), category::GENERAL);
        assert_eq!(general::ERR83.category(), category::GENERAL);
        assert_eq!(general::ERR84.category(), category::GENERAL);
        assert_eq!(general::ERR85.category(), category::GENERAL);
        assert_eq!(general::ERR86.category(), category::GENERAL);
        assert_eq!(general::ERR87.category(), category::GENERAL);
        assert_eq!(general::ERR88.category(), category::GENERAL);
        assert_eq!(general::ERR89.category(), category::GENERAL);
        assert_eq!(general::ERR90.category(), category::GENERAL);
        assert_eq!(general::ERR91.category(), category::GENERAL);
        assert_eq!(general::ERR92.category(), category::GENERAL);
        assert_eq!(general::ERR93.category(), category::GENERAL);
        assert_eq!(general::ERR94.category(), category::GENERAL);
        assert_eq!(general::ERR95.category(), category::GENERAL);
        assert_eq!(general::ERR96.category(), category::GENERAL);
        assert_eq!(general::ERR97.category(), category::GENERAL);
        assert_eq!(general::ERR98.category(), category::GENERAL);
        assert_eq!(general::ERR99.category(), category::GENERAL);
        assert_eq!(general::ERR100.category(), category::GENERAL);
        assert_eq!(general::ERR101.category(), category::GENERAL);
        assert_eq!(general::ERR102.category(), category::GENERAL);
        assert_eq!(general::ERR103.category(), category::GENERAL);
        assert_eq!(general::ERR104.category(), category::GENERAL);
        assert_eq!(general::ERR105.category(), category::GENERAL);
        assert_eq!(general::ERR106.category(), category::GENERAL);
        assert_eq!(general::ERR107.category(), category::GENERAL);
        assert_eq!(general::ERR108.category(), category::GENERAL);
        assert_eq!(general::ERR109.category(), category::GENERAL);
        assert_eq!(general::ERR110.category(), category::GENERAL);
        assert_eq!(general::ERR111.category(), category::GENERAL);
        assert_eq!(general::ERR112.category(), category::GENERAL);
        assert_eq!(general::ERR113.category(), category::GENERAL);
        assert_eq!(general::ERR114.category(), category::GENERAL);
        assert_eq!(general::ERR115.category(), category::GENERAL);
        assert_eq!(general::ERR116.category(), category::GENERAL);
        assert_eq!(general::ERR117.category(), category::GENERAL);
        assert_eq!(general::ERR118.category(), category::GENERAL);
        assert_eq!(general::ERR119.category(), category::GENERAL);
        assert_eq!(general::ERR120.category(), category::GENERAL);
        assert_eq!(general::ERR121.category(), category::GENERAL);
        assert_eq!(general::ERR122.category(), category::GENERAL);
        assert_eq!(general::ERR123.category(), category::GENERAL);
        assert_eq!(general::ERR124.category(), category::GENERAL);
        assert_eq!(general::ERR125.category(), category::GENERAL);
        assert_eq!(general::ERR126.category(), category::GENERAL);
        assert_eq!(general::ERR127.category(), category::GENERAL);
        assert_eq!(general::ERR128.category(), category::GENERAL);
        assert_eq!(general::ERR129.category(), category::GENERAL);
        assert_eq!(general::ERR130.category(), category::GENERAL);
        assert_eq!(general::ERR131.category(), category::GENERAL);
        assert_eq!(general::ERR132.category(), category::GENERAL);
        assert_eq!(general::ERR133.category(), category::GENERAL);
        assert_eq!(general::ERR134.category(), category::GENERAL);
        assert_eq!(general::ERR135.category(), category::GENERAL);
        assert_eq!(general::ERR136.category(), category::GENERAL);
        assert_eq!(general::ERR137.category(), category::GENERAL);
        assert_eq!(general::ERR138.category(), category::GENERAL);
        assert_eq!(general::ERR139.category(), category::GENERAL);
        assert_eq!(general::ERR140.category(), category::GENERAL);
        assert_eq!(general::ERR141.category(), category::GENERAL);
        assert_eq!(general::ERR142.category(), category::GENERAL);
        assert_eq!(general::ERR143.category(), category::GENERAL);
        assert_eq!(general::ERR144.category(), category::GENERAL);
        assert_eq!(general::ERR145.category(), category::GENERAL);
        assert_eq!(general::ERR146.category(), category::GENERAL);
        assert_eq!(general::ERR147.category(), category::GENERAL);
        assert_eq!(general::ERR148.category(), category::GENERAL);
        assert_eq!(general::ERR149.category(), category::GENERAL);
        assert_eq!(general::ERR150.category(), category::GENERAL);
        assert_eq!(general::ERR151.category(), category::GENERAL);
        assert_eq!(general::ERR152.category(), category::GENERAL);
        assert_eq!(general::ERR153.category(), category::GENERAL);
        assert_eq!(general::ERR154.category(), category::GENERAL);
        assert_eq!(general::ERR155.category(), category::GENERAL);
        assert_eq!(general::ERR156.category(), category::GENERAL);
        assert_eq!(general::ERR157.category(), category::GENERAL);
        assert_eq!(general::ERR158.category(), category::GENERAL);
        assert_eq!(general::ERR159.category(), category::GENERAL);
        assert_eq!(general::ERR160.category(), category::GENERAL);
        assert_eq!(general::ERR161.category(), category::GENERAL);
        assert_eq!(general::ERR162.category(), category::GENERAL);
        assert_eq!(general::ERR163.category(), category::GENERAL);
        assert_eq!(general::ERR164.category(), category::GENERAL);
        assert_eq!(general::ERR165.category(), category::GENERAL);
        assert_eq!(general::ERR166.category(), category::GENERAL);
        assert_eq!(general::ERR167.category(), category::GENERAL);
        assert_eq!(general::ERR168.category(), category::GENERAL);
        assert_eq!(general::ERR169.category(), category::GENERAL);
        assert_eq!(general::ERR170.category(), category::GENERAL);
        assert_eq!(general::ERR171.category(), category::GENERAL);
        assert_eq!(general::ERR172.category(), category::GENERAL);
        assert_eq!(general::ERR173.category(), category::GENERAL);
        assert_eq!(general::ERR174.category(), category::GENERAL);
        assert_eq!(general::ERR175.category(), category::GENERAL);
        assert_eq!(general::ERR176.category(), category::GENERAL);
        assert_eq!(general::ERR177.category(), category::GENERAL);
        assert_eq!(general::ERR178.category(), category::GENERAL);
        assert_eq!(general::ERR179.category(), category::GENERAL);
        assert_eq!(general::ERR180.category(), category::GENERAL);
        assert_eq!(general::ERR181.category(), category::GENERAL);
        assert_eq!(general::ERR182.category(), category::GENERAL);
        assert_eq!(general::ERR183.category(), category::GENERAL);
        assert_eq!(general::ERR184.category(), category::GENERAL);
        assert_eq!(general::ERR185.category(), category::GENERAL);
        assert_eq!(general::ERR186.category(), category::GENERAL);
        assert_eq!(general::ERR187.category(), category::GENERAL);
        assert_eq!(general::ERR188.category(), category::GENERAL);
        assert_eq!(general::ERR189.category(), category::GENERAL);
        assert_eq!(general::ERR190.category(), category::GENERAL);
        assert_eq!(general::ERR191.category(), category::GENERAL);
        assert_eq!(general::ERR192.category(), category::GENERAL);
        assert_eq!(general::ERR193.category(), category::GENERAL);
        assert_eq!(general::ERR194.category(), category::GENERAL);
        assert_eq!(general::ERR195.category(), category::GENERAL);
        assert_eq!(general::ERR196.category(), category::GENERAL);
        assert_eq!(general::ERR197.category(), category::GENERAL);
        assert_eq!(general::ERR198.category(), category::GENERAL);
        assert_eq!(general::ERR199.category(), category::GENERAL);
        assert_eq!(general::ERR200.category(), category::GENERAL);
        assert_eq!(general::ERR201.category(), category::GENERAL);
        assert_eq!(general::ERR202.category(), category::GENERAL);
        assert_eq!(general::ERR203.category(), category::GENERAL);
        assert_eq!(general::ERR204.category(), category::GENERAL);
        assert_eq!(general::ERR205.category(), category::GENERAL);
        assert_eq!(general::ERR206.category(), category::GENERAL);
        assert_eq!(general::ERR207.category(), category::GENERAL);
        assert_eq!(general::ERR208.category(), category::GENERAL);
        assert_eq!(general::ERR209.category(), category::GENERAL);
        assert_eq!(general::ERR210.category(), category::GENERAL);
        assert_eq!(general::ERR211.category(), category::GENERAL);
        assert_eq!(general::ERR212.category(), category::GENERAL);
        assert_eq!(general::ERR213.category(), category::GENERAL);
        assert_eq!(general::ERR214.category(), category::GENERAL);
        assert_eq!(general::ERR215.category(), category::GENERAL);
        assert_eq!(general::ERR216.category(), category::GENERAL);
        assert_eq!(general::ERR217.category(), category::GENERAL);
        assert_eq!(general::ERR218.category(), category::GENERAL);
        assert_eq!(general::ERR219.category(), category::GENERAL);
        assert_eq!(general::ERR220.category(), category::GENERAL);
        assert_eq!(general::ERR221.category(), category::GENERAL);
        assert_eq!(general::ERR222.category(), category::GENERAL);
        assert_eq!(general::ERR223.category(), category::GENERAL);
        assert_eq!(general::ERR224.category(), category::GENERAL);
        assert_eq!(general::ERR225.category(), category::GENERAL);
        assert_eq!(general::ERR226.category(), category::GENERAL);
        assert_eq!(general::ERR227.category(), category::GENERAL);
        assert_eq!(general::ERR228.category(), category::GENERAL);
        assert_eq!(general::ERR229.category(), category::GENERAL);
        assert_eq!(general::ERR230.category(), category::GENERAL);
        assert_eq!(general::ERR231.category(), category::GENERAL);
        assert_eq!(general::ERR232.category(), category::GENERAL);
        assert_eq!(general::ERR233.category(), category::GENERAL);
        assert_eq!(general::ERR234.category(), category::GENERAL);
        assert_eq!(general::ERR235.category(), category::GENERAL);
        assert_eq!(general::ERR236.category(), category::GENERAL);
        assert_eq!(general::ERR237.category(), category::GENERAL);
        assert_eq!(general::ERR238.category(), category::GENERAL);
        assert_eq!(general::ERR239.category(), category::GENERAL);
        assert_eq!(general::ERR240.category(), category::GENERAL);
        assert_eq!(general::ERR241.category(), category::GENERAL);
        assert_eq!(general::ERR242.category(), category::GENERAL);
        assert_eq!(general::ERR243.category(), category::GENERAL);
        assert_eq!(general::ERR244.category(), category::GENERAL);
        assert_eq!(general::ERR245.category(), category::GENERAL);
        assert_eq!(general::ERR246.category(), category::GENERAL);
        assert_eq!(general::ERR247.category(), category::GENERAL);
        assert_eq!(general::ERR248.category(), category::GENERAL);
        assert_eq!(general::ERR249.category(), category::GENERAL);
        assert_eq!(general::ERR250.category(), category::GENERAL);
        assert_eq!(general::ERR251.category(), category::GENERAL);
        assert_eq!(general::ERR252.category(), category::GENERAL);
        assert_eq!(general::ERR253.category(), category::GENERAL);
        assert_eq!(general::ERR254.category(), category::GENERAL);
        assert_eq!(general::ERR255.category(), category::GENERAL);
    }


    #[test]
    fn test_err_kind_from_value() {
        use kind::*;
        assert_eq!(ErrorKind::from_value(general::ERR0.value()).unwrap(), general::ERR0);
        assert_eq!(ErrorKind::from_value(general::ERR1.value()).unwrap(), general::ERR1);
        assert_eq!(ErrorKind::from_value(general::ERR2.value()).unwrap(), general::ERR2);
        assert_eq!(ErrorKind::from_value(general::ERR3.value()).unwrap(), general::ERR3);
        assert_eq!(ErrorKind::from_value(general::ERR4.value()).unwrap(), general::ERR4);
        assert_eq!(ErrorKind::from_value(general::ERR5.value()).unwrap(), general::ERR5);
        assert_eq!(ErrorKind::from_value(general::ERR6.value()).unwrap(), general::ERR6);
        assert_eq!(ErrorKind::from_value(general::ERR7.value()).unwrap(), general::ERR7);
        assert_eq!(ErrorKind::from_value(general::ERR8.value()).unwrap(), general::ERR8);
        assert_eq!(ErrorKind::from_value(general::ERR9.value()).unwrap(), general::ERR9);
        assert_eq!(
            ErrorKind::from_value(general::ERR10.value()).unwrap(), general::ERR10
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR11.value()).unwrap(), general::ERR11
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR12.value()).unwrap(), general::ERR12
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR13.value()).unwrap(), general::ERR13
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR14.value()).unwrap(), general::ERR14
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR15.value()).unwrap(), general::ERR15
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR16.value()).unwrap(), general::ERR16
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR17.value()).unwrap(), general::ERR17
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR18.value()).unwrap(), general::ERR18
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR19.value()).unwrap(), general::ERR19
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR20.value()).unwrap(), general::ERR20
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR21.value()).unwrap(), general::ERR21
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR22.value()).unwrap(), general::ERR22
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR23.value()).unwrap(), general::ERR23
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR24.value()).unwrap(), general::ERR24
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR25.value()).unwrap(), general::ERR25
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR26.value()).unwrap(), general::ERR26
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR27.value()).unwrap(), general::ERR27
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR28.value()).unwrap(), general::ERR28
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR29.value()).unwrap(), general::ERR29
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR30.value()).unwrap(), general::ERR30
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR31.value()).unwrap(), general::ERR31
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR32.value()).unwrap(), general::ERR32
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR33.value()).unwrap(), general::ERR33
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR34.value()).unwrap(), general::ERR34
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR35.value()).unwrap(), general::ERR35
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR36.value()).unwrap(), general::ERR36
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR37.value()).unwrap(), general::ERR37
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR38.value()).unwrap(), general::ERR38
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR39.value()).unwrap(), general::ERR39
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR40.value()).unwrap(), general::ERR40
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR41.value()).unwrap(), general::ERR41
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR42.value()).unwrap(), general::ERR42
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR43.value()).unwrap(), general::ERR43
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR44.value()).unwrap(), general::ERR44
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR45.value()).unwrap(), general::ERR45
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR46.value()).unwrap(), general::ERR46
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR47.value()).unwrap(), general::ERR47
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR48.value()).unwrap(), general::ERR48
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR49.value()).unwrap(), general::ERR49
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR50.value()).unwrap(), general::ERR50
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR51.value()).unwrap(), general::ERR51
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR52.value()).unwrap(), general::ERR52
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR53.value()).unwrap(), general::ERR53
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR54.value()).unwrap(), general::ERR54
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR55.value()).unwrap(), general::ERR55
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR56.value()).unwrap(), general::ERR56
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR57.value()).unwrap(), general::ERR57
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR58.value()).unwrap(), general::ERR58
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR59.value()).unwrap(), general::ERR59
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR60.value()).unwrap(), general::ERR60
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR61.value()).unwrap(), general::ERR61
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR62.value()).unwrap(), general::ERR62
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR63.value()).unwrap(), general::ERR63
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR64.value()).unwrap(), general::ERR64
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR65.value()).unwrap(), general::ERR65
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR66.value()).unwrap(), general::ERR66
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR67.value()).unwrap(), general::ERR67
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR68.value()).unwrap(), general::ERR68
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR69.value()).unwrap(), general::ERR69
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR70.value()).unwrap(), general::ERR70
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR71.value()).unwrap(), general::ERR71
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR72.value()).unwrap(), general::ERR72
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR73.value()).unwrap(), general::ERR73
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR74.value()).unwrap(), general::ERR74
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR75.value()).unwrap(), general::ERR75
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR76.value()).unwrap(), general::ERR76
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR77.value()).unwrap(), general::ERR77
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR78.value()).unwrap(), general::ERR78
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR79.value()).unwrap(), general::ERR79
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR80.value()).unwrap(), general::ERR80
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR81.value()).unwrap(), general::ERR81
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR82.value()).unwrap(), general::ERR82
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR83.value()).unwrap(), general::ERR83
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR84.value()).unwrap(), general::ERR84
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR85.value()).unwrap(), general::ERR85
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR86.value()).unwrap(), general::ERR86
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR87.value()).unwrap(), general::ERR87
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR88.value()).unwrap(), general::ERR88
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR89.value()).unwrap(), general::ERR89
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR90.value()).unwrap(), general::ERR90
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR91.value()).unwrap(), general::ERR91
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR92.value()).unwrap(), general::ERR92
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR93.value()).unwrap(), general::ERR93
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR94.value()).unwrap(), general::ERR94
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR95.value()).unwrap(), general::ERR95
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR96.value()).unwrap(), general::ERR96
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR97.value()).unwrap(), general::ERR97
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR98.value()).unwrap(), general::ERR98
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR99.value()).unwrap(), general::ERR99
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR100.value()).unwrap(), general::ERR100
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR101.value()).unwrap(), general::ERR101
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR102.value()).unwrap(), general::ERR102
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR103.value()).unwrap(), general::ERR103
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR104.value()).unwrap(), general::ERR104
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR105.value()).unwrap(), general::ERR105
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR106.value()).unwrap(), general::ERR106
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR107.value()).unwrap(), general::ERR107
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR108.value()).unwrap(), general::ERR108
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR109.value()).unwrap(), general::ERR109
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR110.value()).unwrap(), general::ERR110
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR111.value()).unwrap(), general::ERR111
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR112.value()).unwrap(), general::ERR112
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR113.value()).unwrap(), general::ERR113
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR114.value()).unwrap(), general::ERR114
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR115.value()).unwrap(), general::ERR115
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR116.value()).unwrap(), general::ERR116
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR117.value()).unwrap(), general::ERR117
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR118.value()).unwrap(), general::ERR118
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR119.value()).unwrap(), general::ERR119
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR120.value()).unwrap(), general::ERR120
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR121.value()).unwrap(), general::ERR121
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR122.value()).unwrap(), general::ERR122
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR123.value()).unwrap(), general::ERR123
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR124.value()).unwrap(), general::ERR124
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR125.value()).unwrap(), general::ERR125
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR126.value()).unwrap(), general::ERR126
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR127.value()).unwrap(), general::ERR127
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR128.value()).unwrap(), general::ERR128
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR129.value()).unwrap(), general::ERR129
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR130.value()).unwrap(), general::ERR130
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR131.value()).unwrap(), general::ERR131
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR132.value()).unwrap(), general::ERR132
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR133.value()).unwrap(), general::ERR133
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR134.value()).unwrap(), general::ERR134
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR135.value()).unwrap(), general::ERR135
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR136.value()).unwrap(), general::ERR136
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR137.value()).unwrap(), general::ERR137
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR138.value()).unwrap(), general::ERR138
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR139.value()).unwrap(), general::ERR139
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR140.value()).unwrap(), general::ERR140
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR141.value()).unwrap(), general::ERR141
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR142.value()).unwrap(), general::ERR142
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR143.value()).unwrap(), general::ERR143
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR144.value()).unwrap(), general::ERR144
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR145.value()).unwrap(), general::ERR145
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR146.value()).unwrap(), general::ERR146
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR147.value()).unwrap(), general::ERR147
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR148.value()).unwrap(), general::ERR148
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR149.value()).unwrap(), general::ERR149
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR150.value()).unwrap(), general::ERR150
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR151.value()).unwrap(), general::ERR151
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR152.value()).unwrap(), general::ERR152
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR153.value()).unwrap(), general::ERR153
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR154.value()).unwrap(), general::ERR154
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR155.value()).unwrap(), general::ERR155
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR156.value()).unwrap(), general::ERR156
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR157.value()).unwrap(), general::ERR157
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR158.value()).unwrap(), general::ERR158
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR159.value()).unwrap(), general::ERR159
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR160.value()).unwrap(), general::ERR160
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR161.value()).unwrap(), general::ERR161
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR162.value()).unwrap(), general::ERR162
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR163.value()).unwrap(), general::ERR163
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR164.value()).unwrap(), general::ERR164
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR165.value()).unwrap(), general::ERR165
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR166.value()).unwrap(), general::ERR166
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR167.value()).unwrap(), general::ERR167
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR168.value()).unwrap(), general::ERR168
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR169.value()).unwrap(), general::ERR169
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR170.value()).unwrap(), general::ERR170
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR171.value()).unwrap(), general::ERR171
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR172.value()).unwrap(), general::ERR172
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR173.value()).unwrap(), general::ERR173
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR174.value()).unwrap(), general::ERR174
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR175.value()).unwrap(), general::ERR175
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR176.value()).unwrap(), general::ERR176
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR177.value()).unwrap(), general::ERR177
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR178.value()).unwrap(), general::ERR178
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR179.value()).unwrap(), general::ERR179
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR180.value()).unwrap(), general::ERR180
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR181.value()).unwrap(), general::ERR181
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR182.value()).unwrap(), general::ERR182
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR183.value()).unwrap(), general::ERR183
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR184.value()).unwrap(), general::ERR184
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR185.value()).unwrap(), general::ERR185
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR186.value()).unwrap(), general::ERR186
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR187.value()).unwrap(), general::ERR187
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR188.value()).unwrap(), general::ERR188
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR189.value()).unwrap(), general::ERR189
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR190.value()).unwrap(), general::ERR190
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR191.value()).unwrap(), general::ERR191
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR192.value()).unwrap(), general::ERR192
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR193.value()).unwrap(), general::ERR193
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR194.value()).unwrap(), general::ERR194
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR195.value()).unwrap(), general::ERR195
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR196.value()).unwrap(), general::ERR196
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR197.value()).unwrap(), general::ERR197
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR198.value()).unwrap(), general::ERR198
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR199.value()).unwrap(), general::ERR199
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR200.value()).unwrap(), general::ERR200
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR201.value()).unwrap(), general::ERR201
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR202.value()).unwrap(), general::ERR202
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR203.value()).unwrap(), general::ERR203
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR204.value()).unwrap(), general::ERR204
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR205.value()).unwrap(), general::ERR205
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR206.value()).unwrap(), general::ERR206
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR207.value()).unwrap(), general::ERR207
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR208.value()).unwrap(), general::ERR208
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR209.value()).unwrap(), general::ERR209
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR210.value()).unwrap(), general::ERR210
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR211.value()).unwrap(), general::ERR211
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR212.value()).unwrap(), general::ERR212
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR213.value()).unwrap(), general::ERR213
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR214.value()).unwrap(), general::ERR214
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR215.value()).unwrap(), general::ERR215
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR216.value()).unwrap(), general::ERR216
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR217.value()).unwrap(), general::ERR217
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR218.value()).unwrap(), general::ERR218
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR219.value()).unwrap(), general::ERR219
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR220.value()).unwrap(), general::ERR220
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR221.value()).unwrap(), general::ERR221
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR222.value()).unwrap(), general::ERR222
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR223.value()).unwrap(), general::ERR223
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR224.value()).unwrap(), general::ERR224
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR225.value()).unwrap(), general::ERR225
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR226.value()).unwrap(), general::ERR226
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR227.value()).unwrap(), general::ERR227
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR228.value()).unwrap(), general::ERR228
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR229.value()).unwrap(), general::ERR229
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR230.value()).unwrap(), general::ERR230
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR231.value()).unwrap(), general::ERR231
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR232.value()).unwrap(), general::ERR232
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR233.value()).unwrap(), general::ERR233
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR234.value()).unwrap(), general::ERR234
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR235.value()).unwrap(), general::ERR235
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR236.value()).unwrap(), general::ERR236
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR237.value()).unwrap(), general::ERR237
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR238.value()).unwrap(), general::ERR238
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR239.value()).unwrap(), general::ERR239
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR240.value()).unwrap(), general::ERR240
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR241.value()).unwrap(), general::ERR241
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR242.value()).unwrap(), general::ERR242
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR243.value()).unwrap(), general::ERR243
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR244.value()).unwrap(), general::ERR244
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR245.value()).unwrap(), general::ERR245
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR246.value()).unwrap(), general::ERR246
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR247.value()).unwrap(), general::ERR247
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR248.value()).unwrap(), general::ERR248
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR249.value()).unwrap(), general::ERR249
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR250.value()).unwrap(), general::ERR250
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR251.value()).unwrap(), general::ERR251
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR252.value()).unwrap(), general::ERR252
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR253.value()).unwrap(), general::ERR253
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR254.value()).unwrap(), general::ERR254
        );
        assert_eq!(
            ErrorKind::from_value(general::ERR255.value()).unwrap(), general::ERR255
        );
    }


    #[test]
    fn test_err_display() {
        use kind::*;
        assert_eq!(format!("{}", Error::from(general::ERR0)), "ERR0");
        assert_eq!(format!("{}", Error::from(general::ERR1)), "ERR1");
        assert_eq!(format!("{}", Error::from(general::ERR2)), "ERR2");
        assert_eq!(format!("{}", Error::from(general::ERR3)), "ERR3");
        assert_eq!(format!("{}", Error::from(general::ERR4)), "ERR4");
        assert_eq!(format!("{}", Error::from(general::ERR5)), "ERR5");
        assert_eq!(format!("{}", Error::from(general::ERR6)), "ERR6");
        assert_eq!(format!("{}", Error::from(general::ERR7)), "ERR7");
        assert_eq!(format!("{}", Error::from(general::ERR8)), "ERR8");
        assert_eq!(format!("{}", Error::from(general::ERR9)), "ERR9");
        assert_eq!(format!("{}", Error::from(general::ERR10)), "ERR10");
        assert_eq!(format!("{}", Error::from(general::ERR11)), "ERR11");
        assert_eq!(format!("{}", Error::from(general::ERR12)), "ERR12");
        assert_eq!(format!("{}", Error::from(general::ERR13)), "ERR13");
        assert_eq!(format!("{}", Error::from(general::ERR14)), "ERR14");
        assert_eq!(format!("{}", Error::from(general::ERR15)), "ERR15");
        assert_eq!(format!("{}", Error::from(general::ERR16)), "ERR16");
        assert_eq!(format!("{}", Error::from(general::ERR17)), "ERR17");
        assert_eq!(format!("{}", Error::from(general::ERR18)), "ERR18");
        assert_eq!(format!("{}", Error::from(general::ERR19)), "ERR19");
        assert_eq!(format!("{}", Error::from(general::ERR20)), "ERR20");
        assert_eq!(format!("{}", Error::from(general::ERR21)), "ERR21");
        assert_eq!(format!("{}", Error::from(general::ERR22)), "ERR22");
        assert_eq!(format!("{}", Error::from(general::ERR23)), "ERR23");
        assert_eq!(format!("{}", Error::from(general::ERR24)), "ERR24");
        assert_eq!(format!("{}", Error::from(general::ERR25)), "ERR25");
        assert_eq!(format!("{}", Error::from(general::ERR26)), "ERR26");
        assert_eq!(format!("{}", Error::from(general::ERR27)), "ERR27");
        assert_eq!(format!("{}", Error::from(general::ERR28)), "ERR28");
        assert_eq!(format!("{}", Error::from(general::ERR29)), "ERR29");
        assert_eq!(format!("{}", Error::from(general::ERR30)), "ERR30");
        assert_eq!(format!("{}", Error::from(general::ERR31)), "ERR31");
        assert_eq!(format!("{}", Error::from(general::ERR32)), "ERR32");
        assert_eq!(format!("{}", Error::from(general::ERR33)), "ERR33");
        assert_eq!(format!("{}", Error::from(general::ERR34)), "ERR34");
        assert_eq!(format!("{}", Error::from(general::ERR35)), "ERR35");
        assert_eq!(format!("{}", Error::from(general::ERR36)), "ERR36");
        assert_eq!(format!("{}", Error::from(general::ERR37)), "ERR37");
        assert_eq!(format!("{}", Error::from(general::ERR38)), "ERR38");
        assert_eq!(format!("{}", Error::from(general::ERR39)), "ERR39");
        assert_eq!(format!("{}", Error::from(general::ERR40)), "ERR40");
        assert_eq!(format!("{}", Error::from(general::ERR41)), "ERR41");
        assert_eq!(format!("{}", Error::from(general::ERR42)), "ERR42");
        assert_eq!(format!("{}", Error::from(general::ERR43)), "ERR43");
        assert_eq!(format!("{}", Error::from(general::ERR44)), "ERR44");
        assert_eq!(format!("{}", Error::from(general::ERR45)), "ERR45");
        assert_eq!(format!("{}", Error::from(general::ERR46)), "ERR46");
        assert_eq!(format!("{}", Error::from(general::ERR47)), "ERR47");
        assert_eq!(format!("{}", Error::from(general::ERR48)), "ERR48");
        assert_eq!(format!("{}", Error::from(general::ERR49)), "ERR49");
        assert_eq!(format!("{}", Error::from(general::ERR50)), "ERR50");
        assert_eq!(format!("{}", Error::from(general::ERR51)), "ERR51");
        assert_eq!(format!("{}", Error::from(general::ERR52)), "ERR52");
        assert_eq!(format!("{}", Error::from(general::ERR53)), "ERR53");
        assert_eq!(format!("{}", Error::from(general::ERR54)), "ERR54");
        assert_eq!(format!("{}", Error::from(general::ERR55)), "ERR55");
        assert_eq!(format!("{}", Error::from(general::ERR56)), "ERR56");
        assert_eq!(format!("{}", Error::from(general::ERR57)), "ERR57");
        assert_eq!(format!("{}", Error::from(general::ERR58)), "ERR58");
        assert_eq!(format!("{}", Error::from(general::ERR59)), "ERR59");
        assert_eq!(format!("{}", Error::from(general::ERR60)), "ERR60");
        assert_eq!(format!("{}", Error::from(general::ERR61)), "ERR61");
        assert_eq!(format!("{}", Error::from(general::ERR62)), "ERR62");
        assert_eq!(format!("{}", Error::from(general::ERR63)), "ERR63");
        assert_eq!(format!("{}", Error::from(general::ERR64)), "ERR64");
        assert_eq!(format!("{}", Error::from(general::ERR65)), "ERR65");
        assert_eq!(format!("{}", Error::from(general::ERR66)), "ERR66");
        assert_eq!(format!("{}", Error::from(general::ERR67)), "ERR67");
        assert_eq!(format!("{}", Error::from(general::ERR68)), "ERR68");
        assert_eq!(format!("{}", Error::from(general::ERR69)), "ERR69");
        assert_eq!(format!("{}", Error::from(general::ERR70)), "ERR70");
        assert_eq!(format!("{}", Error::from(general::ERR71)), "ERR71");
        assert_eq!(format!("{}", Error::from(general::ERR72)), "ERR72");
        assert_eq!(format!("{}", Error::from(general::ERR73)), "ERR73");
        assert_eq!(format!("{}", Error::from(general::ERR74)), "ERR74");
        assert_eq!(format!("{}", Error::from(general::ERR75)), "ERR75");
        assert_eq!(format!("{}", Error::from(general::ERR76)), "ERR76");
        assert_eq!(format!("{}", Error::from(general::ERR77)), "ERR77");
        assert_eq!(format!("{}", Error::from(general::ERR78)), "ERR78");
        assert_eq!(format!("{}", Error::from(general::ERR79)), "ERR79");
        assert_eq!(format!("{}", Error::from(general::ERR80)), "ERR80");
        assert_eq!(format!("{}", Error::from(general::ERR81)), "ERR81");
        assert_eq!(format!("{}", Error::from(general::ERR82)), "ERR82");
        assert_eq!(format!("{}", Error::from(general::ERR83)), "ERR83");
        assert_eq!(format!("{}", Error::from(general::ERR84)), "ERR84");
        assert_eq!(format!("{}", Error::from(general::ERR85)), "ERR85");
        assert_eq!(format!("{}", Error::from(general::ERR86)), "ERR86");
        assert_eq!(format!("{}", Error::from(general::ERR87)), "ERR87");
        assert_eq!(format!("{}", Error::from(general::ERR88)), "ERR88");
        assert_eq!(format!("{}", Error::from(general::ERR89)), "ERR89");
        assert_eq!(format!("{}", Error::from(general::ERR90)), "ERR90");
        assert_eq!(format!("{}", Error::from(general::ERR91)), "ERR91");
        assert_eq!(format!("{}", Error::from(general::ERR92)), "ERR92");
        assert_eq!(format!("{}", Error::from(general::ERR93)), "ERR93");
        assert_eq!(format!("{}", Error::from(general::ERR94)), "ERR94");
        assert_eq!(format!("{}", Error::from(general::ERR95)), "ERR95");
        assert_eq!(format!("{}", Error::from(general::ERR96)), "ERR96");
        assert_eq!(format!("{}", Error::from(general::ERR97)), "ERR97");
        assert_eq!(format!("{}", Error::from(general::ERR98)), "ERR98");
        assert_eq!(format!("{}", Error::from(general::ERR99)), "ERR99");
        assert_eq!(format!("{}", Error::from(general::ERR100)), "ERR100");
        assert_eq!(format!("{}", Error::from(general::ERR101)), "ERR101");
        assert_eq!(format!("{}", Error::from(general::ERR102)), "ERR102");
        assert_eq!(format!("{}", Error::from(general::ERR103)), "ERR103");
        assert_eq!(format!("{}", Error::from(general::ERR104)), "ERR104");
        assert_eq!(format!("{}", Error::from(general::ERR105)), "ERR105");
        assert_eq!(format!("{}", Error::from(general::ERR106)), "ERR106");
        assert_eq!(format!("{}", Error::from(general::ERR107)), "ERR107");
        assert_eq!(format!("{}", Error::from(general::ERR108)), "ERR108");
        assert_eq!(format!("{}", Error::from(general::ERR109)), "ERR109");
        assert_eq!(format!("{}", Error::from(general::ERR110)), "ERR110");
        assert_eq!(format!("{}", Error::from(general::ERR111)), "ERR111");
        assert_eq!(format!("{}", Error::from(general::ERR112)), "ERR112");
        assert_eq!(format!("{}", Error::from(general::ERR113)), "ERR113");
        assert_eq!(format!("{}", Error::from(general::ERR114)), "ERR114");
        assert_eq!(format!("{}", Error::from(general::ERR115)), "ERR115");
        assert_eq!(format!("{}", Error::from(general::ERR116)), "ERR116");
        assert_eq!(format!("{}", Error::from(general::ERR117)), "ERR117");
        assert_eq!(format!("{}", Error::from(general::ERR118)), "ERR118");
        assert_eq!(format!("{}", Error::from(general::ERR119)), "ERR119");
        assert_eq!(format!("{}", Error::from(general::ERR120)), "ERR120");
        assert_eq!(format!("{}", Error::from(general::ERR121)), "ERR121");
        assert_eq!(format!("{}", Error::from(general::ERR122)), "ERR122");
        assert_eq!(format!("{}", Error::from(general::ERR123)), "ERR123");
        assert_eq!(format!("{}", Error::from(general::ERR124)), "ERR124");
        assert_eq!(format!("{}", Error::from(general::ERR125)), "ERR125");
        assert_eq!(format!("{}", Error::from(general::ERR126)), "ERR126");
        assert_eq!(format!("{}", Error::from(general::ERR127)), "ERR127");
        assert_eq!(format!("{}", Error::from(general::ERR128)), "ERR128");
        assert_eq!(format!("{}", Error::from(general::ERR129)), "ERR129");
        assert_eq!(format!("{}", Error::from(general::ERR130)), "ERR130");
        assert_eq!(format!("{}", Error::from(general::ERR131)), "ERR131");
        assert_eq!(format!("{}", Error::from(general::ERR132)), "ERR132");
        assert_eq!(format!("{}", Error::from(general::ERR133)), "ERR133");
        assert_eq!(format!("{}", Error::from(general::ERR134)), "ERR134");
        assert_eq!(format!("{}", Error::from(general::ERR135)), "ERR135");
        assert_eq!(format!("{}", Error::from(general::ERR136)), "ERR136");
        assert_eq!(format!("{}", Error::from(general::ERR137)), "ERR137");
        assert_eq!(format!("{}", Error::from(general::ERR138)), "ERR138");
        assert_eq!(format!("{}", Error::from(general::ERR139)), "ERR139");
        assert_eq!(format!("{}", Error::from(general::ERR140)), "ERR140");
        assert_eq!(format!("{}", Error::from(general::ERR141)), "ERR141");
        assert_eq!(format!("{}", Error::from(general::ERR142)), "ERR142");
        assert_eq!(format!("{}", Error::from(general::ERR143)), "ERR143");
        assert_eq!(format!("{}", Error::from(general::ERR144)), "ERR144");
        assert_eq!(format!("{}", Error::from(general::ERR145)), "ERR145");
        assert_eq!(format!("{}", Error::from(general::ERR146)), "ERR146");
        assert_eq!(format!("{}", Error::from(general::ERR147)), "ERR147");
        assert_eq!(format!("{}", Error::from(general::ERR148)), "ERR148");
        assert_eq!(format!("{}", Error::from(general::ERR149)), "ERR149");
        assert_eq!(format!("{}", Error::from(general::ERR150)), "ERR150");
        assert_eq!(format!("{}", Error::from(general::ERR151)), "ERR151");
        assert_eq!(format!("{}", Error::from(general::ERR152)), "ERR152");
        assert_eq!(format!("{}", Error::from(general::ERR153)), "ERR153");
        assert_eq!(format!("{}", Error::from(general::ERR154)), "ERR154");
        assert_eq!(format!("{}", Error::from(general::ERR155)), "ERR155");
        assert_eq!(format!("{}", Error::from(general::ERR156)), "ERR156");
        assert_eq!(format!("{}", Error::from(general::ERR157)), "ERR157");
        assert_eq!(format!("{}", Error::from(general::ERR158)), "ERR158");
        assert_eq!(format!("{}", Error::from(general::ERR159)), "ERR159");
        assert_eq!(format!("{}", Error::from(general::ERR160)), "ERR160");
        assert_eq!(format!("{}", Error::from(general::ERR161)), "ERR161");
        assert_eq!(format!("{}", Error::from(general::ERR162)), "ERR162");
        assert_eq!(format!("{}", Error::from(general::ERR163)), "ERR163");
        assert_eq!(format!("{}", Error::from(general::ERR164)), "ERR164");
        assert_eq!(format!("{}", Error::from(general::ERR165)), "ERR165");
        assert_eq!(format!("{}", Error::from(general::ERR166)), "ERR166");
        assert_eq!(format!("{}", Error::from(general::ERR167)), "ERR167");
        assert_eq!(format!("{}", Error::from(general::ERR168)), "ERR168");
        assert_eq!(format!("{}", Error::from(general::ERR169)), "ERR169");
        assert_eq!(format!("{}", Error::from(general::ERR170)), "ERR170");
        assert_eq!(format!("{}", Error::from(general::ERR171)), "ERR171");
        assert_eq!(format!("{}", Error::from(general::ERR172)), "ERR172");
        assert_eq!(format!("{}", Error::from(general::ERR173)), "ERR173");
        assert_eq!(format!("{}", Error::from(general::ERR174)), "ERR174");
        assert_eq!(format!("{}", Error::from(general::ERR175)), "ERR175");
        assert_eq!(format!("{}", Error::from(general::ERR176)), "ERR176");
        assert_eq!(format!("{}", Error::from(general::ERR177)), "ERR177");
        assert_eq!(format!("{}", Error::from(general::ERR178)), "ERR178");
        assert_eq!(format!("{}", Error::from(general::ERR179)), "ERR179");
        assert_eq!(format!("{}", Error::from(general::ERR180)), "ERR180");
        assert_eq!(format!("{}", Error::from(general::ERR181)), "ERR181");
        assert_eq!(format!("{}", Error::from(general::ERR182)), "ERR182");
        assert_eq!(format!("{}", Error::from(general::ERR183)), "ERR183");
        assert_eq!(format!("{}", Error::from(general::ERR184)), "ERR184");
        assert_eq!(format!("{}", Error::from(general::ERR185)), "ERR185");
        assert_eq!(format!("{}", Error::from(general::ERR186)), "ERR186");
        assert_eq!(format!("{}", Error::from(general::ERR187)), "ERR187");
        assert_eq!(format!("{}", Error::from(general::ERR188)), "ERR188");
        assert_eq!(format!("{}", Error::from(general::ERR189)), "ERR189");
        assert_eq!(format!("{}", Error::from(general::ERR190)), "ERR190");
        assert_eq!(format!("{}", Error::from(general::ERR191)), "ERR191");
        assert_eq!(format!("{}", Error::from(general::ERR192)), "ERR192");
        assert_eq!(format!("{}", Error::from(general::ERR193)), "ERR193");
        assert_eq!(format!("{}", Error::from(general::ERR194)), "ERR194");
        assert_eq!(format!("{}", Error::from(general::ERR195)), "ERR195");
        assert_eq!(format!("{}", Error::from(general::ERR196)), "ERR196");
        assert_eq!(format!("{}", Error::from(general::ERR197)), "ERR197");
        assert_eq!(format!("{}", Error::from(general::ERR198)), "ERR198");
        assert_eq!(format!("{}", Error::from(general::ERR199)), "ERR199");
        assert_eq!(format!("{}", Error::from(general::ERR200)), "ERR200");
        assert_eq!(format!("{}", Error::from(general::ERR201)), "ERR201");
        assert_eq!(format!("{}", Error::from(general::ERR202)), "ERR202");
        assert_eq!(format!("{}", Error::from(general::ERR203)), "ERR203");
        assert_eq!(format!("{}", Error::from(general::ERR204)), "ERR204");
        assert_eq!(format!("{}", Error::from(general::ERR205)), "ERR205");
        assert_eq!(format!("{}", Error::from(general::ERR206)), "ERR206");
        assert_eq!(format!("{}", Error::from(general::ERR207)), "ERR207");
        assert_eq!(format!("{}", Error::from(general::ERR208)), "ERR208");
        assert_eq!(format!("{}", Error::from(general::ERR209)), "ERR209");
        assert_eq!(format!("{}", Error::from(general::ERR210)), "ERR210");
        assert_eq!(format!("{}", Error::from(general::ERR211)), "ERR211");
        assert_eq!(format!("{}", Error::from(general::ERR212)), "ERR212");
        assert_eq!(format!("{}", Error::from(general::ERR213)), "ERR213");
        assert_eq!(format!("{}", Error::from(general::ERR214)), "ERR214");
        assert_eq!(format!("{}", Error::from(general::ERR215)), "ERR215");
        assert_eq!(format!("{}", Error::from(general::ERR216)), "ERR216");
        assert_eq!(format!("{}", Error::from(general::ERR217)), "ERR217");
        assert_eq!(format!("{}", Error::from(general::ERR218)), "ERR218");
        assert_eq!(format!("{}", Error::from(general::ERR219)), "ERR219");
        assert_eq!(format!("{}", Error::from(general::ERR220)), "ERR220");
        assert_eq!(format!("{}", Error::from(general::ERR221)), "ERR221");
        assert_eq!(format!("{}", Error::from(general::ERR222)), "ERR222");
        assert_eq!(format!("{}", Error::from(general::ERR223)), "ERR223");
        assert_eq!(format!("{}", Error::from(general::ERR224)), "ERR224");
        assert_eq!(format!("{}", Error::from(general::ERR225)), "ERR225");
        assert_eq!(format!("{}", Error::from(general::ERR226)), "ERR226");
        assert_eq!(format!("{}", Error::from(general::ERR227)), "ERR227");
        assert_eq!(format!("{}", Error::from(general::ERR228)), "ERR228");
        assert_eq!(format!("{}", Error::from(general::ERR229)), "ERR229");
        assert_eq!(format!("{}", Error::from(general::ERR230)), "ERR230");
        assert_eq!(format!("{}", Error::from(general::ERR231)), "ERR231");
        assert_eq!(format!("{}", Error::from(general::ERR232)), "ERR232");
        assert_eq!(format!("{}", Error::from(general::ERR233)), "ERR233");
        assert_eq!(format!("{}", Error::from(general::ERR234)), "ERR234");
        assert_eq!(format!("{}", Error::from(general::ERR235)), "ERR235");
        assert_eq!(format!("{}", Error::from(general::ERR236)), "ERR236");
        assert_eq!(format!("{}", Error::from(general::ERR237)), "ERR237");
        assert_eq!(format!("{}", Error::from(general::ERR238)), "ERR238");
        assert_eq!(format!("{}", Error::from(general::ERR239)), "ERR239");
        assert_eq!(format!("{}", Error::from(general::ERR240)), "ERR240");
        assert_eq!(format!("{}", Error::from(general::ERR241)), "ERR241");
        assert_eq!(format!("{}", Error::from(general::ERR242)), "ERR242");
        assert_eq!(format!("{}", Error::from(general::ERR243)), "ERR243");
        assert_eq!(format!("{}", Error::from(general::ERR244)), "ERR244");
        assert_eq!(format!("{}", Error::from(general::ERR245)), "ERR245");
        assert_eq!(format!("{}", Error::from(general::ERR246)), "ERR246");
        assert_eq!(format!("{}", Error::from(general::ERR247)), "ERR247");
        assert_eq!(format!("{}", Error::from(general::ERR248)), "ERR248");
        assert_eq!(format!("{}", Error::from(general::ERR249)), "ERR249");
        assert_eq!(format!("{}", Error::from(general::ERR250)), "ERR250");
        assert_eq!(format!("{}", Error::from(general::ERR251)), "ERR251");
        assert_eq!(format!("{}", Error::from(general::ERR252)), "ERR252");
        assert_eq!(format!("{}", Error::from(general::ERR253)), "ERR253");
        assert_eq!(format!("{}", Error::from(general::ERR254)), "ERR254");
        assert_eq!(format!("{}", Error::from(general::ERR255)), "ERR255");
    }

}

