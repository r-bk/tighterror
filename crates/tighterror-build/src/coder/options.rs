use crate::errors::TbError;

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
    pub(crate) spec: Option<String>,
    pub(crate) output: Option<String>,
    pub(crate) test: Option<bool>,
    pub(crate) update: Option<bool>,
}

impl CodegenOptions {
    /// Creates a new options object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the specification file path.
    ///
    /// If `Some` value is not specified the default specification filenames
    /// are used in the following order:
    /// * if the `yaml` feature is enabled the path [DEFAULT_SPEC_PATH_YAML]
    ///   is used
    /// * if specification file is still not found and the `toml` feature is
    ///   enabled the path [DEFAULT_SPEC_PATH_TOML] is used
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().spec(None);
    /// CodegenOptions::new().spec("tighterror.yaml".to_owned());
    /// CodegenOptions::new().spec(Some("myerrors.toml".into()));
    /// ```
    ///
    /// [DEFAULT_SPEC_PATH_YAML]: crate::DEFAULT_SPEC_PATH_YAML
    /// [DEFAULT_SPEC_PATH_TOML]: crate::DEFAULT_SPEC_PATH_TOML
    pub fn spec(&mut self, spec: impl Into<Option<String>>) -> &mut Self {
        self.spec = spec.into();
        self
    }

    /// Sets the output file path.
    ///
    /// If the value is `"-"`, or output file path is not set at all, the
    /// output is written to `stdout`.
    ///
    /// A `Some` value defined here overrides the `output` value in the
    /// specification file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().output(None);
    /// CodegenOptions::new().output("src/errors.rs".to_owned());
    /// CodegenOptions::new().output(Some("myerrors.rs".into()));
    /// ```
    pub fn output(&mut self, output: impl Into<Option<String>>) -> &mut Self {
        self.output = output.into();
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

    /// Invokes the code generator [main function] using these options.
    ///
    /// See the struct documentation for a full example.
    ///
    /// [main function]: crate::codegen
    pub fn codegen(&self) -> Result<(), TbError> {
        super::codegen(self)
    }
}
