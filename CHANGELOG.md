# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.6], [b-0.0.6], [c-0.0.6] - 2024-01-06

This relase cleans up dead code and introduces a bug fix.

### Fixed

- fix `cargo-tighterror` to properly conform to cargo plugin specification

### Removed

- remove the `lint` feature. It was planned initially. However, its necessity
  isn't clear at this moment.

## [0.0.5], [b-0.0.5], [c-0.0.5] - 2024-01-05

This release makes `tighterror` self-hosted in `tighterror-build`.

### Added

- implement `PaertialEq` for the `Error` struct
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
