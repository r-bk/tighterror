# *tighterror* - compact error types + codegen

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/tighterror.svg
[crates-url]: https://crates.io/crates/tighterror
[docs-badge]: https://img.shields.io/docsrs/tighterror
[docs-url]: https://docs.rs/tighterror/latest/tighterror

**tighterror** is a minimalistic framework for error representation in Rust.

It is heavily inspired by the standard `std::io::Error` which
represents a group of errors in a single Rust type. This concept is taken
one step further such that a *tighterror* can represent multiple groups of
errors in a single Rust type.

## Goals

*tighterror* tries to achieve the following goals:

1. Minimize the size of error types while keeping support for idiomatic error
   reporting.
1. Minimize the runtime overhead on error path. No dynamic memory allocation
   (by default).
1. Minimize the runtime overhead of handling a group of errors.
1. Minimize the coding effort:
   - define errors in a markup language and generate Rust boilerplate code
     automatically
   - ergonomic error reporting

## Features

- [x] YAML specification
- [x] TOML specification
- [ ] Categories
  - [x] implicit
  - [x] single
  - [ ] multiple
- [ ] Modules
  - [x] implicit
  - [x] single
  - [ ] multiple
  - [ ] flat kinds
- [ ] Variant error types
- [ ] Category error types
- [ ] Flags
- [ ] Location
- [ ] Backtrace

## Introduction

*tighterror* takes the declarative approach where a user project maintains its
set of errors in a specification file written in a markup language (YAML or
TOML). Generation of Rust source code out of the specification file is done by
a dedicated tool provided by the framework.

In its basic form a *tighterror* is a *newtype* around a small unsigned integer,
e.g., `u8`, whose value is a unique identifier of the error, the *error kind*.

```rust
#[repr(transparent)]
pub struct ErrorKind(u8);

#[repr(transparent)]
pub struct Error(ErrorKind);  // <-- the type to use in Result<T, E>
```

An *error kind* is built bitwise from two parts: *error category* and
*error variant*.
The *error category* groups several errors logically and allows
handling the whole group with a single match arm.
The *error variant* is a unique identifier of the error within its *category*.
Together *category* and *variant* comprise a unique identifier of an error
in a given *tighterror* instantiation.

The following diagram shows an example layout of *error kind*. This layout
can represent up to 4 categories with a maximum of 32 errors in each category.
Note that usually not all categories are of the same length, hence the number of
reserved bits varies among categories.

```text
             ┌───┬───┬───┬───┬───┬───┬───┬───┐
             │   │   │   │   │   │   │   │   │   0-1 2 category bits
           u8│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │   2-6 5 variant bits
             │   │   │   │   │   │   │   │   │   7-7 1 reserved bits
             └───┼───┴───┴───┴───┴───┼───┴───┤
                 │      variant      │  cat  │
```

By default **tighterror** chooses the smallest underlying type that is big
enough to accommodate the number of categories and errors defined in a
specification file. Supported underlying types are `u8`, `u16`, `u32` and `u64`.

## Example

Below is a minimalistic example suitable for projects where documentation
isn't required. For more configuration options see the [crate documentation].

Define errors in `tighterror.yaml` file in the root directory of your project:

```yaml
---
errors:
  - BadArg
  - BadFilePath
```

> [!NOTE]
> The example above defines only the list of errors and doesn't specify any
> categories. In such case **tighterror** creates an implicit `General` category.
>
> When there is only a single category the number of *category* bits is 0.

Add **tighterror** to your Cargo.toml file:

```shell
cargo add tighterror
```

Install the cargo plugin:

```shell
cargo install cargo-tighterror
```

Run the plugin to generate a Rust module in `src/errors.rs`:

```shell
cargo tighterror -o src/errors.rs
```

Include the generated module in your project as any other handwritten Rust
module, i.e., `mod errors;`.

> [!NOTE]
> It is recommended to put both `tighterror.yaml` and `src/errors.rs` under
> source control for visibility and tracking of changes.

The following is a stripped-down view of the generated module:

```rust
#[repr(transparent)]
pub struct ErrorCategory(u8);

#[repr(transparent)]
pub struct ErrorKind(u8);

#[repr(transparent)]
pub struct Error(ErrorKind);

impl tighterror::TightErrorCategory for ErrorCategory { ... }
impl tighterror::TightErrorKind for ErrorKind { ... }
impl tighterror::TightError for Error { ... }

pub mod categories {
   use super::ErrorCategory;
   pub const GENERAL: ErrorCategory = ErrorCategory::new(0);
}

pub mod kinds {
   use super::categories::*;
   use super::ErrorKind;
   pub const BAD_ARG: ErrorKind = ErrorKind::new(GENERAL, 0);
   pub const BAD_FILE_PATH: ErrorKind = ErrorKind::new(GENERAL, 1);
}
```

`Error` is the type to use in `Result<T, E>`. It can be created as follows:

```rust
let e = Error::from(BAD_ARG);
let e: Error = BAD_ARG.into();

// ErrorKind is convertible to Result<T, Error>
fn foo() -> Result<(), Error> {
  BAD_FILE_PATH.into()
}
assert!(foo().is_err_and(|e| e.kind() == BAD_FILE_PATH));
```

## Documentation

The full documentation is on [docs.rs].

[crate documentation]: https://docs.rs/tighterror/latest/tighterror
[docs.rs]: https://docs.rs/tighterror/latest/tighterror

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Tags

`error` `kind` `minimal` `error reporting` `lean` `error handling` `error-kind`
`error-code` `code`
