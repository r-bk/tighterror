//! An error representation framework with compact error types and a code
//! generator.
//!
//! `tighterror` allows developers to declare errors in a concise YAML file and
//! generate the Rust boilerplate code from it using a Cargo extension command.
//!
//! # Goals
//!
//! 1. Minimal underlying representation types (e.g. `u8`).
//! 2. Minimal runtime overhead. No dynamic memory allocation by default.
//! 3. Minimal coding overhead.
//!
//! # Installation
//!
//! Add `tighterror` to the list of dependencies in your projects's Cargo.toml:
//!
//! ```shell
//! cargo add tighterror
//! ```
//!
//! Install the Cargo extension that generates the Rust code for you:
//!
//! ```shell
//! cargo install cargo-tighterror
//! ```

#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod category;
pub use category::*;

mod code;
pub use code::*;

mod error;
pub use error::*;

mod location;
pub use location::*;
