# _tighterror_ - compact error types + codegen

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/tighterror.svg
[crates-url]: https://crates.io/crates/tighterror
[docs-badge]: https://img.shields.io/docsrs/tighterror
[docs-url]: https://docs.rs/tighterror/latest/tighterror

**tighterror** is a minimalistic framework for error representation in Rust.

It is heavily inspired by the standard `std::io::Error` which
represents a group of errors in a single Rust type. This concept is taken
one step further such that a _tighterror_ can represent multiple groups of
errors in a single Rust type.

## Goals

_tighterror_ tries to achieve the following goals:

1. Minimize the size of error types while keeping support for idiomatic error
   reporting.
1. Minimize the runtime overhead on error path. No dynamic memory allocation
   (by default).
1. Minimize the runtime overhead of matching a group of errors.
1. Minimize the coding effort:
   - define errors in a KDL file and generate Rust boilerplate code
     automatically
   - ergonomic error reporting

## Features

- [x] KDL specification
- [x] Categories
  - [x] single
  - [x] multiple
  - [x] flat kinds
- [x] Modules
  - [x] single
  - [x] multiple
  - [x] separate files
- [x] Variant error types
- [x] Wrapped errors of other crates
- [x] Code generation
  - [x] in a build script
  - [x] into the source tree

Next:

- [ ] Category error types

Future options:

- [ ] Flags
- [ ] Location
- [ ] Backtrace

## Introduction

_tighterror_ takes the declarative approach where a user project maintains its
set of errors in a specification file written in [KDL](https://kdl.dev).
The Rust code is generated from the specification file in a build script, by
the [tighterror-build] library of the framework.

In its basic form a _tighterror_ is a _newtype_ around a small non-zero unsigned
integer, e.g., `NonZeroU8`, whose value is a unique identifier of the error, the
_error kind_.

```rust
#[repr(transparent)]
pub struct ErrorKind(NonZeroU8);

#[repr(transparent)]
pub struct Error(ErrorKind);  // <-- the type to use in Result<T, E>
```

An _error kind_ is built bitwise from two parts: _error category_ and
_error variant_.
The _error category_ groups several errors logically and allows
handling the whole group with a single match arm.
The _error variant_ is a unique identifier of the error within its _category_.
Together _category_ and _variant_ comprise a unique identifier of an error
in a given _tighterror_ instantiation.

The following diagram shows an example layout of _error kind_. This layout
can represent up to 4 categories with a maximum of 31 errors in each category.

```text
            ┌───┬───┬───┬───┬───┬───┬───┬───┐
            │   │   │   │   │   │   │   │   │ (0-4) 5 variant bits
          u8│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │ (5-6) 2 category bits
            │   │   │   │   │   │   │   │   │ (7-7) 1 reserved bit
            └───┼───┴───┼───┴───┴───┴───┴───┤
                │  cat  │      variant      │
```

By default **tighterror** chooses the smallest underlying type that is big
enough to accommodate the number of categories and errors defined in a
specification file. Supported underlying types are `u8`, `u16`, `u32` and `u64`.

## Example

Below is a minimalistic example suitable for projects where documentation
isn't required. For more configuration options see the [crate documentation].

Define errors in `tighterror.kdl` file in the root directory of your project:

```kdl
module errors {
    category General {
        error BAD_ARG
        error BAD_FILE_PATH
    }
}
```

> [!NOTE]
> When there is only a single category the number of _category_ bits is 0.

Add **tighterror** to your project, and **tighterror-build** as a
build-dependency:

```shell
cargo add tighterror
cargo add --build tighterror-build
```

Generate the code in the build script `build.rs`:

```rust
fn main() -> Result<(), tighterror_build::errors::TbError> {
    tighterror_build::build()
}
```

Include the generated module in your project:

```rust
/// Crate errors.
pub mod errors {
    tighterror::include_errors!(errors);
}
```

> [!NOTE]
> To keep the generated code in the source tree instead, e.g., for review,
> generate it with the cargo plugin [cargo-tighterror].

The following is a stripped-down view of the generated module:

```rust
#[repr(transparent)]
pub struct ErrorCategory(u8);

#[repr(transparent)]
pub struct ErrorKind(NonZeroU8);

#[repr(transparent)]
pub struct Error(ErrorKind);

impl tighterror::Category for ErrorCategory { ... }
impl tighterror::Kind for ErrorKind { ... }
impl tighterror::Error for Error { ... }

pub mod category {
   use super::ErrorCategory;
   pub const GENERAL: ErrorCategory = ErrorCategory::new(0);
}

pub mod kind {
   use super::category::*;
   use super::ErrorKind;
   pub const BAD_ARG: ErrorKind = ErrorKind::new(GENERAL, 1);
   pub const BAD_FILE_PATH: ErrorKind = ErrorKind::new(GENERAL, 2);
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
[tighterror-build]: https://docs.rs/tighterror-build/latest/tighterror_build
[cargo-tighterror]: https://crates.io/crates/cargo-tighterror

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
