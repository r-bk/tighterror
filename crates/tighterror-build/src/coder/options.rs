use crate::errors::TbError;
use std::path::PathBuf;

/// Options for the code generator.
///
/// This struct is a builder for the code generator options.
///
/// # Examples
/// ```no_run
/// # use tighterror_build::{CodegenOptions, errors::TbError};
/// # pub fn foo() -> Result<(), TbError> {
/// CodegenOptions::new()
///     .spec("tighterror.yaml".to_owned())
///     .output("src/errors.rs".to_owned())
///     .codegen()?;
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct CodegenOptions {
    pub(crate) spec: Option<PathBuf>,
    pub(crate) output: Option<PathBuf>,
    pub(crate) test: Option<bool>,
    pub(crate) update: Option<bool>,
    pub(crate) separate_files: Option<bool>,
}

impl CodegenOptions {
    /// Creates a new options object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the specification file path.
    ///
    /// If a value is not specified the default specification filenames
    /// are used in the following order:
    /// * if the `yaml` feature is enabled the path [DEFAULT_SPEC_PATH_YAML]
    ///   is used
    /// * if specification file is still not found and the `toml` feature is
    ///   enabled the path [DEFAULT_SPEC_PATH_TOML] is used
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().spec("tighterror.yaml");
    /// ```
    ///
    /// [DEFAULT_SPEC_PATH_YAML]: crate::DEFAULT_SPEC_PATH_YAML
    /// [DEFAULT_SPEC_PATH_TOML]: crate::DEFAULT_SPEC_PATH_TOML
    pub fn spec(&mut self, spec: impl Into<PathBuf>) -> &mut Self {
        self.spec = Some(spec.into());
        self
    }

    /// Sets the specification file path option.
    ///
    /// This method enhances [`spec`](Self::spec) to set the specification
    /// file path option. This is handy when one needs to
    /// reset the option back to `None` or has an `Option<PathBuf>` parsed
    /// from command line.
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().spec_option(None);
    /// CodegenOptions::new().spec_option(Some("tighterror.yaml".into()));
    /// ```
    pub fn spec_option(&mut self, spec: Option<PathBuf>) -> &mut Self {
        self.spec = spec;
        self
    }

    /// Sets the output path.
    ///
    /// This can be either an absolute path, a relative path, or hyphen `-`.
    /// A relative path is relative to the location of the specification file.
    ///
    /// If the path points to an existing directory the behavior depends on
    /// *separate files* mode. If *separate files* is disabled the output is
    /// written into file `tighterror.rs` under the directory.
    /// See [`separate_files`](Self::separate_files) for the case when the mode
    /// is enabled.
    ///
    /// If the value is a hyphen `-`, or output path is not set at all, the
    /// output is written to `stdout`.
    ///
    /// A `Some` value defined here overrides the `MainObject::output` attribute
    /// in the specification file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().output("src/errors.rs");
    /// ```
    pub fn output(&mut self, output: impl Into<PathBuf>) -> &mut Self {
        self.output = Some(output.into());
        self
    }

    /// Sets the output path option.
    ///
    /// This method enhances [`output`](Self::output) to set the output path
    /// option. This is handy when one needs to reset the option back to `None`
    /// or has an `Option<PathBuf>` parsed from command line.
    ///
    /// # Examples
    /// ```rust
    ///  # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().output_option(None);
    /// CodegenOptions::new().output_option(Some("./src".into()));
    /// ```
    pub fn output_option(&mut self, output: Option<PathBuf>) -> &mut Self {
        self.output = output;
        self
    }

    /// Enables the unit test.
    ///
    /// When enabled a module unit-test is included in the generated code.
    ///
    /// Note that in `no_std` environments test cases that require `std` are
    /// excluded.
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().test(None);
    /// CodegenOptions::new().test(true);
    /// CodegenOptions::new().test(Some(false));
    /// ```
    pub fn test(&mut self, test: impl Into<Option<bool>>) -> &mut Self {
        self.test = test.into();
        self
    }

    /// Enables the *update* mode.
    ///
    /// If the value is `true` and the output file already exists
    /// it is not overwritten if the new content equals the existing one.
    ///
    /// This option is useful to avoid recompilation of a crate, because
    /// `cargo` rebuilds a crate when one of the source file's modification
    /// time changes. When using the *update* mode the modification time
    /// changes only when the file data changes, and recompilation is really
    /// needed. Without *update* mode the output file is overwritten
    /// unconditionally, even when the new data equals the existing one.
    /// This may cause an unnecessary recompilation.
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().update(None);
    /// CodegenOptions::new().update(true);
    /// CodegenOptions::new().update(Some(false));
    /// ```
    pub fn update(&mut self, update: impl Into<Option<bool>>) -> &mut Self {
        self.update = update.into();
        self
    }

    /// Enables the *separate files* mode.
    ///
    /// When enabled every module in specification is written to a separate
    /// file. The [`output`](Self::output) option must point to an
    /// existing directory. The module files, named after the modules
    /// with addition of the `.rs` suffix, are written under the directory.
    ///
    /// For example, assuming `output` equals `./src` and there
    /// are two modules named `errors` and `internal_errors`, the modules will
    /// be written to `./src/errors.rs` and `./src/internal_errors.rs`
    /// respectively.
    ///
    /// Note that if the output is written to `stdout` the *separate files*
    /// mode is implicitly disabled.
    pub fn separate_files(&mut self, separate_files: impl Into<Option<bool>>) -> &mut Self {
        self.separate_files = separate_files.into();
        self
    }

    /// Invokes the code generator [main function] using these options.
    ///
    /// See the struct documentation for a full example.
    ///
    /// [main function]: crate::codegen
    pub fn codegen(&self) -> Result<(), TbError> {
        super::codegen(self)
    }
}
