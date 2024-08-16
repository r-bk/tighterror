use crate::errors::{kind::coder::TOO_MANY_BITS, TbError};
use std::any::type_name;

type LargestReprType = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReprType {
    U8,
    U16,
    U32,
    U64,
}

impl ReprType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
        }
    }

    pub fn ident(&self) -> syn::Ident {
        quote::format_ident!("{}", self.name())
    }

    pub fn bits(&self) -> usize {
        match self {
            Self::U8 => u8::BITS as usize,
            Self::U16 => u16::BITS as usize,
            Self::U32 => u32::BITS as usize,
            Self::U64 => u64::BITS as usize,
        }
    }

    pub fn from_n_bits(n_bits: usize) -> Result<Self, TbError> {
        Ok(match n_bits {
            0..=8 => ReprType::U8,
            9..=16 => ReprType::U16,
            17..=32 => ReprType::U32,
            33..=64 => ReprType::U64,
            _ => return TOO_MANY_BITS.into(),
        })
    }

    pub fn largest_type_name() -> &'static str {
        type_name::<LargestReprType>()
    }
}
