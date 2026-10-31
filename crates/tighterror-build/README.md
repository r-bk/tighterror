# _tighterror-build_

**tighterror-build** generates the Rust code of the _tighterror_ framework
from a specification file written in KDL.

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/tighterror-build.svg
[crates-url]: https://crates.io/crates/tighterror-build
[docs-badge]: https://img.shields.io/docsrs/tighterror-build
[docs-url]: https://docs.rs/tighterror-build/latest/tighterror-build

The recommended way to use it is a build script, as a build-dependency of
the project. The cargo plugin [cargo-tighterror], a thin wrapper around the
library, writes the code into the source tree instead.

[cargo-tighterror]: https://crates.io/crates/cargo-tighterror

## Documentation

The documentation, including the reference of the specification file, is on
[docs.rs].

The overall framework is documented in the [tighterror] crate.

[docs.rs]: https://docs.rs/tighterror-build/latest/tighterror_build
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
