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
//! 1. [Quick Start](#quick-start)
//! 1. [Concepts](#concepts)
//!     * [Layout](#layout)
//! 1. [Generated API](#generated-api)
//!     * [Variant Types](#variant-types)
//!     * [Wrapped Errors](#wrapped-errors)
//! 1. [Motivation](#motivation)
//!     * [Error as Trait Object](#error-as-trait-object)
//!     * [Error as Enum](#error-as-enum)
//!     * [Source Code Readability](#source-code-readability)
//!
//! # Quick Start
//!
//! The errors of a crate are defined in a specification file written in
//! [KDL](https://kdl.dev), e.g., `tighterror.kdl` in the root directory of
//! the crate:
//!
//! ```kdl
//! module errors {
//!     category General {
//!         error BAD_FILE display="File is invalid."
//!         error BAD_ARG
//!     }
//! }
//! ```
//!
//! The crate depends on *tighterror*, and on [tighterror-build] at build
//! time:
//!
//! ```shell
//! cargo add tighterror
//! cargo add --build tighterror-build
//! ```
//!
//! Its build script generates the code:
//!
//! ```rust,ignore
//! // build.rs
//! fn main() -> Result<(), tighterror_build::errors::TbError> {
//!     tighterror_build::build()
//! }
//! ```
//!
//! and [`include_errors`] includes it in a module of the crate:
//!
//! ```rust,ignore
//! /// Crate errors.
//! pub mod errors {
//!     tighterror::include_errors!(errors);
//! }
//!
//! use errors::{Error, kind};
//!
//! fn check(arg: &str) -> Result<(), Error> {
//!     if arg.is_empty() {
//!         return kind::BAD_ARG.into();
//!     }
//!     Ok(())
//! }
//! ```
//!
//! The [specification reference] is in *tighterror-build*.
//! [cargo-tighterror] writes the code into the source tree instead of a
//! build script, to keep it under source control.
//!
//! [tighterror-build]: https://docs.rs/tighterror-build/latest/tighterror_build
//! [specification reference]: https://docs.rs/tighterror-build/latest/tighterror_build/#specification-file-reference
//! [cargo-tighterror]: https://crates.io/crates/cargo-tighterror
//!
//! # Concepts
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
//! The concrete types are generated from the specification file of the
//! crate.
//!
//! An *error kind* is composed of two parts, *error category* and *error
//! variant*. The *error category* is the group of errors that the *error
//! kind* belongs to, and lets a caller handle the whole group in one match
//! arm. The *error variant* identifies the error within its category.
//!
//! *Error kind* is an unsigned integer rather than an enum, so that both
//! parts, and more information in the future, fit in its bits.
//!
//! [newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
//!
//! ## Layout
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
//! A [wrapped](#wrapped-errors) category stores the whole value of a wrapped
//! *error kind* as its variant. The variant bits are then at least the
//! [`Kind::BITS`] of the wrapped kinds, which is known only when the module
//! is compiled, so the module's kinds grow when the wrapped crate's do. A
//! module that wraps and sets no `repr` takes its underlying type from
//! [`Repr`] at compile time.
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
//! with the `repr` [module setting][module-settings].
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
//! [module-settings]: https://docs.rs/tighterror-build/latest/tighterror_build/#module-settings
//!
//! # Generated API
//!
//! The generated module holds the concrete types and their trait
//! implementations. The error categories and kinds are constants in its
//! `category` and `kind` sub-modules:
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
//! ## Variant Types
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
//! ## Wrapped Errors
//!
//! A crate can wrap the *tighterror* of a crate it builds on in a category
//! of its own. The category names the `Error` and `ErrorKind` types of that
//! crate, and holds no errors:
//!
//! ```kdl
//! module errors {
//!     category General {
//!         error TIMEOUT
//!     }
//!     category Text {
//!         spec {
//!             wrap-error-type text::errors::Error
//!             wrap-kind-type text::errors::ErrorKind
//!         }
//!     }
//! }
//! ```
//!
//! `?` then converts a `text::errors::Error` into the module's `Error`, in
//! the `TEXT` category. A wrapped error kind keeps the name and the display
//! text it has in its crate.
//!
//! A caller matches a wrapped error kind with the constants of its crate
//! through `wrapped()`. It gives the wrapped kind back by its type, or
//! `None` when the kind is in another category, e.g. in a match guard next
//! to the arms on the module's own kinds:
//!
//! ```rust,ignore
//! match e.kind() {
//!     k if let Some(t) = k.wrapped() => match t {
//!         text::errors::kind::NO_FONT => retry(),
//!         _ => fail(),
//!     },
//!     kind::TIMEOUT => wait(),
//!     _ => fail(),
//! }
//! ```
//!
//! `wrapped()` is generated only in a module that wraps the errors of
//! another crate, along with an implementation of [`Wraps`] for every
//! wrapped kind type.
//!
//! The wrapped crate must export both types. A *variant type* of the wrapped
//! crate converts through that crate's `Error`, as in
//! `.map_err(text::errors::Error::from)?`.
//!
//! An error that reaches the module through another wrapping crate is in
//! the category of that crate. So it is a different *error kind* than the
//! same error converted directly, and it comes back with one `wrapped()` per
//! crate on the way.
//!
//! # Motivation
//!
//! Rust makes error reporting hard to forget with [`Result`], which is
//! [`must_use`]. Choosing the error type is still up to the programmer, and
//! this section explains the choices that led to *tighterror*.
//!
//! [`must_use`]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute
//!
//! ## Error as Trait Object
//!
//! Let's take a look at a function with multiple error conditions:
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
//! The function fails to compile: `File::open` and `Read::read_to_string`
//! fail with `io::Error`, while `write!` fails with `fmt::Error`, which `?`
//! can't convert into `io::Error`.
//!
//! A common solution is an error type that holds any error behind a trait
//! object, such as `Box<dyn Error>` or [`anyhow::Error`]: every type that
//! implements `std::error::Error` converts into it. It keeps the
//! information of the original error, at a cost:
//!
//! 1. The conversion allocates memory on the heap.
//! 1. The error may be much larger than the original one, e.g., when the
//!    original one is a ZST.
//! 1. Calling methods of the original error involves dynamic dispatch.
//!
//! In *tighterror* an error condition is a small unsigned integer, the
//! *error kind*. A conversion returns a constant value, which is usually
//! inlined.
//!
//! [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
//!
//! ## Error as Enum
//!
//! Another solution is an `enum` with a variant for every error case, which
//! [`thiserror`] makes short:
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
//! It keeps the information of the inner errors without allocating memory.
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
//! embedding its bits as *error variant* in the outer type, see
//! [Wrapped Errors](#wrapped-errors).
//!
//! ## Source Code Readability
//!
//! An enum without data fields mitigates issues 1-3 of [error as
//! enum](#error-as-enum), and `thiserror` makes it short too. Still, in our
//! opinion, constants read better than enum variants as error kinds. By Rust
//! convention constants are written in upper snake case, e.g.,
//! `CONNECTION_REFUSED`, while enum variants are written in upper camel case,
//! e.g., `ConnectionRefused`, like structs, traits and enums. An error kind is
//! expected to be a constant value, and a constant stands out on the error path
//! during code review.

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

mod wraps;
pub use wraps::*;

mod macros;
