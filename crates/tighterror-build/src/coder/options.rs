use crate::errors::TebError;

/// Options for the code generator.
///
/// This struct is a builder for the code generator options.
///
/// # Examples
/// ```no_run
/// # use tighterror_build::{CodegenOptions, errors::TebError};
/// # pub fn foo() -> Result<(), TebError> {
/// CodegenOptions::new()
///     .spec("tighterror.yaml".to_owned())
///     .dst("src/errors.rs".to_owned())
///     .codegen()?;
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct CodegenOptions {
    pub(crate) spec: Option<String>,
    pub(crate) dst: Option<String>,
    pub(crate) test: Option<bool>,
}

impl CodegenOptions {
    /// Creates a new options object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the specification file path.
    ///
    /// If `Some` value is not specified the path [DEFAULT_SPEC_PATH] is used.
    ///
    /// # Examples
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().spec(None);
    /// CodegenOptions::new().spec("tighterror.yaml".to_owned());
    /// CodegenOptions::new().spec(Some("myerrors.toml".into()));
    /// ```
    ///
    /// [DEFAULT_SPEC_PATH]: crate::DEFAULT_SPEC_PATH
    pub fn spec(&mut self, spec: impl Into<Option<String>>) -> &mut Self {
        self.spec = spec.into();
        self
    }

    /// Sets the destination file path.
    ///
    /// If the value is `"-"`, or destination file path is not set at all, the
    /// output is written to `stdout`.
    ///
    /// A `Some` value specified here overrides the one present in the
    /// specification file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tighterror_build::CodegenOptions;
    /// CodegenOptions::new().dst(None);
    /// CodegenOptions::new().dst("src/errors.rs".to_owned());
    /// CodegenOptions::new().dst(Some("myerrors.rs".into()));
    /// ```
    pub fn dst(&mut self, dst: impl Into<Option<String>>) -> &mut Self {
        self.dst = dst.into();
        self
    }

    /// Enables the unit test.
    ///
    /// If the value is `true` a module unit-test is included in the generated
    /// code.
    ///
    /// A `Some` value specified here overrides the one present in the
    /// specification file.
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

    /// Invokes the code generator [main function] using these options.
    ///
    /// See the struct documentation for a full example.
    ///
    /// [main function]: crate::codegen
    pub fn codegen(&self) -> Result<(), TebError> {
        super::codegen(self)
    }
}
