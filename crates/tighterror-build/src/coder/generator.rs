use crate::{
    coder::{formatter::pretty, idents, FrozenOptions},
    errors::TbError,
    spec::Spec,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

mod module;
mod modules;
use modules::ModulesGenerator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ReprType {
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug)]
pub struct ModuleCode {
    /// The module name
    pub(crate) name: String,
    /// The module code
    pub(crate) code: String,
}

#[allow(dead_code)]
struct RustGenerator<'a> {
    opts: &'a FrozenOptions,
    spec: &'a Spec,
}

impl<'a> RustGenerator<'a> {
    fn new(opts: &'a FrozenOptions, spec: &'a Spec) -> RustGenerator<'a> {
        Self { opts, spec }
    }

    fn rust(&self) -> Result<Vec<ModuleCode>, TbError> {
        let mg = ModulesGenerator::new(self.opts, &self.spec.main, &self.spec.modules);
        let tokens = mg.rust()?;
        let mut ret = Vec::new();
        for mt in tokens.into_iter() {
            ret.push(ModuleCode {
                name: mt.name,
                code: pretty(mt.tokens)?,
            });
        }
        Ok(ret)
    }
}

fn _handle_multiline_doc(doc: &str) -> String {
    let n_lines = doc.lines().count();
    if n_lines <= 1 {
        if doc.starts_with(' ') {
            doc.to_owned()
        } else {
            format!(" {doc}")
        }
    } else {
        let mut s = String::from("\n");
        for line in doc.lines() {
            s.push_str(&format!(" * {}\n", line));
        }
        s
    }
}

fn _doc_tokens(doc: &str, outer: bool) -> TokenStream {
    if doc.is_empty() {
        quote! {}
    } else {
        let doc = _handle_multiline_doc(doc);
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

    fn bits(&self) -> usize {
        match self {
            Self::U8 => u8::BITS as usize,
            Self::U16 => u16::BITS as usize,
            Self::U32 => u32::BITS as usize,
            Self::U64 => u64::BITS as usize,
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

fn err_kinds_mod_ident() -> Ident {
    format_ident!("{}", idents::ERROR_KINDS_MOD)
}

fn tests_mod_ident() -> Ident {
    format_ident!("{}", idents::TESTS_MOD)
}

fn categories_mod_ident() -> Ident {
    format_ident!("{}", idents::CATEGORY_CONSTS_MOD)
}

pub fn spec_to_rust(opts: &FrozenOptions, spec: &Spec) -> Result<Vec<ModuleCode>, TbError> {
    RustGenerator::new(opts, spec).rust()
}
