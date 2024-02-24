//! `tighterror-build` is a library, part of the `tighterror` framework, that
//! implements Rust code generation from a specification provided in a markup
//! language file.

#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

/// The default YAML specification file path.
#[cfg(feature = "yaml")]
#[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
pub const DEFAULT_SPEC_PATH_YAML: &str = "tighterror.yaml";

/// The default TOML specification file path.
#[cfg(feature = "toml")]
#[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
pub const DEFAULT_SPEC_PATH_TOML: &str = "tighterror.toml";

mod coder;
pub use coder::*;

pub mod errors;

pub(crate) mod parser;
pub(crate) mod spec;
pub(crate) mod util;
