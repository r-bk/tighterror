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
//! Assuming there is a specification file `tighterror.kdl`
//! in the root directory of a project, *tighterror-build* can be invoked
//! from project's build script as follows:
//!
//! ```no_run
//! // build.rs
//! fn main() -> Result<(), tighterror_build::errors::TbError> {
//!     tighterror_build::build()
//! }
//! ```
//!
//! To use the code of a module include it somewhere in the project's
//! operational code, e.g., `lib.rs` or `main.rs`, with
//! [`tighterror::include_errors`]:
//!
//! ```text
//! /// Crate errors.
//! pub mod errors {
//!     tighterror::include_errors!(errors);
//! }
//! ```
//!
//! See [BuildOptions] for the options, and for what [build] does besides
//! writing the code.
//!
//! # Specification File Reference
//!
//! The specification file is written in [KDL](https://kdl.dev) 2.0.
//!
//! ## Filename
//!
//! When the specification file path isn't given, *tighterror-build* looks for
//! `tighterror.kdl` in the current working directory. A custom path must have
//! the `.kdl` extension.
//!
//! ## Structure
//!
//! A specification holds one or more `module` nodes. A module holds one or
//! more `category` nodes, and a category holds one or more `error` nodes,
//! unless it [wraps][wrapped-errors] the errors of another crate. Each of
//! them takes its name as its single argument.
//!
//! ```kdl
//! spec {
//!     output "src/errors.rs"
//! }
//!
//! module errors {
//!     spec { doc "Crate errors." }
//!     category Parser {
//!         spec { doc "Parser errors." }
//!         error BAD_TOKEN display="Decoded an invalid token."
//!         error QUEUE_FULL
//!     }
//!     category Io {
//!         error FILE_NOT_FOUND {
//!             spec {
//!                 display "File not found."
//!                 variant-type #true
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! The settings of a node go in its `spec` child node, one `key value` per
//! line, or several on one line separated by `;`. The `spec` node comes
//! first, at most once, and lists a setting at most once. The root `spec`
//! node comes before the modules.
//!
//! An error may also give its `display` on its own line, as
//! `display="..."`. When its `spec` gives one too, the `spec` one wins.
//!
//! Unknown nodes and settings, and values of the wrong type, are rejected
//! with the file, line and column of the offending node.
//!
//! ## Names
//!
//! | Node       | Case             | Becomes                             |
//! |------------|------------------|-------------------------------------|
//! | `module`   | lower_snake_case | a module, see below                 |
//! | `category` | UpperCamelCase   | a constant, e.g. `category::PARSER` |
//! | `error`    | UPPER_SNAKE_CASE | a constant, e.g. `kind::BAD_TOKEN`  |
//!
//! When the output holds several modules, each one is a module block, e.g.
//! `pub mod errors { ... }`. In [*separate files* mode][separate-files]
//! the module name is the file name instead.
//!
//! Module names are unique in the file, and category names in their module.
//! Error names are unique in their module, or in their category when the
//! module sets `flat-kinds #false`.
//!
//! ## Root Settings
//!
//! | Setting  | Type   | Default  | Description                            |
//! |----------|--------|----------|----------------------------------------|
//! | `output` | string | `stdout` | Output path, see below.                |
//! | `no-std` | bool   | `#false` | Generate code for `no_std`.            |
//!
//! `output` is an absolute path, a path relative to the specification file,
//! or `-` for `stdout`. When it is an existing directory, the code goes to
//! `tighterror.rs` under it, or to one file per module in
//! [*separate files* mode][separate-files]. The `-o, --output` argument of
//! *cargo-tighterror* overrides it.
//!
//! `no-std` turns off the unit tests that need `std`.
//!
//! ## Module Settings
//!
//! | Setting                | Type   | Default         | Description                                         |
//! |------------------------|--------|-----------------|-----------------------------------------------------|
//! | `doc`                  | string | none            | Doc comment of the module.                          |
//! | `err-name`             | string | `Error`         | Name of the [Error] struct.                         |
//! | `err-kind-name`        | string | `ErrorKind`     | Name of the [Kind] struct.                          |
//! | `err-cat-name`         | string | `ErrorCategory` | Name of the [Category] struct.                      |
//! | `err-doc`              | string | see below       | Doc comment of the [Error] struct.                  |
//! | `err-kind-doc`         | string | see below       | Doc comment of the [Kind] struct.                   |
//! | `err-cat-doc`          | string | see below       | Doc comment of the [Category] struct.               |
//! | `error-trait`          | bool   | `#true`         | Implement `core::error::Error` for the error type.  |
//! | `result-from-err`      | bool   | `#true`         | Implement `From<Error> for Result<T, Error>`.       |
//! | `result-from-err-kind` | bool   | `#true`         | Implement `From<ErrorKind> for Result<T, Error>`.   |
//! | `flat-kinds`           | bool   | `#true`         | Put all error kinds of the module in one namespace. |
//! | `repr`                 | string | see below       | Underlying type of the [Kind] struct.               |
//! | `doc-from-display`     | bool   | `#false`        | Default for the errors of the module.               |
//! | `variant-type`         | bool   | `#false`        | Default for the errors of the module.               |
//!
//! The struct names are UpperCamelCase. The default struct docs name the
//! struct and point at the `kind` or `category` module.
//!
//! `repr` is `u8`, `u16`, `u32` or `u64`. By default it is the smallest of
//! them that holds the error kinds of the module. A set type keeps the type
//! of `value()` and `from_value()` as the module gains errors, and the
//! generation fails when the kinds need more bits than it has. When the
//! [wrapped][wrapped-errors] kinds make them too wide, the compilation
//! fails instead.
//!
//! With `flat-kinds`, the error kinds are `kind::NAME` and the variant
//! types `variant::types::Name`, so their names are unique in the module.
//! With `flat-kinds #false` they move to a module per category, e.g.
//! `kind::parser::NAME`, and their names need to be unique in the category
//! only.
//!
//! ## Category Settings
//!
//! | Setting            | Type   | Default  | Description                             |
//! |--------------------|--------|----------|-----------------------------------------|
//! | `doc`              | string | none     | Doc comment of the category constant.   |
//! | `doc-from-display` | bool   | `#false` | Default for the errors of the category. |
//! | `variant-type`     | bool   | `#false` | Default for the errors of the category. |
//! | `wrap-error-type`  | string | none     | Path of the wrapped [Error] struct.     |
//! | `wrap-kind-type`   | string | none     | Path of the wrapped [Kind] struct.      |
//!
//! A category that sets `wrap-error-type` and `wrap-kind-type`
//! [wraps][wrapped-errors] the errors of another crate. It sets both, holds
//! no errors, sets neither `doc-from-display` nor `variant-type`, and is not
//! the only category of its module. The paths resolve in the module that
//! holds the generated code, e.g. `text::errors::Error` for a dependency
//! `text`.
//!
//! ## Error Settings
//!
//! | Setting            | Type         | Default  | Description                                          |
//! |--------------------|--------------|----------|------------------------------------------------------|
//! | `display`          | string       | the name | The `Display` text of the error kind.                |
//! | `doc`              | string       | none     | Doc comment of the error kind constant.              |
//! | `doc-from-display` | bool         | `#false` | Use a set `display` as the doc when `doc` is unset.  |
//! | `variant-type`     | bool or name | `#false` | Generate a variant type for the error.               |
//!
//! `doc-from-display` and `variant-type` set on an error win over its
//! category, and a category's win over its module.
//!
//! `variant-type #true` names the variant type after the error, e.g.
//! `OPERATION_TIMEOUT` becomes `OperationTimeout`. A name in UpperCamelCase,
//! e.g. `variant-type Timeout`, enables it under that name.
//!
//! [Error]: tighterror::Error
//! [wrapped-errors]: tighterror#wrapped-errors
//! [separate-files]: CodegenOptions::separate_files
//! [Kind]: tighterror::Kind
//! [Category]: tighterror::Category
//! [cargo-tighterror]: https://crates.io/crates/cargo-tighterror
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [tighterror]: https://docs.rs/tighterror/latest/tighterror

#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(unsafe_code)]

/// The default KDL specification file path.
pub const DEFAULT_SPEC_PATH_KDL: &str = "tighterror.kdl";

mod common;

mod coder;
pub use coder::*;

pub mod errors;

pub(crate) mod parser;
pub(crate) mod spec;
