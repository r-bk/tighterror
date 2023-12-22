use crate::{
    coder::formatter::pretty,
    errors::{TebError, BAD_SPEC},
    spec::{CategorySpec, Spec},
};
use log::error;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::{num::TryFromIntError, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ReprType {
    U8,
    U16,
    U32,
    U64,
}

#[allow(dead_code)]
struct CodeGenerator<'a> {
    spec: &'a Spec,
    /// total number of categories
    n_categories: usize,
    /// number of bits required for categories
    n_category_bits: usize,
    /// number of bits required for errors
    n_error_bits: usize,
    /// the representation type
    repr_type: ReprType,
    /// the mask of kind bits
    kind_mask: usize,
    /// the mask of category bits (shifted)
    category_mask: usize,
}

impl<'a> CodeGenerator<'a> {
    fn new(spec: &'a Spec) -> Result<CodeGenerator<'a>, TebError> {
        let n_categories = spec.categories.len();
        let n_category_bits = Self::calc_n_category_bits(n_categories)?;
        let n_error_bits = Self::calc_n_error_bits(spec)?;
        let kind_mask = (1usize << n_error_bits) - 1;
        let category_mask = ((1usize << n_category_bits) - 1) << n_error_bits;
        let repr_type = Self::calc_repr_type(n_category_bits + n_error_bits)?;

        Ok(Self {
            spec,
            n_categories,
            n_category_bits,
            n_error_bits,
            repr_type,
            kind_mask,
            category_mask,
        })
    }

    fn calc_repr_type(n_bits: usize) -> Result<ReprType, TebError> {
        match n_bits {
            1..=8 => Ok(ReprType::U8),
            9..=16 => Ok(ReprType::U16),
            17..=32 => Ok(ReprType::U32),
            33..=64 => Ok(ReprType::U64),
            _ => {
                error!("repr_type: bad number of bits: {n_bits}");
                BAD_SPEC.into()
            }
        }
    }

    fn calc_n_category_bits(n_categories: usize) -> Result<usize, TebError> {
        match n_categories {
            0 => {
                error!("at least one category must be defined");
                BAD_SPEC.into()
            }
            1 => Ok(0),
            n => Self::calc_n_bits(n, "categories"),
        }
    }

    fn calc_n_error_bits(spec: &Spec) -> Result<usize, TebError> {
        let n = match spec.n_errors_in_largest_category() {
            Some(n) => n,
            None => {
                error!("at least one category must be defined");
                return BAD_SPEC.into();
            }
        };

        match n {
            0 => {
                error!("at least one error must be defined");
                BAD_SPEC.into()
            }
            n => Self::calc_n_bits(n, "errors"),
        }
    }

    fn calc_n_bits(n: usize, name: &str) -> Result<usize, TebError> {
        if let Some(po2) = n.checked_next_power_of_two() {
            Ok(usize::try_from(po2.trailing_zeros()).unwrap())
        } else {
            error!("too many {name}: {n}");
            BAD_SPEC.into()
        }
    }

    fn code(&self) -> Result<String, TebError> {
        let mod_doc = outer_doc_tokens(self.spec.mod_doc());
        let private_modules = self.private_modules_tokens();
        let category_tokens = self.category_tokens();
        let error_code_tokens = self.error_code_tokens();
        let error_tokens = self.error_tokens();
        let category_constants = self.category_constants_tokens();
        let error_code_constants = self.error_code_constants_tokens();
        let tokens = quote! {
            #mod_doc
            #category_tokens
            #error_code_tokens
            #error_tokens
            #private_modules
            #category_constants
            #error_code_constants
        };
        pretty(tokens)
    }

    fn private_modules_tokens(&self) -> TokenStream {
        let constants_tokens = self.private_constants_tokens();
        let category_names = self.private_category_names();
        let error_names = self.private_error_names();
        let error_display = self.private_error_display();
        quote! {
            mod _cn {
                #category_names
            }
            mod _n {
                #error_names
            }
            mod _d {
                #error_display
            }
            mod _p {
                #constants_tokens
            }
        }
    }

    fn private_constants_tokens(&self) -> TokenStream {
        let repr_type = self.repr_type.ident();
        let n_category_bits = Literal::usize_unsuffixed(self.n_category_bits);
        let n_error_bits = Literal::usize_unsuffixed(self.n_error_bits);
        let n_categories = Literal::usize_unsuffixed(self.spec.categories.len());
        let kind_mask = self.usize_to_repr_type_literal(self.kind_mask).unwrap();
        let category_mask = self.usize_to_repr_type_literal(self.category_mask).unwrap();
        let category_max = self
            .usize_to_repr_type_literal(self.spec.category_max())
            .unwrap();
        let kind_max = |c: &CategorySpec| {
            let n_errors = c.errors.len();
            self.usize_to_repr_type_literal(n_errors.checked_sub(1).unwrap())
                .unwrap()
        };
        let kind_maxes_iter = self.spec.categories.iter().map(kind_max);

        let mut tokens = quote! {
            pub type T = #repr_type;
            pub const CAT_BITS: usize = #n_category_bits;
            pub const CAT_COUNT: usize = #n_categories;
            pub const CAT_MASK: T = #category_mask;
            pub const CAT_MAX: T = #category_max;
            pub const KIND_BITS: usize = #n_error_bits;
            pub const KIND_MASK: T = #kind_mask;
            pub static KIND_MAXES: [T; #n_categories] = [
                #(#kind_maxes_iter),*
            ];
            const _: () = assert!(CAT_BITS + KIND_BITS <= T::BITS as usize);
            const _: () = assert!(CAT_COUNT <= i16::MAX as usize);
        };

        let cat_limit_literal = Literal::i16_unsuffixed(i16::MAX);
        if self.repr_type > ReprType::U8 {
            tokens = quote! {
                #tokens
                const _: () = assert!(CAT_MAX <= #cat_limit_literal);
            };
        }

        tokens
    }

    fn private_category_names(&self) -> TokenStream {
        let n_categories = Literal::usize_unsuffixed(self.spec.categories.len());
        let category_consts_iter = self
            .spec
            .categories
            .iter()
            .map(|c| format_ident!("{}", c.ident_name()));
        let mut tokens = TokenStream::default();
        for c in &self.spec.categories {
            let iname = c.ident_name();
            let const_name = format_ident!("{}", iname);
            tokens = quote! {
                #tokens
                pub const #const_name: &str = #iname;
            }
        }
        quote! {
            #tokens
            pub static A: [&str; #n_categories] = [
                #(#category_consts_iter),*
            ];
        }
    }

    fn private_error_names(&self) -> TokenStream {
        let mut tokens = TokenStream::default();
        for c in &self.spec.categories {
            for e in &c.errors {
                let ident = e.ident_name();
                let const_ident = format_ident!("{}", ident);
                tokens = quote! {
                    #tokens
                    pub const #const_ident: &str = #ident;
                };
            }
        }

        for c in &self.spec.categories {
            let ident = format_ident!("{}", category_errors_constant_name(c));
            let n_errors = Literal::usize_unsuffixed(c.errors.len());
            let error_consts_iter = c.errors.iter().map(|e| format_ident!("{}", e.ident_name()));
            tokens = quote! {
                #tokens
                pub static #ident: [&str; #n_errors] = [
                    #(#error_consts_iter),*
                ];
            };
        }

        let n_categories = Literal::usize_unsuffixed(self.spec.categories.len());
        let category_errors_contant_ident_iter = self.spec.categories.iter().map(|c| {
            let ident = format_ident!("{}", category_errors_constant_name(c));
            quote! { &#ident }
        });
        tokens = quote! {
            #tokens
            pub static A: [&[&str]; #n_categories] = [
                #(#category_errors_contant_ident_iter),*
            ];
        };
        tokens
    }

    fn private_error_display(&self) -> TokenStream {
        let mut tokens = TokenStream::default();
        for c in &self.spec.categories {
            for e in &c.errors {
                let ident = format_ident!("{}", e.ident_name());
                let display = self.spec.error_display(c, e);
                tokens = quote! {
                    #tokens
                    pub const #ident: &str = #display;
                };
            }
        }

        for c in &self.spec.categories {
            let ident = format_ident!("{}", category_error_code_display(c));
            let n_errors = Literal::usize_unsuffixed(c.errors.len());
            let error_ident_iter = c.errors.iter().map(|e| format_ident!("{}", e.ident_name()));
            tokens = quote! {
                #tokens
                pub static #ident: [&str; #n_errors] = [
                    #(#error_ident_iter),*
                ];
            };
        }

        let n_categories = Literal::usize_unsuffixed(self.spec.categories.len());
        let category_error_code_display_ident_iter = self.spec.categories.iter().map(|c| {
            let ident = format_ident!("{}", category_error_code_display(c));
            quote! { &#ident }
        });
        tokens = quote! {
            #tokens
            pub static A: [&[&str]; #n_categories] = [
                #(#category_error_code_display_ident_iter),*
            ];
        };
        tokens
    }

    fn error_code_tokens(&self) -> TokenStream {
        let err_code_doc = doc_tokens(self.spec.err_code_doc());
        let category_max_comparison = self.category_max_comparison();
        quote! {
            #err_code_doc
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct ErrorCode(_p::T);

            impl ErrorCode {
                const fn new(cat: ErrorCategory, kind: _p::T) -> Self {
                    Self(cat.0 << _p::KIND_BITS | kind)
                }

                #[inline]
                fn category_value(&self) -> _p::T {
                    self.0 >> _p::KIND_BITS
                }

                #[inline]
                fn kind_value(&self) -> _p::T {
                    self.0 & _p::KIND_MASK
                }

                #[doc = " Returns the error code category."]
                #[inline]
                pub fn category(&self) -> ErrorCategory {
                    ErrorCategory::new(self.category_value())
                }

                #[doc = " Returns the error code name."]
                #[inline]
                pub fn name(&self) -> &'static str {
                    _n::A[self.category_value() as usize][self.kind_value() as usize]
                }

                #[inline]
                fn display(&self) -> &'static str {
                    _d::A[self.category_value() as usize][self.kind_value() as usize]
                }

                #[doc = " Returns the error code value as the underlying Rust type."]
                #[inline]
                pub fn value(&self) -> _p::T {
                    self.0
                }

                #[doc = " Creates an error code from a raw value of the underlying Rust type."]
                #[inline]
                pub fn from_value(value: _p::T) -> Option<Self> {
                    let cat = (value & _p::CAT_MASK) >> _p::KIND_BITS;
                    let kind = value & _p::KIND_MASK;
                    if cat #category_max_comparison _p::CAT_MAX && kind <= _p::KIND_MAXES[cat as usize] {
                        Some(Self::new(ErrorCategory::new(cat), kind))
                    } else {
                        None
                    }
                }
            }

            impl tighterror::TightErrorCode for ErrorCode {
                type ReprType = _p::T;
                type CategoryType = ErrorCategory;
                const CATEGORY_BITS: usize = _p::CAT_BITS;
                const KIND_BITS: usize = _p::KIND_BITS;
                const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

                #[inline]
                fn category(&self) -> Self::CategoryType {
                    self.category()
                }

                #[inline]
                fn name(&self) -> &'static str {
                    self.name()
                }

                #[inline]
                fn value(&self) -> Self::ReprType {
                    self.value()
                }

                #[inline]
                fn from_value(value: Self::ReprType) -> Option<Self> {
                    Self::from_value(value)
                }
            }

            impl core::fmt::Display for ErrorCode {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.name())
                }
            }
        }
    }

    fn error_tokens(&self) -> TokenStream {
        let err_doc = doc_tokens(self.spec.err_doc());
        quote! {
            #err_doc
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Error(ErrorCode);

            impl Error {
                #[doc = " Returns the error code."]
                #[inline]
                pub fn code(&self) -> ErrorCode {
                    self.0
                }

                #[doc = " Returns the error origin location."]
                #[inline]
                pub fn location(&self) -> tighterror::Location {
                    tighterror::Location::undefined()
                }
            }

            impl tighterror::TightError for Error {
                type ReprType = _p::T;
                type CodeType = ErrorCode;
                const CATEGORY_BITS: usize = _p::CAT_BITS;
                const KIND_BITS: usize = _p::KIND_BITS;
                const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

                #[inline]
                fn code(&self) -> Self::CodeType {
                    self.code()
                }

                #[inline]
                fn location(&self) -> tighterror::Location {
                    self.location()
                }
            }

            impl core::convert::From<ErrorCode> for Error {
                #[inline]
                fn from(code: ErrorCode) -> Self {
                    Self(code)
                }
            }

            impl core::fmt::Display for Error {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.code().display())
                }
            }
        }
    }

    fn error_code_constants_tokens(&self) -> TokenStream {
        let mut tokens = TokenStream::default();
        for c in &self.spec.categories {
            for (i, e) in c.errors.iter().enumerate() {
                let cat_ident = format_ident!("{}", c.ident_name());
                let err_value = self.usize_to_repr_type_literal(i).unwrap();
                let err_ident = format_ident!("{}", e.ident_name());
                let err_doc = doc_tokens(self.spec.err_code_const_doc(c, e));
                tokens = quote! {
                    #tokens

                    #err_doc
                    pub const #err_ident: E = E::new(c::#cat_ident, #err_value);
                };
            }
        }

        quote! {
            #[doc = " Error code constants."]
            pub mod codes {
                use super::ErrorCode as E;
                use super::categories as c;
                #tokens
            }
        }
    }

    fn category_tokens(&self) -> TokenStream {
        let cat_doc = doc_tokens(self.spec.cat_doc());
        let category_max_comparison = self.category_max_comparison();
        quote! {
            #cat_doc
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct ErrorCategory(_p::T);

            impl ErrorCategory {
                #[inline]
                const fn new(v: _p::T) -> Self {
                    debug_assert!(v #category_max_comparison _p::CAT_MAX);
                    Self(v)
                }

                #[doc = " Returns the name of the error category."]
                #[inline]
                pub fn name(&self) -> &'static str {
                    _cn::A[self.0 as usize]
                }
            }

            impl tighterror::TightErrorCategory for ErrorCategory {
                type ReprType = _p::T;
                const CATEGORY_BITS: usize = _p::CAT_BITS;
                const KIND_BITS: usize = _p::KIND_BITS;
                const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

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
        }
    }

    fn category_constants_tokens(&self) -> TokenStream {
        let mut tokens = quote! {};
        for (i, c) in self.spec.categories.iter().enumerate() {
            let cat_value = self.usize_to_repr_type_literal(i).unwrap();
            let cat_name_upper_snake = format_ident!("{}", c.ident_name());
            let const_doc = doc_tokens(self.spec.cat_const_doc(c));
            let single = quote! {
                #const_doc
                pub const #cat_name_upper_snake: C = C::new(#cat_value);
            };
            tokens = quote! {
                #tokens
                #single
            };
        }

        quote! {
            #[doc = " Error category constants."]
            pub mod categories {
                use super::ErrorCategory as C;
                #tokens
            }
        }
    }

    fn usize_to_repr_type_literal(&self, v: usize) -> Result<Literal, TryFromIntError> {
        match self.repr_type {
            ReprType::U8 => {
                let v: u8 = u8::try_from(v)?;
                Ok(Literal::u8_unsuffixed(v))
            }
            ReprType::U16 => {
                let v: u16 = u16::try_from(v)?;
                Ok(Literal::u16_unsuffixed(v))
            }
            ReprType::U32 => {
                let v: u32 = u32::try_from(v)?;
                Ok(Literal::u32_unsuffixed(v))
            }
            ReprType::U64 => {
                let v: u64 = u64::try_from(v)?;
                Ok(Literal::u64_unsuffixed(v))
            }
        }
    }

    fn category_max_comparison(&self) -> TokenStream {
        let category_max = self.spec.category_max();
        TokenStream::from_str(if category_max == 0 { "==" } else { "<=" }).unwrap()
    }
}

fn category_errors_constant_name(c: &CategorySpec) -> String {
    format!("{}__NAMES", c.ident_name())
}

fn category_error_code_display(c: &CategorySpec) -> String {
    format!("{}__DISPLAY", c.ident_name())
}

fn _doc_tokens(doc: &str, outer: bool) -> TokenStream {
    if doc.is_empty() {
        quote! {}
    } else {
        let doc = if doc.starts_with(' ') {
            doc.to_owned()
        } else {
            format!(" {}", doc)
        };
        if outer {
            quote! {
                #![doc = #doc]
            }
        } else {
            quote! {
                #[doc = #doc]
            }
        }
    }
}

fn doc_tokens(doc: &str) -> TokenStream {
    const OUTER: bool = false;
    _doc_tokens(doc, OUTER)
}

fn outer_doc_tokens(doc: &str) -> TokenStream {
    const OUTER: bool = true;
    _doc_tokens(doc, OUTER)
}

impl ReprType {
    fn name(&self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
        }
    }

    fn ident(&self) -> syn::Ident {
        format_ident!("{}", self.name())
    }
}

pub fn spec2code(spec: &Spec) -> Result<String, TebError> {
    let gen = CodeGenerator::new(spec)?;
    gen.code()
}
