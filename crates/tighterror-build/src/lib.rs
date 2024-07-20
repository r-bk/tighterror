//! *tighterror-build* is a library implementing the *tighterror*
//! specification file parsing, processing and Rust code generation.
//!
//! This library is part of the [tighterror] framework.
//!
//! A project that uses the *tighterror* framework doesn't need
//! to have *tighterror-build* in its `[dependencies]`. This library is at
//! most a build-dependency of the project, not required at runtime (unless
//! the project implements a tool similar to [cargo-tighterror]).
//!
//! *tighterror-build* itself uses the *tighterror* framework.
//! The [errors] module is a good example of how the end result looks like.
//!
//! The following sections compare two main ways to use *tighterror-build*.
//!
//! # cargo-tighterror
//!
//! The recommended way to use the library is through the framework's cargo
//! plugin [cargo-tighterror]. The plugin is a thin wrapper around
//! *tighterror-build* exposing its capabilities on the command-line.
//!
//! The benefits of this approach are:
//!
//! * *tighterror* Rust code is generated **before** a project is being built.
//! * The generated Rust code can be **reviewed**.
//! * The generated Rust code can be placed under **source control**
//!   to track changes.
//!
//! The downsides of this approach are:
//!
//! * It requires external means to keep the *tighterror* specification file
//!   and the build artifacts in sync.
//! * The sync needs to be done not only when the specification file changes,
//!   but also when *tighterror-build* version changes.
//!   Hence, the plugin is required to be periodically upgraded to follow the
//!   releases of *tighterror-build*.
//!
//! Although the build artifacts' sync can be tricky to solve automatically,
//! in our opinion, having the generated code reviewable and under source
//! control outweighs the cost.
//!
//! See [tighterror] documentation for more information about the plugin.
//!
//! # build.rs
//!
//! In cases where using the cargo plugin is inconvenient or impossible
//! *tighterror-build* can be used in `[build-dependencies]` of a project,
//! i.e., from a [build script].
//!
//! The main (and pretty strong) benefit of this approach is that keeping
//! the *tighterror* specification file and the build artifacts in sync
//! is fully automated, both when the specification file changes and when
//! *tighterror-build* version changes.
//!
//! The downsides of this approach are:
//!
//! * *tighterror* Rust code is generated **during** the project build stage.
//! * The build artifacts are placed in a temporary directory.
//!   This makes it hard to review the code.
//! * The build artifacts aren't versioned. This makes tracking of changes
//!   practically impossible, which is a big downside because the changes may be
//!   caused not only by specification (which itself is under source control),
//!   but also by an upgrade to a newer *tighterror-build* version.
//!
//! *tighterror-build* tries to minimize the amount of changes not caused by
//! specification file change. That said, especially in the early stages of
//! framework's development, such changes may occur.
//!
//! ## Example
//!
//! Assuming there is a specification file `tighterror.yaml`
//! in the root directory of a project, *tighterror-build* can be invoked
//! from project's build script as follows:
//!
//! ```no_run
//! // build.rs
//! use tighterror_build::CodegenOptions;
//!
//! fn tighterror_codegen() {
//!     println!("cargo:rerun-if-changed=tighterror.yaml");
//!     let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR isn't defined");
//!     let out_path = format!("{out_dir}/errors.rs");
//!     if let Err(e) = CodegenOptions::new().output(out_path).codegen() {
//!         panic!("tighterror-build failed: {e}; out_dir: {out_dir}");
//!     }
//! }
//!
//! fn main() {
//!     env_logger::builder().init();
//!     tighterror_codegen();
//! }
//! ```
//!
//! **Notes**
//!
//! 1. It is recommended to initialize some kind of logger in the build script
//!    because *tighterror-build* uses the [log] crate and in case of an
//!    error a detailed error description is traced.
//!    In the example above we used `env_logger` to trace to `stderr`.
//! 1. `cargo` takes care of rebuilding and rerunning the script when
//!    `[build-dependencies]` change, i.e., when *tighterror-build* version
//!    changes. To tell cargo that the specification file also affects the build
//!    we print the `cargo:rerun-if-changed=tighterror.yaml` instruction.
//!
//! To use the generated code include the module somewhere in the project's
//! operational code, e.g., `lib.rs` or `main.rs`, as follows:
//!
//! ```text
//! pub mod errors {
//!     include!(concat!(env!("OUT_DIR"), "/errors.rs"));
//! }
//! ```
//!
//! See [CodegenOptions] documentation for the full list of configurable
//! attributes.
//!
//! [cargo-tighterror]: https://crates.io/crates/cargo-tighterror
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [tighterror]: https://docs.rs/tighterror/latest/tighterror

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

mod common;

mod coder;
pub use coder::*;

pub mod errors;

pub(crate) mod parser;
pub(crate) mod spec;
