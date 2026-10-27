use crate::{coder::CodegenOptions, errors::TbError};
use std::path::PathBuf;

/// Options for the code generator in a [build script].
///
/// This struct is a builder for the options of [build](Self::build).
///
/// # Examples
/// ```no_run
/// // build.rs
/// use tighterror_build::{BuildOptions, errors::TbError};
///
/// fn main() -> Result<(), TbError> {
///     BuildOptions::new().spec("tighterror.kdl").test(true).build()
/// }
/// ```
///
/// [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub(crate) codegen: CodegenOptions,
}

impl Default for BuildOptions {
    fn default() -> Self {
        let mut codegen = CodegenOptions::new();
        codegen
            .separate_files(true)
            .update(false)
            .build_script(true);
        Self { codegen }
    }
}

impl BuildOptions {
    /// Creates a new options object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the specification file path.
    ///
    /// A relative path is relative to the package root, where cargo runs
    /// the build script.
    ///
    /// If a value is not specified the default specification file path
    /// [DEFAULT_SPEC_PATH_KDL] is used.
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::BuildOptions;
    /// BuildOptions::new().spec("tighterror.kdl");
    /// ```
    ///
    /// [DEFAULT_SPEC_PATH_KDL]: crate::DEFAULT_SPEC_PATH_KDL
    pub fn spec(&mut self, spec: impl Into<PathBuf>) -> &mut Self {
        self.codegen.spec(spec);
        self
    }

    /// Enables the unit test.
    ///
    /// When enabled a module unit-test is included in the generated code.
    ///
    /// In `no_std` environments test cases that require `std` are excluded.
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::BuildOptions;
    /// BuildOptions::new().test(None);
    /// BuildOptions::new().test(true);
    /// BuildOptions::new().test(Some(false));
    /// ```
    pub fn test(&mut self, test: impl Into<Option<bool>>) -> &mut Self {
        self.codegen.test(test);
        self
    }

    /// Generates the code of every module in the specification file.
    ///
    /// The code of a module is written to `$OUT_DIR/<module>.rs`, where
    /// `<module>` is the module name. The crate includes it with
    /// [`tighterror::include_errors`]:
    ///
    /// ```text
    /// /// Crate errors.
    /// pub mod errors {
    ///     tighterror::include_errors!(errors);
    /// }
    /// ```
    ///
    /// The `doc` of a module is left out of its file, because `include!`
    /// doesn't accept inner attributes. Write the module documentation on
    /// the `mod` item instead.
    ///
    /// The `output` setting of the specification file is ignored, and the
    /// code isn't formatted with `rustfmt`.
    ///
    /// The function prints `cargo:rerun-if-changed` for the specification
    /// file, so cargo reruns the build script when the file changes.
    ///
    /// The function must be called from a build script, because it reads
    /// the `OUT_DIR` environment variable that cargo sets for it.
    ///
    /// Errors are described in [log] records. When no logger is set, the
    /// function sets one that writes warnings and errors to `stderr`, which
    /// cargo shows when the build script fails. A build script that sets its
    /// own logger must do so before calling the function, because a process
    /// can set a logger only once.
    pub fn build(&self) -> Result<(), TbError> {
        super::build_script(self)
    }
}
