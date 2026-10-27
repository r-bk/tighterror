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
        s.push(' ');
        s
    }
}

fn _doc_tokens(doc: &str, outer: bool) -> TokenStream {
    let doc = doc.trim_end_matches(['\r', '\n']);
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

#[cfg(test)]
mod testing {
    use super::*;
    use crate::coder::formatter::pretty;

    fn render(doc: &str) -> String {
        let doc = doc_tokens(doc);
        pretty(quote! { #doc pub struct S; }).unwrap()
    }

    // Checks that a multi-line doc becomes a block comment whose closing
    // line is ` */`, the form rustfmt 1.11 requires and older rustfmt keeps.
    #[test]
    fn test_multiline_doc_closing_line() {
        // render a two-line doc on a struct
        let code = render("Line one.\nLine two.");
        // the block comment holds both lines and closes with ` */`
        assert!(
            code.contains("/**\n * Line one.\n * Line two.\n */\npub struct S;"),
            "{code}"
        );
    }

    // Checks that a one-line doc ending in a newline, as a YAML block scalar
    // produces, becomes a line comment rather than a block comment.
    #[test]
    fn test_single_line_doc_with_trailing_newline() {
        // render a one-line doc that ends in a newline
        let code = render("Line one.\n");
        // the doc is a line comment, and no block comment is left
        assert!(code.contains("/// Line one.\npub struct S;"), "{code}");
        assert!(!code.contains("*/"), "{code}");
    }

    // Checks that a doc made of newlines only adds no doc attribute.
    #[test]
    fn test_newlines_only_doc() {
        // render a doc that holds newlines only
        let code = render("\n\n");
        // the struct is left without a doc comment
        assert_eq!(code.trim(), "pub struct S;", "{code}");
    }
}
