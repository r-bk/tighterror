use crate::{
    coder::generator::{bits::Bits, helpers::*, repr_type::ReprType},
    errors::TbError,
    spec::{CategorySpec, ErrorSpec, ModuleSpec, Spec},
    FrozenOptions,
};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{num::TryFromIntError, str::FromStr};

pub struct ModuleGenerator<'a> {
    opts: &'a FrozenOptions,
    spec: &'a Spec,
    module: &'a ModuleSpec,
    /// add a module doc string to the generated code
    mod_doc: bool,
    /// bit calculations' results
    bits: Bits,
}

impl<'a> ModuleGenerator<'a> {
    pub fn new(
        opts: &'a FrozenOptions,
        spec: &'a Spec,
        module: &'a ModuleSpec,
        mod_doc: bool,
    ) -> Result<ModuleGenerator<'a>, TbError> {
        Ok(Self {
            opts,
            spec,
            module,
            mod_doc,
            bits: Bits::calculate(spec, module)?,
        })
    }

    pub fn rust(&self) -> Result<TokenStream, TbError> {
        let module_doc = self.module_doc_tokens();
        let private_modules = self.private_modules_tokens();
        let category_tokens = self.category_tokens();
        let error_kind_tokens = self.error_kind_tokens();
        let error_tokens = self.error_tokens();
        let category_constants = self.category_constants_tokens();
        let error_kind_constants = self.error_kind_constants_tokens();
        let variants_module = self.variants_module_tokens();
        let test = self.test_tokens();
        Ok(quote! {
            #module_doc
            #category_tokens
            #error_kind_tokens
            #error_tokens
            #private_modules
            #category_constants
            #error_kind_constants
            #variants_module
            #test
        })
    }

    fn private_modules_tokens(&self) -> TokenStream {
        let constants_tokens = self.private_constants_tokens();
        let types = self.private_types();
        let category_names = self.private_category_names();
        let error_names = self.private_error_names();
        let error_display = self.private_error_display();

        let category_names_mod = category_names_mod_ident();
        let error_names_mod = error_names_mod_ident();
        let error_displays_mod = error_displays_mod_ident();
        let private_mod = private_mod_ident();

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
                #types
            }
        }
    }

    fn private_constants_tokens(&self) -> TokenStream {
        let repr_type = self.bits.repr_type.ident();
        let n_kind_bits = Literal::usize_unsuffixed(self.bits.kind);
        let n_category_bits = Literal::usize_unsuffixed(self.bits.category);
        let n_variant_bits = Literal::usize_unsuffixed(self.bits.variant);
        let n_categories = Literal::usize_unsuffixed(self.module.categories.len());
        let variant_mask = self
            .u64_to_repr_type_literal(self.bits.variant_mask)
            .unwrap();
        let category_mask = self
            .u64_to_repr_type_literal(self.bits.category_mask)
            .unwrap();
        let category_max = self
            .usize_to_repr_type_literal(self.module.category_max())
            .unwrap();
        let variant_max = |c: &CategorySpec| {
            let n_errors = c.errors.len();
            self.usize_to_repr_type_literal(n_errors.checked_sub(1).unwrap())
                .unwrap()
        };
        let variant_maxes_iter = self.module.categories.iter().map(variant_max);

        let optional_tokens = if self.bits.category == 0 {
            quote! {}
        } else {
            quote! {
                pub const CAT_MASK: R = #category_mask;
                pub const VAR_BITS: usize = #n_variant_bits;
            }
        };

        quote! {
            pub type R = #repr_type;
            pub const KIND_BITS: usize = #n_kind_bits;
            pub const CAT_BITS: usize = #n_category_bits;
            pub const CAT_MAX: R = #category_max;
            pub const VAR_MASK: R = #variant_mask;
            pub static VAR_MAXES: [R; #n_categories] = [
                #(#variant_maxes_iter),*
            ];
            #optional_tokens
            const _: () = assert!(KIND_BITS <= R::BITS as usize);
            const _: () = assert!(CAT_BITS <= usize::BITS as usize); // for casting to usize
        }
    }

    fn private_types(&self) -> TokenStream {
        quote! {
            pub(super) struct Ident<'a>(pub(super) &'a str);
            impl core::fmt::Debug for Ident<'_> {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.0)
                }
            }
        }
    }

    fn private_category_names(&self) -> TokenStream {
        let n_categories = Literal::usize_unsuffixed(self.module.categories.len());
        let category_consts_iter = self
            .module
            .categories
            .iter()
            .map(|c| format_ident!("{}", c.ident_name()));
        let mut tokens = TokenStream::default();
        for c in &self.module.categories {
            let ident_name = c.ident_name();
            let const_name = format_ident!("{}", ident_name);
            tokens = quote! {
                #tokens
                pub const #const_name: &str = #ident_name;
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
        let cat_iter = self
            .module
            .categories
            .iter()
            .map(|c| self.private_category_error_names(c));
        let cat_arr_iter = self.module.categories.iter().map(|c| {
            let cat_mod_ident = format_ident!("{}", c.module_name());
            quote! { &#cat_mod_ident::A }
        });
        let n_categories = Literal::usize_unsuffixed(self.module.categories.len());
        quote! {
            #(#cat_iter)*
            pub static A: [&[&str]; #n_categories] = [
                #(#cat_arr_iter),*
            ];
        }
    }

    fn private_category_error_names(&self, c: &CategorySpec) -> TokenStream {
        let cat_mod_ident = format_ident!("{}", c.module_name());
        let const_iter = c.errors.iter().map(|e| {
            let name = e.name.as_str();
            let const_ident = format_ident!("{}", name);
            quote! {
                pub(crate) const #const_ident: &str = #name
            }
        });
        let arr_iter = c.errors.iter().map(|e| {
            let const_ident = format_ident!("{}", e.name);
            quote! { #const_ident }
        });
        let n_errors = Literal::usize_unsuffixed(c.errors.len());
        quote! {
            pub(crate) mod #cat_mod_ident {
                #(#const_iter);* ;

                pub static A: [&str; #n_errors] = [
                    #(#arr_iter),*
                ];
            }
        }
    }

    fn private_error_display(&self) -> TokenStream {
        let cat_iter = self
            .module
            .categories
            .iter()
            .map(|c| self.private_category_error_display(c));
        let cat_arr_iter = self.module.categories.iter().map(|c| {
            let cat_mod_ident = format_ident!("{}", c.module_name());
            quote! { &#cat_mod_ident::A }
        });
        let n_categories = Literal::usize_unsuffixed(self.module.categories.len());
        quote! {
            #(#cat_iter)*
            pub static A: [&[&str]; #n_categories] = [
                #(#cat_arr_iter),*
            ];
        }
    }

    fn private_category_error_display(&self, c: &CategorySpec) -> TokenStream {
        let cat_mod_ident = format_ident!("{}", c.module_name());
        let error_names_mod = error_names_mod_ident();
        let add_names_mod_import = c.errors.iter().any(|e| e.display.is_none());
        let const_iter = c.errors.iter().map(|e| {
            let const_ident = format_ident!("{}", e.name);
            if let Some(ref display) = e.display {
                quote! {
                    pub(crate) const #const_ident: &str = #display
                }
            } else {
                quote! {
                    const #const_ident: &str = #error_names_mod::#cat_mod_ident::#const_ident
                }
            }
        });
        let arr_iter = c.errors.iter().map(|e| {
            let const_ident = format_ident!("{}", e.name);
            quote! { #const_ident }
        });
        let n_errors = Literal::usize_unsuffixed(c.errors.len());

        let names_mod_import = if add_names_mod_import {
            quote! { use super::super::#error_names_mod; }
        } else {
            TokenStream::default()
        };

        quote! {
            pub(crate) mod #cat_mod_ident {
                #names_mod_import

                #(#const_iter);* ;

                pub static A: [&str; #n_errors] = [
                    #(#arr_iter),*
                ];
            }
        }
    }

    fn category_tokens(&self) -> TokenStream {
        let err_cat_name = self.err_cat_name_ident();
        let err_cat_name_str = self.module.err_cat_name();
        let err_cat_doc = doc_tokens(self.module.err_cat_doc());
        let category_names_mod = category_names_mod_ident();
        let private_mod = private_mod_ident();
        quote! {
            #err_cat_doc
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct #err_cat_name(#private_mod::R);

            impl #err_cat_name {
                #[inline]
                const fn new(v: #private_mod::R) -> Self {
                    Self(v)
                }

                #[doc = " Returns the name of the error category."]
                #[inline]
                pub fn name(&self) -> &'static str {
                    #category_names_mod::A[self.0 as usize]
                }
            }

            impl tighterror::Category for #err_cat_name {
                type R = #private_mod::R;
                const BITS: usize = #private_mod::CAT_BITS;

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

            impl core::fmt::Debug for #err_cat_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_tuple(#err_cat_name_str)
                        .field(&#private_mod::Ident(self.name()))
                        .finish()
                }
            }
        }
    }

    fn error_kind_tokens(&self) -> TokenStream {
        let err_name = self.err_name_ident();
        let err_kind_name = self.err_kind_name_ident();
        let err_kind_name_str = self.module.err_kind_name();
        let err_cat_name = self.err_cat_name_ident();
        let private_mod = private_mod_ident();
        let error_names_mod = error_names_mod_ident();
        let error_displays_mod = error_displays_mod_ident();
        let err_kind_doc = doc_tokens(self.module.err_kind_doc());
        let category_max_comparison = self.category_max_comparison();
        let result_from_err_kind = if self.module.result_from_err_kind() {
            quote! {
                impl<T> core::convert::From<#err_kind_name> for Result<T, #err_name> {
                    #[inline]
                    fn from(v: #err_kind_name) -> Self {
                        Err(v.into())
                    }
                }
            }
        } else {
            TokenStream::default()
        };

        let err_kind_new_tokens = if self.bits.category == 0 {
            quote! { assert!(cat.0 == #private_mod::CAT_MAX); Self(variant) }
        } else {
            quote! { Self(cat.0 << #private_mod::VAR_BITS | variant) }
        };

        let cat_value_tokens = if self.bits.category == 0 {
            quote! { #private_mod::CAT_MAX }
        } else {
            quote! { (self.0 & #private_mod::CAT_MASK) >> #private_mod::VAR_BITS }
        };

        let from_value_tokens = if self.bits.category == 0 {
            quote! {
                if value <= #private_mod::VAR_MAXES[0] {
                    Some(Self::new(#err_cat_name::new(0), value))
                } else {
                    None
                }
            }
        } else {
            quote! {
                let cat = (value & #private_mod::CAT_MASK) >> #private_mod::VAR_BITS;
                let variant = value & #private_mod::VAR_MASK;
                if cat #category_max_comparison #private_mod::CAT_MAX && variant <= #private_mod::VAR_MAXES[cat as usize] {
                    Some(Self::new(#err_cat_name::new(cat), variant))
                } else {
                    None
                }
            }
        };

        quote! {
            #err_kind_doc
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct #err_kind_name(#private_mod::R);

            impl #err_kind_name {
                const fn new(cat: #err_cat_name, variant: #private_mod::R) -> Self {
                    #err_kind_new_tokens
                }

                #[inline]
                fn category_value(&self) -> #private_mod::R {
                    #cat_value_tokens
                }

                #[inline]
                fn variant_value(&self) -> #private_mod::R {
                    self.0 & #private_mod::VAR_MASK
                }

                #[doc = " Returns the error category."]
                #[inline]
                pub fn category(&self) -> #err_cat_name {
                    #err_cat_name::new(self.category_value())
                }

                #[doc = " Returns the error kind name."]
                #[inline]
                pub fn name(&self) -> &'static str {
                    #error_names_mod::A[self.category_value() as usize][self.variant_value() as usize]
                }

                #[inline]
                fn display(&self) -> &'static str {
                    #error_displays_mod::A[self.category_value() as usize][self.variant_value() as usize]
                }

                #[doc = " Returns the error kind value as the underlying Rust type."]
                #[inline]
                pub fn value(&self) -> #private_mod::R {
                    self.0
                }

                #[doc = " Creates an error kind from a raw value of the underlying Rust type."]
                #[inline]
                pub fn from_value(value: #private_mod::R) -> Option<Self> {
                    #from_value_tokens
                }
            }

            impl tighterror::Kind for #err_kind_name {
                type R = #private_mod::R;
                type Category = #err_cat_name;

                const BITS: usize = #private_mod::KIND_BITS;

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

            impl core::fmt::Display for #err_kind_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.name())
                }
            }

            impl core::fmt::Debug for #err_kind_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct(#err_kind_name_str)
                        .field("cat", &#private_mod::Ident(self.category().name()))
                        .field("var", &#private_mod::Ident(self.name()))
                        .field("val", &self.0)
                        .finish()
                }
            }

            #result_from_err_kind
        }
    }

    fn error_tokens(&self) -> TokenStream {
        let err_name = self.err_name_ident();
        let err_kind_name = self.err_kind_name_ident();
        let err_cat_name = self.err_cat_name_ident();
        let err_doc = doc_tokens(self.module.err_doc());
        let private_mod = private_mod_ident();
        let result_from_err = if self.module.result_from_err() {
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
        let error_trait = if self.module.error_trait(self.spec.main.no_std) {
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
            pub struct #err_name(#err_kind_name);

            impl #err_name {
                #[doc = " Returns the error kind."]
                #[inline]
                pub fn kind(&self) -> #err_kind_name {
                    self.0
                }

                #[doc = " Returns the error origin location."]
                #[inline]
                pub fn location(&self) -> tighterror::Location {
                    tighterror::Location::undefined()
                }
            }

            impl tighterror::Error for #err_name {
                type R = #private_mod::R;
                type Category = #err_cat_name;
                type Kind = #err_kind_name;

                #[inline]
                fn kind(&self) -> Self::Kind {
                    self.kind()
                }

                #[inline]
                fn location(&self) -> tighterror::Location {
                    self.location()
                }
            }

            impl core::convert::From<#err_kind_name> for #err_name {
                #[inline]
                fn from(kind: #err_kind_name) -> Self {
                    Self(kind)
                }
            }

            impl core::fmt::Display for #err_name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(self.kind().display())
                }
            }

            impl core::cmp::PartialEq for #err_name {
                #[doc = " Checks equality based on the error kind only."]
                #[inline]
                fn eq(&self, other: &#err_name) -> bool {
                    self.0 == other.0
                }
            }

            #result_from_err
            #error_trait
        }
    }

    fn error_kind_constants_tokens(&self) -> TokenStream {
        let err_kinds_mod = error_kinds_mod_ident();
        let err_kind_name = self.err_kind_name_ident();
        let mut tokens = TokenStream::default();
        for c in &self.module.categories {
            let cat_tokens = self.error_kind_category_constants_tokens(c);
            if self.module.flat_kinds() {
                tokens = quote! {
                    #tokens
                    #cat_tokens
                };
            } else {
                let cat_mod_ident = format_ident!("{}", c.module_name());
                let cat_mod_doc = doc_tokens(&format!("{} category error kind constants.", c.name));
                tokens = quote! {
                    #tokens

                    #cat_mod_doc
                    pub mod #cat_mod_ident {
                        use super::c;
                        use super::EK;
                        #cat_tokens
                    }
                };
            }
        }

        let categories_mod = categories_mod_ident();
        quote! {
            #[doc = " Error kind constants."]
            pub mod #err_kinds_mod {
                use super::#err_kind_name as EK;
                use super::#categories_mod as c;
                #tokens
            }
        }
    }

    fn error_kind_category_constants_tokens(&self, c: &CategorySpec) -> TokenStream {
        let mut tokens = TokenStream::default();
        for (i, e) in c.errors.iter().enumerate() {
            let cat_ident = format_ident!("{}", c.ident_name());
            let err_value = self.usize_to_repr_type_literal(i).unwrap();
            let err_ident = format_ident!("{}", e.name);
            let err_doc = doc_tokens(self.module.err_kind_const_doc(c, e));
            tokens = quote! {
                #tokens

                #err_doc
                pub const #err_ident: EK = EK::new(c::#cat_ident, #err_value);
            };
        }
        tokens
    }

    fn category_constants_tokens(&self) -> TokenStream {
        let err_cat_name = self.err_cat_name_ident();
        let mut tokens = quote! {};
        for (i, c) in self.module.categories.iter().enumerate() {
            let cat_value = self.usize_to_repr_type_literal(i).unwrap();
            let cat_name_upper_snake = format_ident!("{}", c.ident_name());
            let const_doc = doc_tokens(self.module.cat_const_doc(c));
            let single = quote! {
                #const_doc
                pub const #cat_name_upper_snake: C = C::new(#cat_value);
            };
            tokens = quote! {
                #tokens
                #single
            };
        }

        let categories_mod = categories_mod_ident();
        quote! {
            #[doc = " Error category constants."]
            pub mod #categories_mod {
                use super::#err_cat_name as C;
                #tokens
            }
        }
    }

    fn needs_variants_module(&self) -> bool {
        self.module.has_variant_types()
    }

    fn variants_module_tokens(&self) -> TokenStream {
        if !self.needs_variants_module() {
            return TokenStream::default();
        }

        let variant_types_mod_tokens = self.variant_types_module_tokens();
        let variants_mod = variants_mod_ident();
        quote! {
            #[doc = " Error variant types and constants."]
            pub mod #variants_mod {
                #variant_types_mod_tokens
            }
        }
    }

    fn variant_types_module_tokens(&self) -> TokenStream {
        let mut tokens = TokenStream::default();

        if !self.module.has_variant_types() {
            return tokens;
        }

        let kinds_mod = error_kinds_mod_ident();
        let private_mod = private_mod_ident();
        let category_names_mod = category_names_mod_ident();
        let categories_mod = categories_mod_ident();
        let display_mod = error_displays_mod_ident();
        let error_names_mod = error_names_mod_ident();
        let err_kind_name = self.err_kind_name_ident();
        let err_name = self.err_name_ident();
        let cat_name = self.err_cat_name_ident();
        let use_display_mod = if self.module.has_display_variant_types() {
            quote! { #display_mod, }
        } else {
            TokenStream::default()
        };
        let use_tokens = quote! {
            super::{
                #use_display_mod #private_mod, #category_names_mod,
                #error_names_mod,
                #categories_mod, #kinds_mod, #err_kind_name,
                #err_name, #cat_name
            }
        };
        if self.module.flat_kinds() {
            tokens = quote! { use super::#use_tokens; };
        }

        for c in &self.module.categories {
            if !self.module.cat_has_variant_types(c) {
                continue;
            }

            let cvt = self.category_variant_types_tokens(c);
            if self.module.flat_kinds() {
                tokens = quote! {
                    #tokens
                    #cvt
                };
            } else {
                let cat_mod_ident = format_ident!("{}", c.module_name());
                let cat_mod_doc = doc_tokens(&format!("{} category error variant types.", c.name));

                tokens = quote! {
                    #tokens

                    #cat_mod_doc
                    pub mod #cat_mod_ident {
                        use super::super::#use_tokens;
                        #cvt
                    }
                };
            }
        }

        let types_mod = types_mod_ident();
        quote! {
            #[doc = " Error variant types."]
            pub mod #types_mod {
                #tokens
            }
        }
    }

    fn category_variant_types_tokens(&self, c: &CategorySpec) -> TokenStream {
        let mut tokens = TokenStream::default();

        for e in &c.errors {
            if !self.module.err_has_variant_type(c, e) {
                continue;
            }
            let evt = self.error_variant_type_tokens(c, e);
            tokens = quote! {
                #tokens
                #evt
            };
        }

        tokens
    }

    fn error_variant_type_tokens(&self, c: &CategorySpec, e: &ErrorSpec) -> TokenStream {
        let display_mod = error_displays_mod_ident();
        let kinds_mod = error_kinds_mod_ident();
        let private_mod = private_mod_ident();
        let category_names_mod = category_names_mod_ident();
        let categories_mod = categories_mod_ident();
        let error_names_mod = error_names_mod_ident();
        let cat_const_ident = format_ident!("{}", c.ident_name());
        let err_kind_name_ident = self.err_kind_name_ident();
        let err_name_ident = self.err_name_ident();
        let cat_mod = format_ident!("{}", c.module_name());
        let cat_name_ident = self.err_cat_name_ident();
        let err_ident = format_ident!("{}", e.name);
        let err_kind_const_ident = format_ident!("{}", e.name);
        let var_type_name = e.variant_type_name();
        let var_type_ident = format_ident!("{}", var_type_name);
        let err_doc = doc_tokens(self.module.err_kind_const_doc(c, e));
        let display = if e.display.is_some() {
            quote! { #display_mod::#cat_mod::#err_ident }
        } else {
            quote! { <Self as tighterror::VariantType>::NAME }
        };
        let error_trait = if self.module.error_trait(self.spec.main.no_std) {
            quote! { impl std::error::Error for #var_type_ident {} }
        } else {
            TokenStream::default()
        };
        let err_kind_tokens = if self.module.flat_kinds() {
            quote! { #kinds_mod::#err_kind_const_ident }
        } else {
            quote! { #kinds_mod::#cat_mod::#err_kind_const_ident }
        };
        quote! {
            #err_doc
            #[derive(Clone, Copy)]
            #[non_exhaustive]
            pub struct #var_type_ident;

            impl #var_type_ident {
                #[doc = " Returns the struct name."]
                #[inline]
                pub fn name(&self) -> &'static str {
                    <#var_type_ident as tighterror::VariantType>::NAME
                }

                #[doc = " Returns the error kind constant."]
                #[inline]
                pub fn kind(&self) -> #err_kind_name_ident {
                    <#var_type_ident as tighterror::VariantType>::KIND
                }

                #[doc = " Returns the error category constant."]
                #[inline]
                pub fn category(&self) -> #cat_name_ident {
                    <#var_type_ident as tighterror::VariantType>::CATEGORY
                }
            }

            impl core::fmt::Display for #var_type_ident {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.pad(#display)
                }
            }

            impl core::fmt::Debug for #var_type_ident {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct(#var_type_name)
                        .field("cat", &#private_mod::Ident(#category_names_mod::#cat_const_ident))
                        .field("var", &#private_mod::Ident(#error_names_mod::#cat_mod::#err_ident))
                        .finish()
                }
            }

            impl tighterror::VariantType for #var_type_ident {
                type R = #private_mod::R;
                type Category = #cat_name_ident;
                type Kind = #err_kind_name_ident;

                const CATEGORY: Self::Category = #categories_mod::#cat_const_ident;
                const KIND: Self::Kind = #err_kind_tokens;
                const NAME: &'static str = #var_type_name;
            }

            impl core::convert::From<#var_type_ident> for #err_kind_name_ident {
                #[inline]
                fn from(_: #var_type_ident) -> #err_kind_name_ident {
                    <#var_type_ident as tighterror::VariantType>::KIND
                }
            }

            impl core::convert::From<#var_type_ident> for #err_name_ident {
                #[inline]
                fn from(_: #var_type_ident) -> Self {
                    <#var_type_ident as tighterror::VariantType>::KIND.into()
                }
            }

            impl<T> core::convert::From<#var_type_ident> for core::result::Result<T, #err_name_ident> {
                #[inline]
                fn from(_: #var_type_ident) -> Self {
                    <#var_type_ident as tighterror::VariantType>::KIND.into()
                }
            }

            impl<T> core::convert::From<#var_type_ident> for core::result::Result<T, #var_type_ident> {
                #[inline]
                fn from(v: #var_type_ident) -> Self {
                    Err(v)
                }
            }

            #error_trait
        }
    }

    fn test_tokens(&self) -> TokenStream {
        let tests_mod = tests_mod_ident();
        if self.opts.test {
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
        let ut_err_kind_name = self.ut_err_kind_name();
        let ut_err_kind_display = self.ut_err_kind_display();
        let ut_err_kind_uniqueness = self.ut_err_kind_uniqueness();
        let ut_err_kind_value_uniqueness = self.ut_err_kind_value_uniqueness();
        let ut_err_kind_category = self.ut_err_kind_category();
        let ut_err_kind_from_value = self.ut_err_kind_from_value();
        let ut_err_display = self.ut_err_display();
        let ut_variant_types_display = self.ut_variant_types_display();
        let ut_variant_types_to_kind = self.ut_variant_types_to_kind();
        let ut_variant_types_to_error = self.ut_variant_types_to_error();
        let ut_variant_types_to_result = self.ut_variant_types_to_result();

        quote! {
            #ut_category_name
            #ut_category_display
            #ut_category_uniqueness
            #ut_category_values
            #ut_err_kind_name
            #ut_err_kind_display
            #ut_err_kind_uniqueness
            #ut_err_kind_value_uniqueness
            #ut_err_kind_category
            #ut_err_kind_from_value
            #ut_err_display
            #ut_variant_types_display
            #ut_variant_types_to_kind
            #ut_variant_types_to_error
            #ut_variant_types_to_result
        }
    }

    fn ut_category_name_tokens(&self) -> TokenStream {
        let categories_mod = categories_mod_ident();
        let check_cat_name_iter = self.module.categories.iter().map(|c| {
            let ident_name = c.ident_name();
            let ident = format_ident!("{}", ident_name);
            quote! {
                assert_eq!(#ident.name(), #ident_name);
                assert_eq!(tighterror::Category::name(&#ident), #ident_name)
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
        if self.spec.main.no_std() {
            return TokenStream::default();
        }
        let categories_mod = categories_mod_ident();
        let check_cat_display_iter = self.module.categories.iter().map(|c| {
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
        if self.spec.main.no_std() {
            return TokenStream::default();
        }
        let err_cat_name = self.err_cat_name_ident();
        let categories_mod = categories_mod_ident();
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
        let categories_mod = categories_mod_ident();
        let cat_arr = self.ut_cat_arr();
        let n_categories = Literal::usize_unsuffixed(self.module.categories.len());
        let category_max = Literal::usize_unsuffixed(self.module.categories.len() - 1);
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

    fn ut_err_kind_name(&self) -> TokenStream {
        let err_kinds_mod = error_kinds_mod_ident();
        let iter = self.module.categories.iter().map(|c| {
            let ec_iter = c.errors.iter().map(|e| {
                let name = e.name.as_str();
                let add_cat_mod = !self.module.flat_kinds();
                let ident = self.err_const_tokens(c, e, add_cat_mod);
                quote! {
                    assert_eq!(#ident.name(), #name);
                    assert_eq!(tighterror::Kind::name(&#ident), #name);
                }
            });
            quote! {
                #(#ec_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_kind_name() {
                use #err_kinds_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_kind_display(&self) -> TokenStream {
        if self.spec.main.no_std() {
            return TokenStream::default();
        }
        let err_kinds_mod = error_kinds_mod_ident();
        let iter = self.module.categories.iter().map(|c| {
            let ec_iter = c.errors.iter().map(|e| {
                let name = e.name.as_str();
                let add_cat_mod = !self.module.flat_kinds();
                let ident = self.err_const_tokens(c, e, add_cat_mod);
                quote! {
                    assert_eq!(format!("{}", #ident), #name);
                }
            });
            quote! {
                #(#ec_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_kind_display() {
                use #err_kinds_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_kind_uniqueness(&self) -> TokenStream {
        if self.spec.main.no_std() {
            return TokenStream::default();
        }
        let err_kind_name = self.err_kind_name_ident();
        let err_kinds_mod = error_kinds_mod_ident();
        let err_kind_arr = self.ut_err_kind_arr();
        let n_errors =
            Literal::usize_unsuffixed(self.module.categories.iter().map(|c| c.errors.len()).sum());
        quote! {
            #[test]
            fn test_err_kind_uniqueness() {
                use #err_kinds_mod::*;
                use std::collections::HashSet;
                let errs: [#err_kind_name; #n_errors] = #err_kind_arr;
                let set = HashSet::<#err_kind_name>::from_iter(errs);
                assert_eq!(set.len(), #n_errors);
            }
        }
    }

    fn ut_err_kind_value_uniqueness(&self) -> TokenStream {
        if self.spec.main.no_std() {
            return TokenStream::default();
        }
        let err_kind_name = self.err_kind_name_ident();
        let err_kinds_mod = error_kinds_mod_ident();
        let repr_type = self.bits.repr_type.ident();
        let err_kind_arr = self.ut_err_kind_arr();
        let n_errors =
            Literal::usize_unsuffixed(self.module.categories.iter().map(|c| c.errors.len()).sum());
        quote! {
            #[test]
            fn test_err_kind_value_uniqueness() {
                use #err_kinds_mod::*;
                use std::collections::HashSet;
                let errs: [#err_kind_name; #n_errors] = #err_kind_arr;
                let set = HashSet::<#repr_type>::from_iter(errs.iter().map(|ec| ec.value()));
                assert_eq!(set.len(), #n_errors);
            }
        }
    }

    fn ut_err_kind_category(&self) -> TokenStream {
        let categories_mod = categories_mod_ident();
        let err_kinds_mod = error_kinds_mod_ident();
        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                let add_cat_mod = !self.module.flat_kinds();
                let ident = self.err_const_tokens(c, e, add_cat_mod);
                let cat_ident = format_ident!("{}", c.ident_name());
                quote! {
                    assert_eq!(#ident.category(), #categories_mod::#cat_ident);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_kind_category() {
                use #err_kinds_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_kind_from_value(&self) -> TokenStream {
        let err_kind_name = self.err_kind_name_ident();
        let err_kinds_mod = error_kinds_mod_ident();
        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                let add_cat_mod = !self.module.flat_kinds();
                let ident = self.err_const_tokens(c, e, add_cat_mod);
                quote! {
                    assert_eq!(#err_kind_name::from_value(#ident.value()).unwrap(), #ident);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_kind_from_value() {
                use #err_kinds_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_err_display(&self) -> TokenStream {
        if self.spec.main.no_std() {
            return TokenStream::default();
        }
        let err_name = self.err_name_ident();
        let err_kinds_mod = error_kinds_mod_ident();
        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                let add_cat_mod = !self.module.flat_kinds();
                let err_ident = self.err_const_tokens(c, e, add_cat_mod);
                let display = if let Some(ref d) = e.display {
                    d.as_str()
                } else {
                    e.name.as_str()
                };
                quote! {
                    assert_eq!(format!("{}", #err_name::from(#err_ident)), #display);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });
        quote! {
            #[test]
            fn test_err_display() {
                use #err_kinds_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_variant_types_display(&self) -> TokenStream {
        if !self.module.has_variant_types() {
            return TokenStream::default();
        }

        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                if !self.module.err_has_variant_type(c, e) {
                    return TokenStream::default();
                }
                let add_cat_mod = !self.module.flat_kinds();
                let var_type_ident = self.err_var_type_tokens(c, e, add_cat_mod);
                let display = if let Some(ref d) = e.display {
                    d.clone()
                } else {
                    e.variant_type_name()
                };
                quote! {
                    assert_eq!(format!("{}", #var_type_ident), #display);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });

        let variants_mod = variants_mod_ident();
        let types_mod = types_mod_ident();
        quote! {
            #[test]
            fn test_variant_types_display() {
                use #variants_mod::#types_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_variant_types_to_kind(&self) -> TokenStream {
        if !self.module.has_variant_types() {
            return TokenStream::default();
        }

        let err_kinds_mod = error_kinds_mod_ident();
        let err_kind_name = self.err_kind_name_ident();

        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                if !self.module.err_has_variant_type(c, e) {
                    return TokenStream::default();
                }
                let add_cat_mod = !self.module.flat_kinds();
                let err_ident = self.err_const_tokens(c, e, add_cat_mod);
                let var_type_ident = self.err_var_type_tokens(c, e, add_cat_mod);
                quote! {
                    assert_eq!(#err_kind_name::from(#var_type_ident), #err_kinds_mod::#err_ident);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });

        let variants_mod = variants_mod_ident();
        let types_mod = types_mod_ident();
        quote! {
            #[test]
            fn test_variant_types_to_kind() {
                use #variants_mod::#types_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_variant_types_to_error(&self) -> TokenStream {
        if !self.module.has_variant_types() {
            return TokenStream::default();
        }

        let err_kinds_mod = error_kinds_mod_ident();
        let err_name = self.err_name_ident();

        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                if !self.module.err_has_variant_type(c, e) {
                    return TokenStream::default();
                }
                let add_cat_mod = !self.module.flat_kinds();
                let err_ident = self.err_const_tokens(c, e, add_cat_mod);
                let var_type_ident = self.err_var_type_tokens(c, e, add_cat_mod);
                quote! {
                    assert_eq!(#err_name::from(#var_type_ident).kind(), #err_kinds_mod::#err_ident);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });

        let variants_mod = variants_mod_ident();
        let types_mod = types_mod_ident();
        quote! {
            #[test]
            fn test_variant_types_to_error() {
                use #variants_mod::#types_mod::*;
                #(#iter)*
            }
        }
    }

    fn ut_variant_types_to_result(&self) -> TokenStream {
        if !self.module.has_variant_types() {
            return TokenStream::default();
        }

        let err_kinds_mod = error_kinds_mod_ident();
        let err_name = self.err_name_ident();

        let iter = self.module.categories.iter().map(|c| {
            let err_iter = c.errors.iter().map(|e| {
                if !self.module.err_has_variant_type(c, e) {
                    return TokenStream::default();
                }
                let add_cat_mod = !self.module.flat_kinds();
                let err_ident = self.err_const_tokens(c, e, add_cat_mod);
                let var_type_ident = self.err_var_type_tokens(c, e, add_cat_mod);
                quote! {
                    let res: Result<(), #err_name> = #var_type_ident.into();
                    assert_eq!(res.unwrap_err().kind(), #err_kinds_mod::#err_ident);
                }
            });
            quote! {
                #(#err_iter)*
            }
        });

        let variants_mod = variants_mod_ident();
        let types_mod = types_mod_ident();
        quote! {
            #[test]
            fn test_variant_types_to_result() {
                use #variants_mod::#types_mod::*;
                #(#iter)*
            }
        }
    }

    fn err_cat_name_ident(&self) -> Ident {
        format_ident!("{}", self.module.err_cat_name())
    }

    fn err_name_ident(&self) -> Ident {
        format_ident!("{}", self.module.err_name())
    }

    fn err_kind_name_ident(&self) -> Ident {
        format_ident!("{}", self.module.err_kind_name())
    }

    fn u64_to_repr_type_literal(&self, v: u64) -> Result<Literal, TryFromIntError> {
        match self.bits.repr_type {
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
            ReprType::U64 => Ok(Literal::u64_unsuffixed(v)),
        }
    }

    fn usize_to_repr_type_literal(&self, v: usize) -> Result<Literal, TryFromIntError> {
        match self.bits.repr_type {
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
        let category_max = self.module.category_max();
        TokenStream::from_str(if category_max == 0 { "==" } else { "<=" }).unwrap()
    }

    fn ut_cat_arr(&self) -> TokenStream {
        let cat_iter = self
            .module
            .categories
            .iter()
            .map(|c| format_ident!("{}", c.ident_name()));
        quote! {
            [#(#cat_iter),*]
        }
    }

    fn err_const_tokens(&self, c: &CategorySpec, e: &ErrorSpec, add_cat_mod: bool) -> TokenStream {
        let err_ident = format_ident!("{}", e.name);
        let cat_mod_ident = format_ident!("{}", c.module_name());
        if add_cat_mod {
            quote! {
                #cat_mod_ident::#err_ident
            }
        } else {
            quote! {
                #err_ident
            }
        }
    }

    fn err_var_type_tokens(
        &self,
        c: &CategorySpec,
        e: &ErrorSpec,
        add_cat_mod: bool,
    ) -> TokenStream {
        let var_type_ident = format_ident!("{}", e.variant_type_name());
        let cat_mod_ident = format_ident!("{}", c.module_name());
        if add_cat_mod {
            quote! {
                #cat_mod_ident::#var_type_ident
            }
        } else {
            quote! {
                #var_type_ident
            }
        }
    }

    fn n_categories_literal(&self) -> Literal {
        Literal::usize_unsuffixed(self.module.categories.len())
    }

    fn ut_err_kind_arr_impl(&self, add_cat_mod: bool) -> TokenStream {
        let iter = self.module.categories.iter().map(|c| {
            let iter = c
                .errors
                .iter()
                .map(|e| self.err_const_tokens(c, e, add_cat_mod));
            quote! {
                #(#iter),*
            }
        });

        quote! {
            [#(#iter),*]
        }
    }

    fn ut_err_kind_arr(&self) -> TokenStream {
        let add_cat_mod = !self.module.flat_kinds();
        self.ut_err_kind_arr_impl(add_cat_mod)
    }

    fn module_doc_tokens(&self) -> TokenStream {
        if self.mod_doc {
            outer_doc_tokens(self.module.doc())
        } else {
            TokenStream::default()
        }
    }
}
