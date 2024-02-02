//! `tighterror-build` is a library, part of the `tighterror` framework, that
//! implements Rust code generation from a specification provided in a markup
//! language file.

#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

/// The default spec file path when none is provided.
///
/// The value of this constant depends on the set of enabled markup language
/// features. If the `yaml` feature is enabled, the value is `tighterror.yaml`.
/// Otherwise the value is `tighterror.toml`.
#[cfg(feature = "yaml")]
pub const DEFAULT_SPEC_PATH: &str = "tighterror.yaml";
#[cfg(not(feature = "yaml"))]
pub const DEFAULT_SPEC_PATH: &str = "tighterror.toml";

mod coder;
pub use coder::*;

pub mod errors;

pub(crate) mod parser;
pub(crate) mod spec;
pub(crate) mod util;
