# *cargo-tighterror*

The Cargo plugin of the *tighterror* framework.

[![crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/cargo-tighterror.svg
[crates-url]: https://crates.io/crates/cargo-tighterror

**cargo-tighterror** is a command-line utility that deals with *tighterror*
specification files and Rust code generation. It is a thin wrapper around the
[tighterror-build] library.

[tighterror-build]: https://crates.io/crates/tighterror-build

## Installation

```shell
cargo install cargo-tighterror
```

## Synopsis

```text
$> cargo help tighterror
The cargo plugin of the tighterror framework.

Usage: cargo tighterror [OPTIONS]

Options:
  -s, --spec <PATH>    The specification file path
  -o, --output <PATH>  The output file path
  -t, --test           Include a unit-test in the generated code
  -u, --update         Do not overwrite the output file if data is unchanged
  -h, --help           Print help
  -V, --version        Print version
```

## Documentation

See the documentation in the [tighterror] crate.

[tighterror]: https://docs.rs/tighterror/latest/tighterror

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
