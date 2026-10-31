//! *tighterror* is a minimalistic error representation framework with compact
//! error types and a code generator.
//!
//! It is heavily inspired by the standard `std::io::Error` which
//! represents a group of errors in a single Rust type.<br>
//! This concept is taken one step further so that a *tighterror* can represent
//! multiple groups of errors in a single Rust type.
//!
//! *tighterror* is designed to achieve the following goals:
//!
//! 1. Minimal error type size, e.g., `u8` or ZST, while still supporting
//!    idiomatic error reporting.
//! 1. Minimal runtime overhead on error path. No dynamic memory allocation.
//! 1. Minimal runtime overhead in matching a whole logical group of errors.
//! 1. Minimal coding overhead:
//!    - specify errors in a KDL file and generate all the boilerplate
//!      Rust code automatically
//!    - ergonomic error reporting
//!
//! # Table Of Contents
//!
//! 1. [High-Level Overview](#high-level-overview)
//!     * [Variant Types](#variant-types)
//! 1. [Implementation Details](#implementation-details)
//! 1. [Specification File Reference](#specification-file-reference)
//!     * [Filename](#filename)
//!     * [Structure](#structure)
//!     * [Names](#names)
//!     * [Root Settings](#root-settings)
//!     * [Module Settings](#module-settings)
//!     * [Category Settings](#category-settings)
//!     * [Error Settings](#error-settings)
//! 1. [tighterror-build](#tighterror-build)
//! 1. [cargo-tighterror](#cargo-tighterror)
//! 1. [Motivation](#motivation)
//!     * [Error as Trait Object](#error-as-trait-object)
//!     * [Error as Enum](#error-as-enum)
//!     * [Source Code Readability](#source-code-readability)
//!
//! ## High-Level Overview
//!
//! In its basic form a *tighterror* is a [newtype] around a small non-zero
//! unsigned integer, e.g., `NonZeroU8`.<br>
//! Error conditions are identified by an error [`kind`](Error::kind).
//! Every *error kind* has a [`category`](Kind::category) that
//! identifies the logical group the error belongs to.
//!
//! ```rust
//! # use core::num::NonZeroU8;
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! #[repr(transparent)]
//! pub struct ErrorKind(NonZeroU8);
//!
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! #[repr(transparent)]
//! pub struct ErrorCategory(u8);
//!
//! #[repr(transparent)]
//! pub struct Error(ErrorKind);
//! ```
//!
//! The [tighterror](crate) crate defines only the "public interface" of these
//! structs, and associated types:
//!
//! - [`Category`] for `ErrorCategory`
//! - [`Kind`] for `ErrorKind`
//! - [`Error`] for `Error`
//!
//! The concrete types are auto-generated from a specification file defined in
//! the user project. The specification file is written in
//! [KDL](https://kdl.dev). For example, `tighterror.kdl`:
//!
//! ```kdl
//! module errors {
//!     category General {
//!         error BAD_FILE
//!         error BAD_ARG
//!     }
//! }
//! ```
//!
//! The specification file is translated to Rust using a dedicated command
//! line tool [`cargo-tighterror`](#cargo-tighterror) or
//! using the [`tighterror-build`](#tighterror-build) library:
//!
//! ```shell
//! cargo tighterror -o src/errors.rs
//! ```
//!
//! This creates a module with all the concrete Rust types and
//! corresponding trait implementations. The error categories and kinds are
//! placed in the `category` and `kind` sub-modules respectively:
//!
//! ```rust
//! # mod _doc {
//! # pub struct ErrorKind(u8);
//! # pub struct ErrorCategory(u8);
//! # impl ErrorKind {
//! #    pub const fn new(cat: ErrorCategory, variant: u8) -> Self {
//! #        Self(variant << 1 | cat.0)
//! #    }
//! # }
//! # impl ErrorCategory { pub const fn new(v: u8) -> Self { Self(v) } }
//! pub mod category {
//! #   use super::*;
//!     pub const GENERAL: ErrorCategory = ErrorCategory::new(0);
//! }
//!
//! pub mod kind {
//! #   use super::*;
//! #   use super::category::*;
//!     pub const BAD_FILE: ErrorKind = ErrorKind::new(GENERAL, 1);
//!     pub const BAD_ARG: ErrorKind = ErrorKind::new(GENERAL, 2);
//! }
//! # } // mod _doc
//! ```
//!
//! `Error` is the main type to be used in `Result<T, E>`.
//! It can be created from an *error kind* constant like so:
//!
//! ```rust
//! # #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! # #[repr(transparent)]
//! # pub struct ErrorKind(u8);
//! # #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! # #[repr(transparent)]
//! # pub struct ErrorCategory(u8);
//! # #[repr(transparent)]
//! # pub struct Error(ErrorKind);
//! # const BAD_FILE: ErrorKind = ErrorKind(0);
//! # const GENERAL: ErrorCategory = ErrorCategory(0);
//! # impl From<ErrorKind> for Error {
//! #     fn from(v: ErrorKind) -> Self { Self(v) }
//! # }
//! # impl Error {
//! #     pub fn kind(&self) -> ErrorKind { self.0 }
//! #     pub fn category(&self) -> ErrorCategory { self.kind().category() }
//! # }
//! # impl ErrorKind {
//! #     pub fn category(&self) -> ErrorCategory { ErrorCategory(self.0 >> 2) }
//! # }
//! # fn foo() {
//! let e: Error = BAD_FILE.into();
//! assert!(matches!(e.kind(), BAD_FILE));
//! assert!(matches!(e.category(), GENERAL));
//! # }
//! ```
//!
//! Callers handle a whole category with one match arm on its constant, and
//! single errors with arms on *error kind* constants. A `_` arm is always
//! needed, because the compiler doesn't know that the constants are the only
//! values of these types:
//!
//! ```rust
//! # #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! # pub struct ErrorKind(u8);
//! # #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! # pub struct ErrorCategory(u8);
//! # pub struct Error(ErrorKind);
//! # impl ErrorKind {
//! #     pub fn category(&self) -> ErrorCategory { ErrorCategory(self.0 >> 4) }
//! # }
//! # impl Error {
//! #     pub fn kind(&self) -> ErrorKind { self.0 }
//! #     pub fn category(&self) -> ErrorCategory { self.0.category() }
//! # }
//! # pub mod category {
//! #     use super::ErrorCategory;
//! #     pub const PARSER: ErrorCategory = ErrorCategory(0);
//! #     pub const IO: ErrorCategory = ErrorCategory(1);
//! # }
//! # pub mod kind {
//! #     use super::ErrorKind;
//! #     pub const BAD_TOKEN: ErrorKind = ErrorKind(0);
//! #     pub const QUEUE_FULL: ErrorKind = ErrorKind(1);
//! #     pub const FILE_NOT_FOUND: ErrorKind = ErrorKind(16);
//! # }
//! fn group(e: &Error) -> &'static str {
//!     match e.category() {
//!         category::PARSER => "parser",
//!         category::IO => "io",
//!         _ => "other",
//!     }
//! }
//!
//! fn worth_retrying(e: &Error) -> bool {
//!     match e.kind() {
//!         kind::QUEUE_FULL | kind::FILE_NOT_FOUND => true,
//!         _ => false,
//!     }
//! }
//! # fn main() {
//! #     assert_eq!(group(&Error(kind::FILE_NOT_FOUND)), "io");
//! #     assert!(worth_retrying(&Error(kind::QUEUE_FULL)));
//! #     assert!(!worth_retrying(&Error(kind::BAD_TOKEN)));
//! # }
//! ```
//!
//! ### Variant Types
//!
//! There are many cases when a function has only a single error condition.
//! Returning an `Error` from such function may be excessive because it causes
//! a caller to handle a non-exhaustive list of errors (because an Error can
//! contain any of the error kinds defined in a module).
//!
//! Enabling a *variant type* on such error causes creation of a dedicated
//! error type (a ZST) to be used as the error type of the function.
//! This way a caller of the function needs to handle a single error condition,
//! making error handling simpler.
//!
//! Like *error kind*, *variant type* is convertible to `Error`, `ErrorKind`,
//! and `Result<T, Error>`. Hence, it is seamless to propagate through layers
//! of abstraction using the `?` operator.
//!
//! When at least one *variant type* is enabled *tighterror* creates a submodule
//! `variant::types` that contains the variant types, or per-category modules
//! with them when the `flat-kinds` module setting is disabled.
//!
//! ```kdl
//! module errors {
//!     category General {
//!         error OPERATION_TIMEOUT {
//!             spec { variant-type #true }
//!         }
//!     }
//! }
//! ```
//!
//! ```rust
//! pub mod kind {
//!     // Error kind constants defined here.
//!     // ...
//!     // pub const OPERATION_TIMEOUT: ErrorKind = ErrorKind::new(GENERAL, 1);
//! }
//!
//! pub mod variant {
//!     pub mod types {
//!         // Variant types defined here.
//!         #[non_exhaustive]
//!         pub struct OperationTimeout;
//!
//!         // impl tighterror::VariantType for OperationTimeout  { /* ... */ }
//!         // impl Debug for OperationTimeout { /* ... */ }
//!         // impl Display for OperationTimeout { /* ... */ }
//!         // impl OperationTimeout { /* ... */ }
//!         // impl core::error::Error for OperationTimeout {}
//!     }
//! }
//! ```
//!
//! [newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
//!
//!
//! ## Implementation Details
//!
//! *Error kind* is implemented with an unsigned integer (rather than an enum).
//! This allows us to embed some additional information in its bits.
//!
//! It is composed of two main parts - *error category* and *error variant*.
//! *Error category* denotes the group of errors a specific *error kind* belongs
//! to. *Error variant* is a unique identifier of the error within its category.
//!
//! The following is an example layout of an *error kind* with
//! allocation of bits suitable for 4 categories with a maximum of 31 errors
//! within each category:
//!
//! ```text
//!           ┌───┬───┬───┬───┬───┬───┬───┬───┐                      
//!           │   │   │   │   │   │   │   │   │ (0-4) 5 variant bits
//!         u8│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │ (5-6) 2 category bits
//!           │   │   │   │   │   │   │   │   │ (7-7) 1 reserved bit
//!           └───┼───┴───┼───┴───┴───┴───┴───┤                      
//!               │  cat  │      variant      │                      
//! ```
//!
//! When there is only a single *error category* the number of *category* bits
//! is 0.
//!
//! The value 0 is never an *error kind*: the variants of every category start
//! at 1. This lets the *error kind* be stored in a `NonZero` integer, e.g.,
//! `NonZeroU8`, so that `Option<Error>` and `Result<(), Error>` take no more
//! space than `Error`. Every category takes one variant more for it, which
//! adds a variant bit when the number of errors in the largest category is a
//! power of two.
//!
//! By default *tighterror* chooses the smallest unsigned integer that is
//! large enough to accommodate a required layout. Supported underlying
//! types are `u8`, `u16`, `u32` and `u64`. A module can set one of them
//! with the `repr` [module setting](#module-settings).
//!
//! Note that the layout of *error kind* is packed and may change
//! during project's lifetime. For example, addition of a new *error variant*
//! may lead to a left-shift of *category* bits, thus changing the numeric
//! value of all *error kinds* of the project, and possibly changing the
//! underlying Rust type to a wider unsigned integer, unless the module sets
//! `repr`.
//! Consequently, *error kinds* should be matched only using named constants
//! defined by their origin crate. No assumption should be made about a named
//! constant having any specific numeric value.
//!
//! [`tighterror-build`]: #tighterror-build
//! [`cargo-tighterror`]: #cargo-tighterror
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
//! more `category` nodes, and a category holds one or more `error` nodes.
//! Each of them takes its name as its single argument.
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
//! `pub mod errors { ... }`. In [*separate files* mode](#separate-files-mode)
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
//! <a name="output-setting"></a>`output` is an absolute path, a path
//! relative to the specification file, or `-` for `stdout`. When it is an
//! existing directory, the code goes to `tighterror.rs` under it, or to one
//! file per module in [*separate files* mode](#separate-files-mode). The
//! `-o, --output` argument of *cargo-tighterror* overrides it.
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
//! generation fails when the kinds need more bits than it has.
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
//! # tighterror-build
//!
//! [tighterror-build][tb-docs] is a library that implements *tighterror*
//! specification file parsing, processing, and code generation.
//! The library is usually not a direct dependency of a project that uses the
//! *tighterror* framework, i.e., it is needed at project's build time,
//! rather than at project's runtime.
//! The recommended way to use it is through
//! [`cargo-tighterror`](#cargo-tighterror) command-line utility.
//! In special cases, when the cargo plugin cannot be used, the library can be
//! used as a build-dependency of the project, i.e., in `build.rs`.
//!
//! See the [library documentation][tb-docs] for more details.
//!
//! [tb-docs]: https://docs.rs/tighterror-build/latest/tighterror_build
//!
//! # cargo-tighterror
//!
//! [`cargo-tighterror`][ct-docs] is the cargo plugin of the framework.
//! It is a thin wrapper around the [`tighterror-build`] library.
//!
//! [ct-docs]: https://crates.io/crates/cargo-tighterror
//!
//! Install the plugin, or upgrade to the latest version,
//! using the following command:
//!
//! ```shell
//! cargo install cargo-tighterror
//! ```
//!
//! The plugin needs to be periodically upgraded to follow releases of
//! [`tighterror-build`].
//!
//! The plugin is the recommended way to process *tighterror* specification
//! files and generate code. It allows a project to generate code in pre-build
//! stage so that both the specification file and the generated code are under
//! source control. Additionally, the plugin allows to override part of the
//! settings defined in the specification file.
//!
//! ```text
//! $> cargo help tighterror
//! The cargo plugin of the tighterror framework.
//!
//! Usage: cargo tighterror [OPTIONS]
//!
//! Options:
//!   -s, --spec <PATH>     The specification file path
//!   -o, --output <PATH>   The output path
//!   -t, --test            Include a unit-test in the generated code
//!   -u, --update          Do not overwrite the output file if data is unchanged
//!   -S, --separate-files  Write modules in separate files
//!   -h, --help            Print help
//!   -V, --version         Print version
//! ```
//!
//! * `-s, --spec <PATH>` (optional)
//!
//!   Defines the specification file path.
//!
//!   It is required if a specification file has a custom name.
//!   It can be omitted if the specification file uses the default
//!   name `tighterror.kdl` and is present in the current working directory.
//!
//!   The file extension `.kdl` is mandatory in custom filenames.
//!   <br><br>
//!
//! * `-o, --output <PATH>` (optional)
//!
//!   Defines the output path.
//!
//!   This can be either an absolute path, a relative path, or a hyphen `-`.
//!   A relative path is relative to the location of the specification file.
//!
//!   This argument overrides the [`output`](#output-setting)
//!   setting defined in the specification file.
//!   If this argument is omitted and output path isn't
//!   defined in the specification file the output is written to `stdout`.
//!
//!   Use hyphen `-` to force the output to be written to `stdout`.<br><br>
//!
//! * `-t, --test` (optional)
//!
//!   Includes a unit-test in the generated Rust code.
//!
//!   In `no_std` environments test cases that require `std` are excluded.<br><br>
//!
//! * `-u, --update` (optional)
//!
//!   Enables the update mode.
//!
//!   By default the output file is overwritten unconditionally, even when
//!   the new data equals the already existing one. This may cause an
//!   unnecessary crate rebuild, because file's write timestamp is updated.
//!   In update mode the new data is compared to the existing one,
//!   and the output file is overwritten only if the new data is different.
//!   Therefore, the file's write timestamp is updated only if the file content
//!   has changed, i.e., a rebuild is indeed required.
//!   <br><br>
//!
//! * `-S, --separate-files` (optional)<a name="separate-files-mode"></a>
//!
//!   Enables the *separate files* mode.
//!
//!   By default the output, even with multiple modules, is written in a single
//!   file. This means that every module is enclosed in a *module block*,
//!   e.g., `pub mod my_errors { ... }`.
//!
//!   In *separate files* mode every module is written in a separate Rust file.
//!   The `-o, --output` argument must point to an existing directory under
//!   which the separate files are written. The filenames are derived from
//!   the corresponding module names with the addition of `.rs` suffix.
//!
//!   If the output is written to `stdout` the *separate files* mode is
//!   implicitly disabled.
//!
//!   When a module is written in a separate file, or when there is only a
//!   single module in specification, the module isn't enclosed in a *module
//!   block*. Therefore inclusion of such file generated by a build-script
//!   allows addition of custom error types and trait implementations.
//!   For example:
//!
//!   ```text
//!   /// Module doc string
//!   pub mod errors {
//!       tighterror::include_errors!(errors);
//!
//!       #[derive(thiserror::Error, Debug)]
//!       #[error("Custom display string")]
//!       pub struct CustomError;
//!
//!       impl From<CustomError> for Error {
//!         ...
//!       }
//!   }
//!   ```
//!   <br><br>
//!
//! # Motivation
//!
//! Error handling in general and error representation/reporting in particular
//! are non-trivial topics in many programming languages.
//!
//! Rust simplifies error reporting by introduction of [`Result`] which is
//! [`must_use`]. This is great, as forgetting to check an error condition
//! becomes a compilation error. However, a programmer still has a lot of
//! decisions to make.
//!
//! [`must_use`]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute
//!
//! This rather large section describes the reasons for *tighterror* creation.
//!
//! ## Error as Trait Object
//!
//! Lets take a look at a function with multiple error conditions:
//!
//! ```compile_fail
//! use std::{error::Error, fmt::Write, fs::File, io::Read};
//!
//! fn foo() -> Result<String, std::io::Error> {
//!     let mut f = File::options().read(true).open("/etc/hosts")?;
//!     let mut hosts = String::new();
//!     f.read_to_string(&mut hosts)?;
//!     let mut v = String::new();
//!     write!(v, "hosts = {:?}", hosts)?;
//!     Ok(v)
//! }
//! ```
//!
//! This function has three error cases: `File::open` on line 4,
//! `Read::read_to_string` on line 6, and `write!` on line 8.
//! The first two have `io::Error` as the error type, while the third has
//! `fmt::Error`.
//! As a consequence this function doesn't compile because the error type
//! `fmt::Error` is not implicitly convertible to `io::Error` by the `?`
//! operator:
//!
//! ```text
//! ---- src/lib.rs - (line 37) stdout ----
//! error[E0277]: `?` couldn't convert the error to `std::io::Error`
//!   --> src/lib.rs:45:37
//!    |
//! 5  | fn foo() -> Result<String, std::io::Error> {
//!    |             ------------------------------ expected `std::io::Error` because of this
//! ...
//! 10 |     write!(v, "hosts = {:?}", hosts)?;
//!    |                                     ^ the trait `From<std::fmt::Error>` is not implemented for `std::io::Error`
//!    |
//!    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
//! ...
//! error: aborting due to previous error
//! ```
//!
//! So, what should be the error type of `foo`?
//!
//! One solution is to return a type that can "wrap" any type
//! that implements the `std::error::Error` trait.<br>
//! For example, `Box<dyn Error>`:
//!
//! ```rust
//! use std::{boxed::Box, error::Error, fmt::Write, fs::File, io::Read};
//!
//! fn foo() -> Result<String, Box<dyn Error>> {
//!     let mut f = File::options().read(true).open("/etc/hosts")?;
//!     let mut hosts = String::new();
//!     f.read_to_string(&mut hosts)?;
//!     let mut v = String::new();
//!     write!(v, "hosts = {:?}", hosts)?;
//!     Ok(v)
//! }
//! ```
//!
//! This example compiles successfully because any type that implements
//! `std::error::Error` is convertible to `Box<dyn Error>` via the
//! implementations of `From`
//! [here](https://doc.rust-lang.org/std/boxed/struct.Box.html#impl-From%3CE%3E-for-Box%3Cdyn+Error%3E)
//! and
//! [here](https://doc.rust-lang.org/std/boxed/struct.Box.html#impl-From%3CE%3E-for-Box%3Cdyn+Error+%2B+Send+%2B+Sync%3E).
//!
//! A more feature-rich solution is provided by the very popular [`anyhow`]
//! crate:
//!
//! [`anyhow`]: https://docs.rs/anyhow/latest/anyhow
//!
//! ```rust
//! use std::{fmt::Write, fs::File, io::Read};
//!
//! fn foo() -> Result<String, anyhow::Error> {
//!     let mut f = File::options().read(true).open("/etc/hosts")?;
//!     let mut hosts = String::new();
//!     f.read_to_string(&mut hosts)?;
//!     let mut v = String::new();
//!     write!(v, "hosts = {:?}", hosts)?;
//!     Ok(v)
//! }
//! ```
//!
//! This works because `anyhow::Error` can be created from any type that
//! implements `std::error::Error` as implemented
//! [here](https://docs.rs/anyhow/latest/anyhow/struct.Error.html#impl-From%3CE%3E-for-Error).
//!
//! The common technique to both of these solutions is to "hide" a real
//! error instance behind a trait object allocated on the heap.
//! This works great in the common case, and also preserves the information
//! contained in the original error instance.
//!
//! However, this convenience has a cost:
//!
//! 1. Dynamic memory allocation is made to construct the target error instance.
//! 1. The size of the target instance (even without the heap-allocated memory)
//!    may be different from that of the original one, and in some cases may be
//!    significantly larger, e.g., when the original instance is a ZST.
//! 1. Calling methods of the "hidden" instance involves dynamic dispatch.
//!
//! These costs may be negligible in many projects, but not in all of them.
//!
//! In *tighterror* an error condition is identified by a small unique unsigned
//! integer, i.e., an *error kind*.
//! Conversion into *tighterror* is very cheap as it entails only a function
//! call that returns a constant value (which in many cases may be inlined).
//!
//! ## Error as Enum
//!
//! Another solution to support multiple error conditions in a given function
//! is to enumerate them, i.e., the error type of the function is an `enum`
//! which has a distinct variant for every error case of the function.
//!
//! For example:
//!
//! ```rust
//! use std::{fmt::Write, fs::File, io::Read};
//!
//! #[derive(Debug)]
//! enum Error {
//!     IoError(std::io::Error),
//!     FmtError(std::fmt::Error),
//! }
//!
//! impl std::fmt::Display for Error {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//!         write!(f, "{:?}", self)
//!     }
//! }
//!
//! impl std::convert::From<std::io::Error> for Error {
//!     fn from(value: std::io::Error) -> Self {
//!         Error::IoError(value)
//!     }
//! }
//!
//! impl std::convert::From<std::fmt::Error> for Error {
//!     fn from(value: std::fmt::Error) -> Self {
//!         Error::FmtError(value)
//!     }
//! }
//!
//! impl std::error::Error for Error {}
//!
//! fn foo() -> Result<String, Error> {
//!     let mut f = File::options().read(true).open("/etc/hosts")?;
//!     let mut hosts = String::new();
//!     f.read_to_string(&mut hosts)?;
//!     let mut v = String::new();
//!     write!(v, "hosts = {:?}", hosts)?;
//!     Ok(v)
//! }
//! ```
//! This is a lot of coding to support a single function with two error
//! conditions. Fortunately, we have [`thiserror`] crate which
//! makes all of this much shorter:
//!
//! [`thiserror`]: https://docs.rs/thiserror/latest/thiserror
//!
//! ```rust
//! use std::{fmt::Write, fs::File, io::Read};
//!
//! #[derive(thiserror::Error, Debug)]
//! enum Error {
//!     #[error(transparent)]
//!     IoError(#[from] std::io::Error),
//!     #[error(transparent)]
//!     FmtError(#[from] std::fmt::Error),
//! }
//!
//! fn foo() -> Result<String, Error> {
//!     let mut f = File::options().read(true).open("/etc/hosts")?;
//!     let mut hosts = String::new();
//!     f.read_to_string(&mut hosts)?;
//!     let mut v = String::new();
//!     write!(v, "hosts = {:?}", hosts)?;
//!     Ok(v)
//! }
//! ```
//!
//! This code is pretty short and clean. It preserves the information
//! contained in the inner errors, and there is no dynamic memory
//! allocation in translation of inner errors into the outer one.
//!
//! However, in our opinion, this solution works well only in case of small
//! units with low amount of error conditions (up to three).
//! For a large unit, e.g., a library entrypoint, which can fail on numerous
//! error conditions spread across several layers of abstraction this solution
//! doesn't scale well for the following reasons:
//!
//! 1. Different layers of abstraction suggest a different error type per
//!    layer, each with its own list of error variants. This implies that
//!    the top-level error type returned from the library entrypoint has
//!    an enum variant per inner abstraction layer.
//!
//! 1. Embedding inner errors as fields of outer errors creates
//!    a nested structure which already at the depth of 2 layers becomes
//!    hard to deal with, especially on the caller side.
//!
//! 1. The size of inner error types affects the size of outer types in the
//!    upper layers. This can easily get out of control when data
//!    fields are added to enum variants in the inner layers.
//!
//! 1. Adding meta-data, e.g., `Location` or `Backtrace`, to all errors
//!    means adding a corresponding field to every enum variant individually.
//!
//! *tighterror* takes a minimalistic approach where there is a single
//! error type that is capable to represent every error condition of a unit
//! via a unique *error kind*, even when errors are spread across different
//! layers of abstraction. The error type doesn't convey any additional
//! error-specific information, except for the optional meta-data,
//! e.g., `Location` or `Backtrace`. Representation of 3'rd party *tighterrors*
//! is done by dedicating an *error category* to the 3'rd party type and
//! embedding its bits as *error variant* in the outer type.
//!
//! ## Source Code Readability
//!
//! To mitigate issues 1-3 in [error as enum](#error-as-enum) one can use
//! `thiserror` to create error kinds from a simple enum without data fields,
//! like so:
//!
//! ```rust
//! #[derive(thiserror::Error, Debug)]
//! enum Error {
//!     #[error("io error")]
//!     IoError,
//!     #[error("formatting error")]
//!     FmtError,
//! }
//! ```
//!
//! This is simple and clean, and can work well even with a high number of
//! error variants in the same enum.
//!
//! Nevertheless, in our opinion, using constants for error kinds is much more
//! readable than enum variants. Because constants by Rust coding
//! convention are written in capital-snake case, e.g., `CONNECTION_REFUSED`,
//! while enum variants are written in upper-camel case, e.g.,
//! `ConnectionRefused`, just like structs, traits, unions and enums themselves.
//! Thus, using constants on error path is more intuitive and easier to spot
//! during code review because (a) an error kind is expected to be a constant
//! value, and (b) it stands out and makes the code less cluttered,
//! i.e., it doesn't add more items of the same
//! case as structs/traits/unions/enums that themselves appear in source code
//! pretty frequently.

#![no_std]
#![forbid(missing_docs)]
#![deny(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod category;
pub use category::*;

mod kind;
pub use kind::*;

mod error;
pub use error::*;

mod location;
pub use location::*;

mod repr;
pub use repr::*;

mod variant_type;
pub use variant_type::*;

mod macros;
