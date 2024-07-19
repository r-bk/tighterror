# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.18], [b-0.0.18], [c-0.0.18] - 2024-07-05

A backward incompatible release.

### Changed

Rename "interface" traits to a shorter form without the `TightError` prefix:

- `TightErrorCategory` -> `Category`
- `TightErrorKind` -> `Kind`
- `TightError` -> `Error`

## [0.0.17], [b-0.0.17], [c-0.0.17] - 2024-06-29

### Changed

- `MainObject::output` can be either absolute path, a relative path or hyphen.
  Till now a relative path was calculated relative to the current working
  directory. Starting from this version a relative path is calculated
  relative to the directory containing the specification file.

### Added

- `MainObject::output` path can now point to a directory. When done so,
  and *separate files* mode is disabled (see below), the output is written to
  file `tighterror.rs` under the directory.

- add support for *separate files* mode. In this mode every module in
  specification is written into a separate file. `MainObject::output` must point
  to an existing directory. The files are written under the directory and
  the filenames are derived from corresponding module names with addition of the
  `.rs` extension.

## [0.0.16], [b-0.0.16], [c-0.0.16] - 2024-06-21

### Changed

- add support for *multiple modules*

  Now it is possible to maintain more than one *tighterror* module in the
  same specification file. This allows having dedicated error types
  for different sub-systems in a large codebase.

  ```yaml
  ---
  modules:
    - name: errors
      categories:
        - name: Parser
          errors:
            - BadToken

    - name: internal_errors
      categories:
        - name: Scheduler
          errors:
            - QueueFull: Scheduler queue is full.
            - Timeout: Operation timed out.
  ```

- add fine-grained errors in `tighterror-build`.

  This allows a better unit-testing because error conditions have more specific
  error codes. Hence, it becomes less possible that a unit has returned an error
  code, but the error condition that returned the error code is not the one that
  the unit-test intended to check.

## [0.0.15], [b-0.0.15], [c-0.0.15] - 2024-05-18

A small release on the way to multiple modules.

### Changed

- add a custom implementation of `core::fmt::Debug` for `ErrorKind` and
  `ErrorCategory`. The default implementation only prints the decimal value
  of the underlying Rust type. The new custom implementation prints the name,
  and in case of `ErrorKind` also the category, of the object.

## [0.0.14], [b-0.0.14], [c-0.0.14] - 2024-04-26

A relatively big release with support for multiple categories.

### Added

- add support for multiple categories

  Now it is possible to define multiple custom categories using the new
  `categories` keyword. For example, the following YAML specification defines
  two error categories `Parsing` and `Network`, each with its own set of errors.
  Note that error names are unique only within a category, i.e., they may repeat
  in different categories.

  ```yaml
  ---
  categories:
    - name: Parsing
      errors:
        - BadToken
        - BadOperator
        - Timeout

    - name: Network
      errors:
        - Timeout
        - ConnectionRefused
  ```

- add a new module-level attribute `flat_kinds`

  By default error names are required to be unique on category level only and
  may repeat in different categories. This forces *tighterror* to place the
  constants within a category-specific sub-module so that repeating error names
  in different categories do not collide. This makes the constant's paths longer
  due to the addition of category-specific module.

  In specifications where error names are unique on module-level, e.g., when
  there is a single category only, the category-specific module in constants'
  paths have no practical function and only make the paths longer.

  When `flat_kinds` is enabled error kind constants are placed directly under
  the `kinds` submodule, e.g. `kinds::ERR`. Error names must be unique on
  module-level.

### Changed

- **(breaking)** error kind constants are now placed within a category-specific
  sub-module. In the following example the error constant path was previously
  `kinds::ERR` and now it is `kinds::general::ERR` (the name of the implicit
  category is `General`):

  ```yaml
  ---
  errors:
    - Err
  ```

  To preserve the old behavior use the new module-level attribute `flat_kinds`:

  ```yaml
  ---
  module:
    flat_kinds: true
  errors:
    - Err
  ```

## [0.0.13], [b-0.0.13], [c-0.0.13] - 2024-03-23

A breaking change to prepare for multiple *tighterror* modules in a single
specification file.

### Changed

- move module-specific attributes from `[tighterror]` to `[module]` section
- remove `mod_doc` attribute and use `doc` instead
- rename `[tighterror]` section to `[main]`

## [0.0.12], [b-0.0.12], [c-0.0.12] - 2024-03-22

A breaking change to clean the interface traits.

### Changed

- rename `TightErrorCategory::ReprType` to `TightErrorCategory::R`
- rename `TightErrorKind::ReprType` to `TightErrorKind::R`
- rename `TightErrorKind::CategoryType` to `TightErrorKind::Category`
- rename `TightError::ReprType` to `TightError::R`
- rename `TightError::CategoryType` to `TightError::Category`
- rename `TightError::KindType` to `TightError::Kind`
- cleanup documentation

## [0.0.11], [b-0.0.11], [c-0.0.11] - 2024-03-16

Finally, some sufficient documentation.

### Changed

- renamed `MainSpec` to `ModuleSpec` in preparation for supporting
  multiple *tighterror* modules in a single specification file

### Added

- add `no_std` module spec attribute. This enables code generation for Rust
  `no_std` crates. At this stage this flag implicitly disables the `error_trait`
  flag, because `std::error::Error` trait isn't available in `core`,
  and transitively in `no_std`. Moreover, this flag skips generation of
  some unit tests that require `std`.

- add sufficient framework documentation in all crates, both for docs.rs and
  in README files.

## [0.0.10], [b-0.0.10], [c-0.0.10] - 2024-03-09

More breaking cleanups on the way to stability and documentation.

### Changed

- rename `cat_doc` keyword to `err_cat_doc` in main specification section
- rename `err_into_result` to `result_from_err` in main specification section
- rename `err_kind_into_result` to `result_from_err_kind` in main specification
  section

## [0.0.9], [b-0.0.9], [c-0.0.9] - 2024-03-01

Another breaking change on the way to more stable documented release.

### Changed

- put Category bits before Variant bits. This affects the numeric value
  of all defined errors. However, except for a diff in the output module it
  shouldn't be noticed otherwise.

- rename Destination to Output. This is a breaking change that affects the
  public API, `cargo-tighterror` arguments, and specification file syntax.
  `CodegenOptions::dst` is renamed to `CodegenOptions::output`. The `-d, --dst`
  argument is renamed to `-o, --output`. The `dst` specification file keyword
  is renamed to `output`.

## [0.0.8], [b-0.0.8], [c-0.0.8] - 2024-02-09

This release is a big breaking change. The interfaces of *tighterror* types are
revised.

### Added

- add *update mode* in which destination file ins't overwritten if the new
  to-be-written data equals the existing one. This avoids updating destination
  file's modification time, and transitively recompilation of a user crate.

### Changed

- rename `error-kind`, i.e., the unique ID of an error in a category,
  to `error-variant`
- rename `ErrorCode` to `ErrorKind`
- update the public interfaces to have less associated constants

## [0.0.7], [b-0.0.7], [c-0.0.7] - 2024-02-02

### Added

- implement the TOML specification file parser

### Changed

- stop using `__` in global statics to prevent `non_upper_case_globals`
  warning in rust-analyzer

## [0.0.6], [b-0.0.6], [c-0.0.6] - 2024-01-06

This release cleans up dead code and introduces a bug fix.

### Fixed

- fix `cargo-tighterror` to properly conform to cargo plugin specification

### Removed

- remove the `lint` feature. It was planned initially. However, its necessity
  isn't clear at this moment.

## [0.0.5], [b-0.0.5], [c-0.0.5] - 2024-01-05

This release makes `tighterror` self-hosted in `tighterror-build`.

### Added

- implement `PartialEq` for the `Error` struct
- explicitly forbid `unsafe` code in all crates

### Changed

- update the `errors` module in `tighterror-build` to use the `tighterror`
  framework itself
- implement negative compilation tests using [compiletest-rs] instead of
  `compile_fail` doc-tests

[compiletest-rs]: https://crates.io/crates/compiletest_rs

## [0.0.4], [b-0.0.4], [c-0.0.4] - 2023-12-30

This is a small incremental release on the way to self-hosted `tighterror`.

### Added

- add support for a custom name for `Error`, `ErrorCategory` and `ErrorCode`
  structs

### Fixed

- check for invalid characters in identifiers; disallow whitespace

## [0.0.3], [b-0.0.3], [c-0.0.3] - 2023-12-22

This is a small incremental release bringing `tighterror` closer to the first
milestone of being self-hosted in its inner crates.

### Added

- add `From<ErrorCode> for Error`
- add an auto-generated unit-test
- add a new argument `-t/--test` to `cargo-tighterror` to include the unit-test
  in the generated code
- add `err_into_result` setting to enable generation of `From<Error> for Result<T, Error>`
- add `err_code_into_result` setting to enable generation of `From<ErrorCode> for Result<T, Error>`

### Changed

- enhance the Yaml parser unit test
- enhance `CodegenOptions` documentation
- `Display for ErrorCode` now shows the error name. The content of `display`
  attribute is used in `Display for Error`
- update documentation accordingly

## [0.0.2], [b-0.0.2], [c-0.0.2] - 2023-12-09

This is the initial development release with basic implementation. Its goal is
to create a basis for the following incremental updates.

The tagging scheme that will be maintained in the project is as follows:

- `vX.Y.Z` - tracks the changes in the main `tighterror` package
- `b-X.Y.Z` - tracks the changes in the `tighterror-build` package
- `c-X.Y.Z` - tracks the changes in the `cargo-tighterror` package

## [0.0.1] - 2023-10-27

- crates.io placeholder

[0.0.1]: https://github.com/r-bk/tighterror/releases/tag/v0.0.1
[0.0.2]: https://github.com/r-bk/tighterror/releases/tag/v0.0.2
[b-0.0.2]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.2
[c-0.0.2]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.2
[0.0.3]: https://github.com/r-bk/tighterror/releases/tag/v0.0.3
[b-0.0.3]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.3
[c-0.0.3]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.3
[0.0.4]: https://github.com/r-bk/tighterror/releases/tag/v0.0.4
[b-0.0.4]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.4
[c-0.0.4]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.4
[0.0.5]: https://github.com/r-bk/tighterror/releases/tag/v0.0.5
[b-0.0.5]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.5
[c-0.0.5]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.5
[0.0.6]: https://github.com/r-bk/tighterror/releases/tag/v0.0.6
[b-0.0.6]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.6
[c-0.0.6]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.6
[0.0.7]: https://github.com/r-bk/tighterror/releases/tag/v0.0.7
[b-0.0.7]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.7
[c-0.0.7]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.7
[0.0.8]: https://github.com/r-bk/tighterror/releases/tag/v0.0.8
[b-0.0.8]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.8
[c-0.0.8]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.8
[0.0.9]: https://github.com/r-bk/tighterror/releases/tag/v0.0.9
[b-0.0.9]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.9
[c-0.0.9]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.9
[0.0.10]: https://github.com/r-bk/tighterror/releases/tag/v0.0.10
[b-0.0.10]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.10
[c-0.0.10]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.10
[0.0.11]: https://github.com/r-bk/tighterror/releases/tag/v0.0.11
[b-0.0.11]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.11
[c-0.0.11]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.11
[0.0.12]: https://github.com/r-bk/tighterror/releases/tag/v0.0.12
[b-0.0.12]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.12
[c-0.0.12]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.12
[0.0.13]: https://github.com/r-bk/tighterror/releases/tag/v0.0.13
[b-0.0.13]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.13
[c-0.0.13]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.13
[0.0.14]: https://github.com/r-bk/tighterror/releases/tag/v0.0.14
[b-0.0.14]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.14
[c-0.0.14]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.14
[0.0.15]: https://github.com/r-bk/tighterror/releases/tag/v0.0.15
[b-0.0.15]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.15
[c-0.0.15]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.15
[0.0.16]: https://github.com/r-bk/tighterror/releases/tag/v0.0.16
[b-0.0.16]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.16
[c-0.0.16]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.16
[0.0.17]: https://github.com/r-bk/tighterror/releases/tag/v0.0.17
[b-0.0.17]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.17
[c-0.0.17]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.17
[0.0.18]: https://github.com/r-bk/tighterror/releases/tag/v0.0.18
[b-0.0.18]: https://github.com/r-bk/tighterror/releases/tag/b-0.0.18
[c-0.0.18]: https://github.com/r-bk/tighterror/releases/tag/c-0.0.18
