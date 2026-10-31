# _cargo-tighterror_

The Cargo plugin of the _tighterror_ framework.

[![crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/cargo-tighterror.svg
[crates-url]: https://crates.io/crates/cargo-tighterror

**cargo-tighterror** writes the Rust code of a _tighterror_ specification file
into the source tree, so that both the specification and the generated code
are under source control, and the code can be reviewed. It is a thin wrapper
around the [tighterror-build] library.

Generating the code in a build script with [tighterror-build] is the
recommended workflow. It keeps the specification as the only source, so the
generated code never goes stale. With the plugin, the code has to be
generated again whenever the specification changes, and whenever the plugin
is upgraded, which it needs to be along with the version of _tighterror_ that
the crate depends on.

[tighterror-build]: https://docs.rs/tighterror-build/latest/tighterror_build

## Installation

Install the plugin, or upgrade it to the latest version:

```shell
cargo install cargo-tighterror
```

## Synopsis

```text
$> cargo help tighterror
The cargo plugin of the tighterror framework.

Usage: cargo tighterror [OPTIONS]

Options:
  -s, --spec <PATH>     The specification file path
  -o, --output <PATH>   The output path
  -t, --test            Include a unit-test in the generated code
  -u, --update          Do not overwrite the output file if data is unchanged
  -S, --separate-files  Write modules in separate files
  -h, --help            Print help
  -V, --version         Print version
```

For example, this generates a module in `src/errors.rs` from `tighterror.kdl`
in the current directory:

```shell
cargo tighterror -o src/errors.rs
```

The module is then included as any other module, i.e., `mod errors;`.

## Options

### `-s, --spec <PATH>`

The specification file path. It can be omitted when the file has the default
name `tighterror.kdl` and is in the current working directory. A custom name
must have the `.kdl` extension.

### `-o, --output <PATH>`

The output path: an absolute path, a path relative to the working directory,
or a hyphen `-` for `stdout`. When it is an existing directory, the code goes
to `tighterror.rs` under it.

It overrides the [`output`][root-settings] setting of the specification file,
whose relative path is relative to the specification file. When neither is
given, the output is written to `stdout`.

### `-t, --test`

Includes unit tests in the generated code. In `no_std` modules the tests that
need `std` are left out.

### `-u, --update`

Overwrites the output file only when its content changes.

By default the output file is written unconditionally, which updates its
timestamp and may cause an unnecessary rebuild of the crate. In update mode
the file is written only when the new code differs from the existing one.

### `-S, --separate-files`

Writes every module of the specification in a file of its own, named after
the module with the `.rs` extension, under the output directory. The
directory, given by `-o, --output` or by the `output` setting, must exist.

By default the output, even of several modules, is a single file in which
every module is enclosed in a module block, e.g., `pub mod my_errors { ... }`.
When a module is written in a file of its own, or when the specification has
a single module, the file holds the items of the module without such a block,
so that it is included as the module itself, e.g., `mod errors;`.

When the output goes to `stdout`, the modules are written in a single file.

[root-settings]: https://docs.rs/tighterror-build/latest/tighterror_build/#root-settings

## Documentation

The specification file is described in the
[specification reference][spec-reference] of _tighterror-build_, and the
framework in the [tighterror] crate.

[spec-reference]: https://docs.rs/tighterror-build/latest/tighterror_build/#specification-file-reference
[tighterror]: https://docs.rs/tighterror/latest/tighterror

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
