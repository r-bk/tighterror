use crate::{
    coder::{formatter::pretty, idents, options::CodegenOptions},
    errors::TbError,
    spec::Spec,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

mod module;
use module::ModuleGenerator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ReprType {
    U8,
    U16,
    U32,
    U64,
}

#[allow(dead_code)]
struct RustGenerator<'a> {
    opts: &'a CodegenOptions,
    spec: &'a Spec,
}

impl<'a> RustGenerator<'a> {
    fn new(opts: &'a CodegenOptions, spec: &'a Spec) -> RustGenerator<'a> {
        Self { opts, spec }
    }

    fn rust(&self) -> Result<String, TbError> {
        let mg = ModuleGenerator::new(self.opts, &self.spec.main, &self.spec.module)?;
        let tokens = mg.rust()?;
        pretty(tokens)
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

pub fn spec_to_rust(opts: &CodegenOptions, spec: &Spec) -> Result<String, TbError> {
    RustGenerator::new(opts, spec).rust()
}
