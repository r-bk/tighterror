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
//!    - specify errors in markup language and generate all the boilerplate
//!      Rust code automatically
//!    - ergonomic error reporting
//!
//! # Table Of Contents
//!
//! 1. [High-Level Overview](#high-level-overview)
//! 1. [Implementation Details](#implementation-details)
//! 1. [Specification File Reference](#specification-file-reference)
//!     * [Filename](#filename)
//!     * [Error Object](#error-object)
//!     * [Error List](#error-list)
//!     * [Category Object](#category-object)
//!     * [Category List](#category-list)
//!     * [Module Object](#module-object)
//!     * [Main Object](#main-object)
//! 1. [tighterror-build](#tighterror-build)
//! 1. [cargo-tighterror](#cargo-tighterror)
//! 1. [Motivation](#motivation)
//!     * [Error as Trait Object](#error-as-trait-object)
//!     * [Error as Enum](#error-as-enum)
//!     * [Source Code Readability](#source-code-readability)
//!
//! ## High-Level Overview
//!
//! In its basic form a *tighterror* is a [newtype] around a small unsigned
//! integer, e.g., `u8`.<br>
//! Error conditions are identified by an error [`kind`](TightError::kind).
//! Every *error kind* has a [`category`](TightErrorKind::category) that
//! identifies the logical group the error belongs to.
//!
//! ```rust
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! #[repr(transparent)]
//! pub struct ErrorKind(u8);
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
//! - [`TightErrorCategory`] for `ErrorCategory`
//! - [`TightErrorKind`] for `ErrorKind`
//! - [`TightError`] for `Error`
//!
//! The concrete types are auto-generated from a specification file defined in
//! the user project. The specification file is written in one of the supported
//! markup languages. For example, `tighterror.yaml`:
//!
//! ```yaml
//! ---
//! errors:
//!   - BadFile
//!   - BadArg
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
//! placed in the `categories` and `kinds` sub-modules respectively:
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
//! pub mod categories {
//! #   use super::*;
//!     pub const GENERAL: ErrorCategory = ErrorCategory::new(0);
//! }
//!
//! pub mod kinds {
//! #   use super::*;
//!     pub const BAD_FILE: ErrorKind = ErrorKind::new(categories::GENERAL, 0);
//!     pub const BAD_ARG: ErrorKind = ErrorKind::new(categories::GENERAL, 1);
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
//! allocation of bits suitable for 4 categories with a maximum of 32 errors
//! within each category:
//!
//! ```text
//!        ┌───┬───┬───┬───┬───┬───┬───┬───┐
//!        │   │   │   │   │   │   │   │   │   0-1 2 category bits
//!      u8│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │   2-6 5 variant bits
//!        │   │   │   │   │   │   │   │   │   7-7 1 reserved bit
//!        └───┼───┴───┴───┴───┴───┼───┴───┤
//!            │      variant      │  cat  │
//! ```
//!
//! The number of *variant* bits is large enough to accommodate the
//! largest *category*. Therefore some categories may have more *reserved*
//! bits than others, assuming not all categories are of the same length.
//!
//! When there is only a single *error category* the number of *category* bits
//! is 0.
//!
//! By default *tighterror* chooses the smallest unsigned integer that is
//! large enough to accommodate a required layout. Supported underlying
//! types are `u8`, `u16`, `u32` and `u64`.
//!
//! Note that the layout of *error kind* is packed and may change
//! during project's lifetime. For example, addition of a new *error category*
//! may lead to a left-shift of *variant* bits, thus changing the numeric
//! value of all *error kinds* of the project, and possibly changing the
//! underlying Rust type to a wider unsigned integer.
//! Consequently, *error kinds* should be matched only using named constants
//! defined by their origin crate. No assumption should be made about a named
//! constant having any specific numeric value.
//!
//! [`tighterror-build`]: #tighterror-build
//! [`cargo-tighterror`]: #cargo-tighterror
//!
//! # Specification File Reference
//!
//! *tighterror* uses markup languages for its specification file.
//! Currently supported languages are [YAML](https://yaml.org) and
//! [TOML](https://toml.io).
//!
//! ## Filename
//!
//! When specification filename isn't explicitly provided *tighterror-build*
//! searches the current working directory for a file with one of the default
//! names `tighterror.yaml` or `tighterror.toml`.
//!
//! Custom filenames, or files outside the current working directory,
//! must be explicitly specified. A custom filename must have one of the
//! supported markup language extensions `.yaml` or `.toml`.
//!
//! ## Error Object
//!
//! An *error object* is comprised from the following attributes:
//!
//! ```yaml
//! name: <string> (required)
//! display: <string>
//! doc: <string>
//! doc_from_display: <bool>
//! ```
//!
//! * `name` - string (required)
//!
//!   Defines the *error kind* name. Must be a string in UpperCamelCase.
//!   This string becomes an *error kind* constant after conversion to
//!   UPPER_SNAKE_CASE.<br><br>
//!
//! * `display` - string (optional)
//!
//!   Defines the *error kind's* display string. This string is used in
//!   `std::fmt::Display` implementation to display the *error kind*.<br>
//!   When undefined the *error kind's* UPPER_SNAKE_CASE `name` is used
//!   as display string.<br><br>
//!
//! * `doc` - string (optional)
//!
//!   Defines the error's documentation comment. This becomes the
//!   doc comment of the *error kind's* constant.
//!   When undefined the constant doesn't receive a doc comment.<br><br>
//!
//! * `doc_from_display` - bool (optional)<a name="err-obj-doc-from-display"></a>
//!
//!   When enabled the `display` string is also used as the doc comment,
//!   unless `doc` is defined.
//!   The `display` attribute must be explicitly set for this to take effect.<br>
//!
//!   This attribute can be set on a higher level in
//!   [category specification](#category-doc-from-display) to affect all
//!   errors in the category, or in
//!   [module specification](#module-doc-from-display)
//!   to affect all errors in the module.
//!   Values defined on lower levels win.<br>
//!   Default: `false`<br>
//!
//! An *error object* never appears standalone. It must be defined as an item
//! in an [*error list*](#error-list).
//!
//! ### Error Object Examples
//!
//! **YAML**
//!
//! ```yaml
//! errors:
//!   - name: BadPath
//!     display: Path is malformed.
//!     doc: Returned when a path value has an invalid structure.
//!     doc_from_display: false
//! ```
//!
//! **TOML**
//!
//! ```toml
//! [[errors]]
//! name = "BadPath"
//! display = "Path is malformed."
//! doc = "Returned when a path value has an invalid structure."
//! doc_from_display = false
//! ```
//!
//! [identifier requirements]: https://doc.rust-lang.org/reference/identifiers.html
//!
//! ## Error List
//!
//! *Error list* is an ordered list of [*error objects*](#error-object)
//! with unique names. It is identified by the `errors` keyword.
//!
//! The `errors` keyword may appear as a standalone root-level attribute or as
//! an attribute of a [*category object*](#category-object).
//! When specified at the root-level the specification file defines only a single
//! category and the list defines all the errors of the specification.
//! When specified at *category object* level the specification file can define
//! more than one category, and the list defines errors belonging to the
//! category in which it is specified. The root-level and per-category notations
//! are mutually exclusive.
//!
//! To allow more compact specification of errors *tighterror* supports
//! shorthand notations where part of *error object* attributes are
//! specified without corresponding keywords (see the examples below).
//!
//! The order of elements in a list matters because *error variant* is
//! assigned to errors in the order of their appearance in the list.
//! Therefore, insertion or deletion of a not-last object causes
//! reassignment of *error variant* of all subsequent objects in the list.
//! This may create a larger than expected diff in the generated code.
//!
//! ### YAML Error List
//!
//! In general an *error list* is an array of error objects where every
//! item can have any of the [*error object's*](#error-object) attributes.
//! The `name` attribute is mandatory.
//!
//! ```yaml
//! ---
//! errors:
//!   - name: MissingAttribute
//!     display: An object attribute is missing.
//!     doc_from_display: true
//!   - name: Timeout
//!     display: Operation timed out.
//!     doc_from_display: true
//! ```
//!
//! **Name-Only Notation**
//!
//! *Error object* has only a single mandatory attribute, the `name`.
//! This allows a short *name-only* notation, where every element
//! of the list is an error name specified without the `name` keyword.
//!
//! This notation doesn't allow specification of attributes other than `name`.
//! Therefore they receive default values. In particular, an *error kind*
//! constant has no doc comment and is displayed as UPPER_SNAKE_CASE of the
//! name.
//!
//! ```yaml
//! ---
//! errors:
//!   - MissingAttribute
//!   - Timeout
//! ```
//!
//! **Name-Display Notation**
//!
//! *Name-display* notation slightly enhances the *name-only* notation
//! with an addition of a display string, without using the `display`
//! keyword. Every element in the list is a `name: display` single-item mapping.
//! This notation is available in YAML only.
//!
//! ```yaml
//! ---
//! errors:
//!   - MissingAttribute: An object attribute is missing.
//!   - Timeout: Operation timed out.
//! ```
//!
//! ### TOML Error List
//!
//! In TOML an *error list* is an array of tables:
//!
//! ```toml
//! [[errors]]
//! name = "MissingAttribute"
//! display = "An object attribute is missing."
//! doc_from_display = true
//!
//! [[errors]]
//! name = "Timeout"
//! display = "Operation timed out."
//! doc_from_display = true
//! ```
//!
//! or array of inline tables:
//!
//! ```toml
//! errors = [
//!     { name = "MissingAttribute", display = "An object attribute is missing.", doc_from_display = true },
//!     { name = "Timeout", display = "Operation timed out.", doc_from_display = true }
//! ]
//! ```
//!
//! **Name-Only Notation**
//!
//! *Name-only* shorthand notation is available in TOML as follows:
//!
//! ```toml
//! errors = ["MissingAttribute", "Timeout"]
//! ```
//!
//! ## Category Object
//!
//! A *category object* is comprised from the following attributes:
//!
//! ```yaml
//! name: <string> (required*)
//! doc: <string>
//! doc_from_display: <bool>
//! errors: <ErrorList> (required*)
//! ```
//!
//! * `name` - string
//!
//!   Defines the name of the category.<br>
//!
//!   The value must be a valid Rust struct name specified in UpperCamelCase.
//!   This string becomes a *category* constant after transition to
//!   UPPER_SNAKE_CASE.
//!
//!   The name is a mandatory attribute when *category object* is defined
//!   as an item in a [*category list*](#category-list).
//!   Otherwise it is optional with the default value `General`.<br><br>
//!
//! * `doc` - string (optional)
//!
//!   Defines the category's document string.<br>
//!
//!   This becomes the doc comment of the category constant.<br><br>
//!
//! * `doc_from_display` - bool (optional)<a name="category-doc-from-display"></a>
//!
//!   Sets a default value for the [`doc_from_display`](#err-obj-doc-from-display)
//!   *error object* attribute. This affects errors belonging to this category
//!   only. The value is used for all errors that do not define it
//!   explicitly as part of *error object* specification.
//!
//!   Usage of this attribute allows addition of a doc comment in *name-display*
//!   error list notation, which otherwise isn't possible.<br><br>
//!
//! * `errors` - ErrorList
//!
//!   Defines the list of errors belonging to this category.<br>
//!
//!   This is a mandatory attribute when *category object* is defined
//!   as an item in a [*category list*](#category-list). Conversely, when
//!   a *category object* is defined as a standalone root-level attribute
//!   (see below) this attribute is forbidden, and the error list must be
//!   defined as a root-level attribute.
//!
//!   See the [*error list*](#error-list) section for more information.<br><br>
//!
//! A *category object* can appear as a standalone root-level attribute
//! `category` or as an item in a [*category list*](#category-list).
//! Definition at the root-level allows specification of a single category.
//! Definition as an item in a *category list* allows specification
//! of multiple categories.
//!
//! When a *category object* is specified at the root-level its `errors`
//! attribute is forbidden, it must be defined at the root-level instead.
//! Therefore, the whole *category object* at the root-level is
//! optional because all other attributes have default values.
//!
//! When no *category object* is defined *tighterror* creates an implicit
//! `General` category.
//!
//! When there is only a single category the number of *category* bits is zero.
//!
//! See [*category list*](#category-list) documentation for more information.
//!
//! ### Category Object Examples
//!
//! A specification with a single implicit category `General`. Note, the error
//! constants have no documentation in this scenario.
//!
//! ```yaml
//! ---
//! errors:
//!   - MissingAttribute: An object attribute is missing.
//!   - Timeout: Operation timed out.
//! ```
//!
//! A specification with a single category with a custom name. Note, the
//! error constants receive a doc string that is equal to the display string.
//!
//! ```yaml
//! ---
//! category:
//!   name: MyErrorCategory
//!   doc_from_display: true
//!
//! errors:
//!   - MissingAttribute: An object attribute is missing.
//!   - Timeout: Operation timed out.
//! ```
//!
//! A TOML specification with a single category with a custom name and
//! documentation equal to display string.
//!
//! ```toml
//! [category]
//! name = "MyErrorCategory"
//! doc_from_display = true
//!
//! [[errors]]
//! name = "MissingAttribute"
//! display = "An object attribute is missing."
//!
//! [[errors]]
//! name = "Timeout"
//! display = "Operation timed out"
//! ```
//!
//! ## Category List
//!
//! *Category list* is an ordered list of [*category objects*](#category-object)
//! with unique names. It is identified by the `categories` keyword.
//!
//! The `categories` keyword may appear as a standalone root-level attribute of a
//! specification file. It allows definition of one or more categories, each
//! with its own set of errors.
//!
//! Note that when a *category object* is defined as an item in a
//! *category list* the `name` and `errors` attributes are mandatory.
//!
//! Also note that `category` and `categories` keywords are mutually exclusive,
//! as well as root-level `errors` and `categories` keywords. `category` and
//! root-level `errors` keywords are used only for a single-category
//! specification.
//!
//! The order of elements in a list matters because category constant identifier
//! is assigned in the order of their appearance in the list. Therefore,
//! insertion or deletion of a non-last category causes reassignment of
//! category identifiers of all subsequent objects in the list. This may create
//! a larger than expected diff in the generated code.
//!
//! ### Category List Examples
//!
//! A two-category YAML example.
//!
//! ```yaml
//! ---
//! categories:
//!   - name: Parsing
//!     doc_from_display: true
//!     errors:
//!       - MissingAttribute: An object attribute is missing.
//!       - InvalidAttribute: An object contains an invalid attribute.
//!
//!   - name: CodeGeneration
//!     doc_from_display: true
//!     errors:
//!       - InvalidName: An object has an invalid name.
//!       - InvalidPath: A file path is invalid.
//! ```
//!
//! An equivalent two-category TOML example.
//!
//! ```toml
//! [[categories]]
//! name = "Parsing"
//! doc_from_display = true
//!
//! [[categories.errors]]
//! name = "MissingAttribute"
//! display = "An object attribute is missing."
//!
//! [[categories.errors]]
//! name = "InvalidAttribute"
//! display = "An object contains an invalid attribute."
//!
//! [[categories]]
//! name = "CodeGeneration"
//! doc_from_display = true
//!
//! [[categories.errors]]
//! name = "InvalidName"
//! display = "An object has an invalid name."
//!
//! [[categories.errors]]
//! name = "InvalidPath"
//! display = "A file path is invalid."
//! ```
//!
//! ## Module Object
//!
//! The module object is identified by the `module` keyword
//! and is found at the root-level of a specification file.
//! It defines attributes of objects that are present at the Rust module level
//! and below.
//!
//! All attributes of the *module object* have default values. Therefore,
//! the whole `module` section is optional.
//!
//! ```yaml
//! ---
//! module:
//!   doc: <string>
//!   doc_from_display: <bool>
//!   err_cat_doc: <string>
//!   err_cat_name: <string>
//!   err_doc: <string>
//!   err_kind_doc: <string>
//!   err_kind_name: <string>
//!   err_name: <string>
//!   error_trait: <bool>
//!   flat_kinds: <bool>
//!   result_from_err: <bool>
//!   result_from_err_kind: <bool>
//! ```
//!
//! * `doc` - string (optional)
//!
//!   Defines the doc comment of the generated module.<br>
//!   By default module doc comment is not defined.<br><br>
//!
//! * `doc_from_display` - bool (optional)<a name="module-doc-from-display"></a>
//!
//!   Sets a default value for the [`doc_from_display`](#err-obj-doc-from-display)
//!   *error object* attribute.
//!   This value is used for all errors that do not define it
//!   explicitly as part of *error object* specification.
//!
//!   Usage of this attribute allows addition of a doc comment in *name-display*
//!   error list notation, which otherwise isn't possible.
//!
//!   ```yaml
//!   ---
//!   module:
//!     doc_from_display: true
//!   errors:
//!     - MissingAttribute: An object attribute is missing.
//!     - Timeout: Operation timed out.
//!   ```
//!   <br>
//!
//! * `err_cat_doc` - string (optional)
//!
//!   Defines the doc comment of the *error category* struct.
//!
//!   This becomes the doc comment of the struct that implements the
//!   [TightErrorCategory] trait.<br>
//!   The default value is equivalent to the following YAML specification:
//!   ```yaml
//!   ---
//!   module:
//!     err_cat_doc: |
//!       Error category type.
//!
//!       See the [categories] module for category constants.
//!   ```
//!   <br>
//!
//! * `err_cat_name` - string (optional)
//!
//!   Defines the name of the *error category* struct.
//!
//!   This becomes the name of the struct that implements the
//!   [TightErrorCategory] trait.<br>
//!   The value must be a valid Rust struct name.<br>
//!   Default: `ErrorCategory`<br><br>
//!
//! * `err_doc` - string (optional)
//!
//!   Defines the doc comment of the *error* struct.
//!
//!   This becomes the doc comment of the struct that implements the
//!   [TightError] trait.<br>
//!   The default value is equivalent to the following YAML specification:
//!   ```yaml
//!   ---
//!   module:
//!     err_doc: |
//!       Error type.
//!
//!       See the [kinds] module for error kind constants.
//!   ```
//!   <br>
//!
//! * `err_kind_doc` - string (optional)
//!
//!   Defines the doc comment of the *error kind* struct.
//!
//!   This becomes the doc comment of the struct that implements the
//!   [TightErrorKind] trait.<br>
//!   The default value is equivalent to the following YAML specification:
//!   ```yaml
//!   ---
//!   module:
//!     err_kind_doc: |
//!       Error kind type.
//!
//!       See the [kinds] module for error kind constants.
//!   ```
//!   <br>
//!
//! * `err_kind_name` - string (optional)
//!
//!   Defines the name of the *error kind* struct.
//!
//!   This becomes the name of the struct that implements the [TightErrorKind]
//!   trait.<br>
//!   The value must be a valid Rust struct name.<br>
//!   Default: `ErrorKind`<br><br>
//!
//! * `err_name` - string (optional)
//!
//!   Defines the name of the *error* struct.
//!
//!   This becomes the name of the struct that implements the [TightError]
//!   trait.<br>
//!   The value must be a valid Rust struct name.<br>
//!   Default: `Error`<br><br>
//!
//! * `error_trait` - bool (optional)
//!
//!   When enabled implements `std::error::Error` trait on the *error*
//!   struct.<br>
//!   This attribute is ignored when `no_std` is enabled.<br>
//!   Default: `true`<br><br>
//!
//! * `flat_kinds` - bool (optional)
//!
//!   Puts the error kind constants directly under the `kinds` sub-module
//!   instead of under `kinds::<category-module-name>` sub-sub-module.
//!
//!   By default error kind constants are placed in a category-specific
//!   sub-module. For example:
//!
//!   ```yaml
//!   categories:
//!     - name: Foo
//!       errors:
//!         - Err
//!     - name: Baz
//!       errors:
//!         - Err
//!   ```
//!   The error kind constants' paths are `kinds::foo::ERR` and
//!   `kinds::baz::ERR`. The category-specific sub-modules `foo` and
//!   `baz` are required because by default error names are required to be
//!   unique on category level only.
//!
//!   There are cases when error names are unique per module. In these
//!   cases having the category-specific module in the constants' paths
//!   has no practical function and just makes the paths longer.
//!
//!   The `flat_kinds` attribute forces *tighterror* to put the constants
//!   directly under the `kinds` sub-module, e.g., `kinds::ERR`.
//!   Enabling this option requires the error names to be unique
//!   on module level (and not only on category level).
//!
//!   ```yaml
//!   module:
//!     flat_kinds: true
//!
//!   categories:
//!     - name: Foo
//!       errors:
//!         - Err
//!     - name: Baz
//!       errors:
//!         - AnotherErr
//!   ```
//!
//!   Default: `false`<br><br>
//!
//! * `result_from_err` - bool (optional)
//!
//!   When enabled an implementation of [From] trait is added
//!   to create a `Result<T, Error>` from `Error`.<br>
//!   Default: `true`<br><br>
//!
//! * `result_from_err_kind` - bool (optional)
//!
//!    When enabled an implementation of [From] trait is added
//!    to create a `Result<T, Error>` from `ErrorKind`.<br>
//!    Default: `true`<br><br>
//!
//! ## Main Object
//!
//! The main configuration object is identified by the `main`
//! keyword and is found at the root-level of a specification file. It defines
//! attributes that affect the code generation globally, applicable to all
//! other specification objects in the file.
//!
//! All attributes of the *main object* have default values. Therefore, the
//! whole `main` section is optional.
//!
//! The *main object* is comprised of the following attributes:
//!
//! ```yaml
//! main:
//!   no_std: <bool>
//!   output: <string>
//! ```
//!
//! * `no_std` - bool (optional)
//!
//!   Generates code suitable for Rust `no_std` environment.
//!
//!   When enabled this attribute implicitly disables `error_trait` and
//!   skips unit tests that require `std`.<br>
//!   Default: `false`<br><br>
//!
//! * `output` - string (optional)
//!
//!   Defines the output file path. When undefined the output is written to
//!   `stdout`.
//!
//!   The value `-` forces the output to be written to `stdout`.
//!
//!   This attribute is overridden by the `-o, --output` command-line
//!   argument in *cargo-tighterror*.<br><br>
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
//! Note that the plugin needs to be periodically upgraded to follow
//! releases of [`tighterror-build`].
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
//!   -s, --spec <PATH>    The specification file path
//!   -o, --output <PATH>  The output file path
//!   -t, --test           Include a unit-test in the generated code
//!   -u, --update         Do not overwrite the output file if data is unchanged
//!   -h, --help           Print help
//!   -V, --version        Print version
//! ```
//!
//! * `-s, --spec <PATH>` (optional)
//!
//!   Defines the specification file path.
//!
//!   It is required if a specification file has a custom name.
//!   It can be omitted if the specification file uses the default
//!   name `tighterror.yaml` or `tighterror.toml` and is present in the
//!   current working directory.
//!
//!   Note that the file extension `.yaml` or `.toml` is mandatory in
//!   custom filenames.<br><br>
//!
//! * `-o, --output <PATH>` (optional)
//!
//!   Defines the output file path.
//!
//!   This argument overrides the `output` attribute defined in the
//!   specification file. If this argument is omitted and output file
//!   path isn't defined in the specification file the output is written to
//!   `stdout`.
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
