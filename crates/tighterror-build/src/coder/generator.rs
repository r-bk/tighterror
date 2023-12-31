use crate::{
    coder::{formatter::pretty, idents, options::CodegenOptions},
    errors::{codes::BAD_SPEC, TebError},
    spec::{CategorySpec, Spec},
};
use log::error;
use proc_macro2::{Ident, Literal, TokenStream};
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
    opts: &'a CodegenOptions,
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
    fn new(opts: &'a CodegenOptions, spec: &'a Spec) -> Result<CodeGenerator<'a>, TebError> {
        let n_categories = spec.categories.len();
        let n_category_bits = Self::calc_n_category_bits(n_categories)?;
        let n_error_bits = Self::calc_n_error_bits(spec)?;
        let kind_mask = (1usize << n_error_bits) - 1;
        let category_mask = ((1usize << n_category_bits) - 1) << n_error_bits;
        let repr_type = Self::calc_repr_type(n_category_bits + n_error_bits)?;

        Ok(Self {
            opts,
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
        let test = self.test_tokens();
        let tokens = quote! {
            #mod_doc
            #category_tokens
            #error_code_tokens
            #error_tokens
            #private_modules
            #category_constants
            #error_code_constants
            #test
        };
        pretty(tokens)
    }

    fn private_modules_tokens(&self) -> TokenStream {
        let constants_tokens = self.private_constants_tokens();
        let category_names = self.private_category_names();
        let error_names = self.private_error_names();
        let error_display = self.private_error_display();

        let category_names_mod = Self::category_names_mod_ident();
        let error_names_mod = Self::error_names_mod_ident();
        let error_displays_mod = Self::error_displays_mod_ident();
        let private_mod = Self::private_mod_ident();

        quote! {
            mod #category_names_mod {
                #category_names
            }
            mod #error_names_mod {
                #error_names
            }
            mod #error_displays_mod {
                #error_display
            }
            mod #private_mod {
                #constants_tokens
            }
        }
    }

    fn category_names_mod_ident() -> Ident {
        format_ident!("{}", idents::CATEGORY_NAMES_MOD)
    }

    fn error_names_mod_ident() -> Ident {
        format_ident!("{}", idents::ERROR_NAMES_MOD)
    }

    fn error_displays_mod_ident() -> Ident {
        format_ident!("{}", idents::ERROR_DISPLAYS_MOD)
    }

    fn private_mod_ident() -> Ident {
        format_ident!("{}", idents::PRIVATE_MOD)
    }

    fn err_name_ident(&self) -> Ident {
        format_ident!("{}", self.spec.err_name())
    }

    fn err_cat_name_ident(&self) -> Ident {
        format_ident!("{}", self.spec.err_cat_name())
    }

    fn err_code_name_ident(&self) -> Ident {
        format_ident!("{}", self.spec.err_code_name())
    }

    fn tests_mod_ident(&self) -> Ident {
        format_ident!("{}", idents::TESTS_MOD)
    }

    fn err_codes_mod_ident(&self) -> Ident {
        format_ident!("{}", idents::ERROR_CODES_MOD)
    }

    fn categories_mod_ident(&self) -> Ident {
        format_ident!("{}", idents::CATEGORY_CONSTS_MOD)
    }

    fn n_categories_literal(&self) -> Literal {
        Literal::usize_unsuffixed(self.n_categories)
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
        let err_name = self.err_name_ident();
        let err_code_name = self.err_code_name_ident();
        let err_cat_name = self.err_cat_name_ident();
        let private_mod = Self::private_mod_ident();
        let error_names_mod = Self::error_names_mod_ident();
        let err_code_doc = doc_tokens(self.spec.err_code_doc());
        let category_max_comparison = self.category_max_comparison();
        let err_code_into_result = if self.spec.err_code_into_result() {
            quote! {
                impl<T> core::convert::From<#err_code_name> for Result<T, #err_name> {
                    #[inline]
                    fn from(v: #err_code_name) -> Self {
                        Err(v.into())
                    }
                }
            }
        } else {
            TokenStream::default()
        };
        quote! {
            #err_code_doc
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct #err_code_name(#private_mod::T);

            impl #err_code_name {
                const fn new(cat: #err_cat_name, kind: #private_mod::T) -> Self {
                    Self(cat.0 << #private_mod::KIND_BITS | kind)
                }

                #[inline]
                fn category_value(&self) -> #private_mod::T {
                    self.0 >> #private_mod::KIND_BITS
                }

                #[inline]
                fn kind_value(&self) -> #private_mod::T {
                    self.0 & #private_mod::KIND_MASK
                }

                #[doc = " Returns the error code category."]
                #[inline]
                pub fn category(&self) -> #err_cat_name {
                    #err_cat_name::new(self.category_value())
                }

                #[doc = " Returns the error code name."]
                #[inline]
                pub fn name(&self) -> &'static str {
                    #error_names_mod::A[self.category_value() as usize][self.kind_value() as usize]
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
                        Some(Self::new(#err_cat_name::new(cat), kind))
                    } else {
                        None
                    }
                }
            }

            impl tighterror::TightErrorCode for #err_code_name {
                type ReprType = _p::T;
                type CategoryType = #err_cat_name;
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

            impl core::fmt::Display for #err_code_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.name())
                }
            }

            #err_code_into_result
        }
    }

    fn error_tokens(&self) -> TokenStream {
        let err_name = self.err_name_ident();
        let err_code_name = self.err_code_name_ident();
        let err_doc = doc_tokens(self.spec.err_doc());
        let err_into_result = if self.spec.err_into_result() {
            quote! {
                impl<T> core::convert::From<#err_name> for core::result::Result<T, #err_name> {
                    #[inline]
                    fn from(err: #err_name) -> Self {
                        Err(err)
                    }
                }
            }
        } else {
            TokenStream::default()
        };
        let error_trait = if self.spec.error_trait() {
            quote! {
                impl std::error::Error for #err_name {}
            }
        } else {
            TokenStream::default()
        };
        quote! {
            #err_doc
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct #err_name(#err_code_name);

            impl #err_name {
                #[doc = " Returns the error code."]
                #[inline]
                pub fn code(&self) -> #err_code_name {
                    self.0
                }

                #[doc = " Returns the error origin location."]
                #[inline]
                pub fn location(&self) -> tighterror::Location {
                    tighterror::Location::undefined()
                }
            }

            impl tighterror::TightError for #err_name {
                type ReprType = _p::T;
                type CodeType = #err_code_name;
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

            impl core::convert::From<#err_code_name> for #err_name {
                #[inline]
                fn from(code: #err_code_name) -> Self {
                    Self(code)
                }
            }

            impl core::fmt::Display for #err_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.code().display())
                }
            }

            impl core::cmp::PartialEq for #err_name {
                #[doc = " Checks equality based on the error code only."]
                #[inline]
                fn eq(&self, other: &#err_name) -> bool {
                    self.0 == other.0
                }
            }

            #err_into_result
            #error_trait
        }
    }

    fn error_code_constants_tokens(&self) -> TokenStream {
        let err_code_name = self.err_code_name_ident();
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
                    pub const #err_ident: EC = EC::new(c::#cat_ident, #err_value);
                };
            }
        }

        quote! {
            #[doc = " Error-code constants."]
            pub mod codes {
                use super::#err_code_name as EC;
                use super::categories as c;
                #tokens
            }
        }
    }

    fn category_tokens(&self) -> TokenStream {
        let err_cat_name = self.err_cat_name_ident();
        let cat_doc = doc_tokens(self.spec.cat_doc());
        let category_max_comparison = self.category_max_comparison();
        quote! {
            #cat_doc
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct #err_cat_name(_p::T);

            impl #err_cat_name {
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

            impl tighterror::TightErrorCategory for #err_cat_name {
                type ReprType = _p::T;
                const CATEGORY_BITS: usize = _p::CAT_BITS;
                const KIND_BITS: usize = _p::KIND_BITS;
                const CATEGORIES_COUNT: usize = _p::CAT_COUNT;

                #[inline]
                fn name(&self) -> &'static str {
                    self.name()
                }
            }

            impl core::fmt::Display for #err_cat_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.name())
                }
            }
        }
    }

    fn category_constants_tokens(&self) -> TokenStream {
        let err_cat_name = self.err_cat_name_ident();
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
                use super::#err_cat_name as C;
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

    fn test_tokens(&self) -> TokenStream {
        let tests_mod = self.tests_mod_ident();
        let do_test = self.spec.test(self.opts.test);
        if do_test {
            let test_tokens = self.test_tokens_impl();
            quote! {
                #[cfg(test)]
                mod #tests_mod {
                    use super::*;
                    #test_tokens
                }
            }
        } else {
            TokenStream::default()
        }
    }

    fn test_tokens_impl(&self) -> TokenStream {
        let ut_category_name = self.ut_category_name_tokens();
        let ut_category_display = self.ut_category_display();
        let ut_category_uniqueness = self.ut_category_uniqueness();
        let ut_category_values = self.ut_category_values();
        let ut_err_code_name = self.ut_err_code_name();
        let ut_err_code_display = self.ut_err_code_display();
        let ut_err_code_uniqueness = self.ut_err_code_uniqueness();
        let ut_err_code_value_uniqueness = self.ut_err_code_value_uniqueness();
        let ut_err_code_category = self.ut_err_code_category();
        let ut_err_code_from_value = self.ut_err_code_from_value();
        let ut_err_display = self.ut_err_display();

        quote! {
            #ut_category_name
            #ut_category_display
            #ut_category_uniqueness
            #ut_category_values
            #ut_err_code_name
            #ut_err_code_display
            #ut_err_code_uniqueness
            #ut_err_code_value_uniqueness
            #ut_err_code_category
            #ut_err_code_from_value
            #ut_err_display
        }
    }

    fn ut_category_name_tokens(&self) -> TokenStream {
        let categories_mod = self.categories_mod_ident();
        let check_cat_name_iter = self.spec.categories.iter().map(|c| {
            let ident_name = c.ident_name();
            let ident = format_ident!("{}", ident_name);
            quote! {
                assert_eq!(#ident.name(), #ident_name);
                assert_eq!(tighterror::TightErrorCategory::name(&#ident), #ident_name)
            }
        });
        quote! {
            #[test]
            fn test_category_name() {
                use #categories_mod::*;
                #(#check_cat_name_iter);*
            }
        }
    }

    fn ut_category_display(&self) -> TokenStream {
        let categories_mod = self.categories_mod_ident();
        let check_cat_display_iter = self.spec.categories.iter().map(|c| {
            let ident_name = c.ident_name();
            let ident = format_ident!("{}", ident_name);
            quote! {
                assert_eq!(format!("{}", #ident), #ident_name);
            }
        });
        quote! {
            #[test]
            fn test_category_display() {
                use #categories_mod::*;
                #(#check_cat_display_iter)*
            }
        }
    }

    fn ut_category_uniqueness(&self) -> TokenStream {
        let err_cat_name = self.err_cat_name_ident();
        let categories_mod = self.categories_mod_ident();
        let cat_arr = self.ut_cat_arr();
        let n_categories = self.n_categories_literal();
        quote! {
            #[test]
            fn test_category_uniqueness() {
                use #categories_mod::*;
                use std::collections::HashSet;
                let cats: [#err_cat_name; #n_categories] = #cat_arr;
                let set = HashSet::<#err_cat_name>::from_iter(cats);
                assert_eq!(set.len(), #n_categories);
            }
        }
    }

    fn ut_category_values(&self) -> TokenStream {
        let err_cat_name = self.err_cat_name_ident();
        let categories_mod = self.categories_mod_ident();
        let cat_arr = self.ut_cat_arr();
        let n_categories = Literal::usize_unsuffixed(self.spec.categories.len());
        let category_max = Literal::usize_unsuffixed(self.spec.categories.len() - 1);
        let comparison = self.category_max_comparison();
        quote! {
            #[test]
            fn test_category_values() {
                use #categories_mod::*;
                let cats: [#err_cat_name; #n_categories] = #cat_arr;
                for c in cats {
                    assert!(c.0 #comparison #category_max);
                }
            }
        }
    }

    fn ut_cat_arr(&self) -> TokenStream {
        let cat_iter = self
            .spec
            .categories
            .iter()
            .map(|c| format_ident!("{}", c.ident_name()));
        quote! {
            [#(#cat_iter),*]
        }
    }

    fn ut_err_code_name(&self) -> TokenStream {
        let err_codes_mod = self.err_codes_mod_ident();
        let iter = self.spec.categories.iter().map(|c| {
            let ec_iter = c.errors.iter().map(|e| {
                let ident_name = e.ident_name();
                let ident = format_ident!("{}", ident_name);
                quote! {
                    assert_eq!(#ident.name(), #ident_name);
                    assert_eq!(tighterror::TightErrorCode::name(&#ident), #ident_name);
                }
            });
            quote! {
                #(#ec_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_code_name() {
                use #err_codes_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_code_display(&self) -> TokenStream {
        let err_codes_mod = self.err_codes_mod_ident();
        let iter = self.spec.categories.iter().map(|c| {
            let ec_iter = c.errors.iter().map(|e| {
                let ident_name = e.ident_name();
                let ident = format_ident!("{}", ident_name);
                quote! {
                    assert_eq!(format!("{}", #ident), #ident_name);
                }
            });
            quote! {
                #(#ec_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_code_display() {
                use #err_codes_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_code_arr(&self) -> TokenStream {
        let iter = self.spec.categories.iter().map(|c| {
            let eiter = c.errors.iter().map(|e| format_ident!("{}", e.ident_name()));
            quote! {
                #(#eiter),*
            }
        });

        quote! {
            [#(#iter),*]
        }
    }

    fn ut_err_code_uniqueness(&self) -> TokenStream {
        let err_code_name = self.err_code_name_ident();
        let err_codes_mod = self.err_codes_mod_ident();
        let err_code_arr = self.ut_err_code_arr();
        let n_errors =
            Literal::usize_unsuffixed(self.spec.categories.iter().map(|c| c.errors.len()).sum());
        quote! {
            #[test]
            fn test_err_code_uniqueness() {
                use #err_codes_mod::*;
                use std::collections::HashSet;
                let errs: [#err_code_name; #n_errors] = #err_code_arr;
                let set = HashSet::<#err_code_name>::from_iter(errs);
                assert_eq!(set.len(), #n_errors);
            }
        }
    }

    fn ut_err_code_value_uniqueness(&self) -> TokenStream {
        let err_code_name = self.err_code_name_ident();
        let err_codes_mod = self.err_codes_mod_ident();
        let repr_type = self.repr_type.ident();
        let err_code_arr = self.ut_err_code_arr();
        let n_errors =
            Literal::usize_unsuffixed(self.spec.categories.iter().map(|c| c.errors.len()).sum());
        quote! {
            #[test]
            fn test_err_code_value_uniqueness() {
                use #err_codes_mod::*;
                use std::collections::HashSet;
                let errs: [#err_code_name; #n_errors] = #err_code_arr;
                let set = HashSet::<#repr_type>::from_iter(errs.iter().map(|ec| ec.value()));
                assert_eq!(set.len(), #n_errors);
            }
        }
    }

    fn ut_err_code_category(&self) -> TokenStream {
        let categories_mod = self.categories_mod_ident();
        let err_codes_mod = self.err_codes_mod_ident();
        let iter = self.spec.categories.iter().map(|c| {
            let eiter = c.errors.iter().map(|e| {
                let ident = format_ident!("{}", e.ident_name());
                let cat_ident = format_ident!("{}", c.ident_name());
                quote! {
                    assert_eq!(#ident.category(), #categories_mod::#cat_ident);
                }
            });
            quote! {
                #(#eiter)*
            }
        });
        quote! {
            #[test]
            fn test_err_code_category() {
                use #err_codes_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_code_from_value(&self) -> TokenStream {
        let err_code_name = self.err_code_name_ident();
        let err_codes_mod = self.err_codes_mod_ident();
        let iter = self.spec.categories.iter().map(|c| {
            let eiter = c.errors.iter().map(|e| {
                let ident = format_ident!("{}", e.ident_name());
                quote! {
                    assert_eq!(#err_code_name::from_value(#ident.value()).unwrap(), #ident);
                }
            });
            quote! {
                #(#eiter)*
            }
        });
        quote! {
            #[test]
            fn test_err_code_from_value() {
                use #err_codes_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_display(&self) -> TokenStream {
        let err_name = self.err_name_ident();
        let err_codes_mod = self.err_codes_mod_ident();
        let iter = self.spec.categories.iter().map(|c| {
            let eiter = c.errors.iter().map(|e| {
                let eident = format_ident!("{}", e.ident_name());
                let display = if let Some(ref d) = e.display {
                    d.clone()
                } else {
                    e.ident_name()
                };
                quote! {
                    assert_eq!(format!("{}", #err_name::from(#eident)), #display);
                }
            });
            quote! {
                #(#eiter)*
            }
        });
        quote! {
            #[test]
            fn test_err_display() {
                use #err_codes_mod::*;
                #(#iter)*
            }
        }
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

pub fn spec2code(opts: &CodegenOptions, spec: &Spec) -> Result<String, TebError> {
    let gen = CodeGenerator::new(opts, spec)?;
    gen.code()
}
