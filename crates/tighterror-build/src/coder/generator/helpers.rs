use crate::coder::idents;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

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

pub fn doc_tokens(doc: &str) -> TokenStream {
    const OUTER: bool = false;
    _doc_tokens(doc, OUTER)
}

pub fn outer_doc_tokens(doc: &str) -> TokenStream {
    const OUTER: bool = true;
    _doc_tokens(doc, OUTER)
}

pub fn category_names_mod_ident() -> Ident {
    format_ident!("{}", idents::CATEGORY_NAMES_MOD)
}

pub fn error_names_mod_ident() -> Ident {
    format_ident!("{}", idents::ERROR_NAMES_MOD)
}

pub fn error_displays_mod_ident() -> Ident {
    format_ident!("{}", idents::ERROR_DISPLAYS_MOD)
}

pub fn private_mod_ident() -> Ident {
    format_ident!("{}", idents::PRIVATE_MOD)
}

pub fn error_kinds_mod_ident() -> Ident {
    format_ident!("{}", idents::ERROR_KINDS_MOD)
}

pub fn tests_mod_ident() -> Ident {
    format_ident!("{}", idents::TESTS_MOD)
}

pub fn categories_mod_ident() -> Ident {
    format_ident!("{}", idents::CATEGORY_CONSTS_MOD)
}

pub fn variants_mod_ident() -> Ident {
    format_ident!("{}", idents::VARIANTS_MOD)
}

pub fn types_mod_ident() -> Ident {
    format_ident!("{}", idents::TYPES_MOD)
}
