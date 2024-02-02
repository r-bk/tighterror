use crate::coder::idents;

mod error;
pub use error::*;

mod category;
pub use category::*;

pub const STDOUT_DST: &str = "-";
pub const DEFAULT_MODULE_DOC: &str = "";
pub const DEFAULT_ERROR_STRUCT_DOC: &str =
    "Error type.\n\nSee the [kinds] module for error kind constants.";
pub const DEFAULT_ERROR_KIND_STRUCT_DOC: &str =
    "Error kind type.\n\nSee the [kinds] module for error kind constants.";
pub const DEFAULT_ERROR_KIND_CONST_DOC: &str = "";
pub const DEFAULT_CATEGORY_STRUCT_DOC: &str =
    "Error category type.\n\nSee the [categories] module for category constants.";
pub const DEFAULT_GENERAL_CAT_DOC: &str = "General error category.";
pub const DEFAULT_DOC_FROM_DISPLAY: bool = false;
pub const DEFAULT_TEST: bool = false;
pub const DEFAULT_ERR_INTO_RESULT: bool = true;
pub const DEFAULT_ERR_KIND_INTO_RESULT: bool = true;
pub const DEFAULT_ERROR_TRAIT: bool = true;
pub const DEFAULT_UPDATE_MODE: bool = false;
pub const DEFAULT_NO_STD: bool = false;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModuleSpec {
    /// Module documentation
    pub mod_doc: Option<String>,
    /// Error struct's documentation
    pub err_doc: Option<String>,
    /// ErrorKind struct's documentation
    pub err_kind_doc: Option<String>,
    /// A doc string for the ErrorCategory struct
    pub err_cat_doc: Option<String>,
    /// output file path: relative to the specification file, or an
    /// absolute path.
    pub output: Option<String>,
    /// Add `impl From<Error> for Result`
    pub result_from_err: Option<bool>,
    /// Add `impl From<ErrorKind> for Result<T, Error>`
    pub result_from_err_kind: Option<bool>,
    /// Add `impl std::error::Error for Error`
    pub error_trait: Option<bool>,
    /// A custom name for the Error struct
    pub err_name: Option<String>,
    /// A custom name for the ErrorKind struct
    pub err_kind_name: Option<String>,
    /// A custom name for the ErrorCategory struct
    pub err_cat_name: Option<String>,
    pub oes: OverridableErrorSpec,
    /// Generate code for `no_std` environment
    pub no_std: Option<bool>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Spec {
    pub module: ModuleSpec,
    pub categories: Vec<CategorySpec>,
}

impl Spec {
    pub fn output<'a>(&'a self, path: Option<&'a str>) -> &'a str {
        path.or(self.module.output.as_deref()).unwrap_or(STDOUT_DST)
    }

    pub fn test(&self, test: Option<bool>) -> bool {
        test.unwrap_or(DEFAULT_TEST)
    }

    pub fn n_errors_in_largest_category(&self) -> Option<usize> {
        self.categories.iter().map(|c| c.errors.len()).max()
    }

    pub fn err_doc(&self) -> &str {
        self.module
            .err_doc
            .as_deref()
            .unwrap_or(DEFAULT_ERROR_STRUCT_DOC)
    }

    pub fn err_cat_doc(&self) -> &str {
        self.module
            .err_cat_doc
            .as_deref()
            .unwrap_or(DEFAULT_CATEGORY_STRUCT_DOC)
    }

    pub fn cat_const_doc<'a>(&'a self, c: &'a CategorySpec) -> &'a str {
        if self.categories.len() == 1 {
            c.doc.as_deref().unwrap_or(DEFAULT_GENERAL_CAT_DOC)
        } else {
            c.doc.as_deref().unwrap_or_default()
        }
    }

    pub fn err_kind_const_doc<'a>(&self, c: &'a CategorySpec, e: &'a ErrorSpec) -> &'a str {
        if let Some(doc) = e.doc.as_deref() {
            return doc;
        }

        let dfd = e
            .oes
            .doc_from_display
            .or(c.oes.doc_from_display)
            .or(self.module.oes.doc_from_display)
            .unwrap_or(DEFAULT_DOC_FROM_DISPLAY);

        if dfd {
            e.display.as_deref().unwrap_or(DEFAULT_ERROR_KIND_CONST_DOC)
        } else {
            DEFAULT_ERROR_KIND_CONST_DOC
        }
    }

    pub fn error_display(&self, _c: &CategorySpec, e: &ErrorSpec) -> String {
        if let Some(disp) = e.display.as_ref() {
            return disp.clone();
        }
        e.ident_name()
    }

    pub fn err_kind_doc(&self) -> &str {
        self.module
            .err_kind_doc
            .as_deref()
            .unwrap_or(DEFAULT_ERROR_KIND_STRUCT_DOC)
    }

    pub fn mod_doc(&self) -> &str {
        self.module.mod_doc.as_deref().unwrap_or(DEFAULT_MODULE_DOC)
    }

    pub fn category_max(&self) -> usize {
        debug_assert!(!self.categories.is_empty());
        self.categories.len() - 1
    }

    pub fn result_from_err(&self) -> bool {
        self.module
            .result_from_err
            .unwrap_or(DEFAULT_ERR_INTO_RESULT)
    }

    pub fn result_from_err_kind(&self) -> bool {
        self.module
            .result_from_err_kind
            .unwrap_or(DEFAULT_ERR_KIND_INTO_RESULT)
    }

    pub fn error_trait(&self) -> bool {
        self.module
            .no_std
            .map(|v| !v)
            .or(self.module.error_trait)
            .unwrap_or(DEFAULT_ERROR_TRAIT)
    }

    pub fn err_name(&self) -> &str {
        self.module.err_name.as_deref().unwrap_or(idents::ERROR)
    }

    pub fn err_kind_name(&self) -> &str {
        self.module
            .err_kind_name
            .as_deref()
            .unwrap_or(idents::ERROR_KIND)
    }

    pub fn err_cat_name(&self) -> &str {
        self.module
            .err_cat_name
            .as_deref()
            .unwrap_or(idents::ERROR_CATEGORY)
    }

    pub fn no_std(&self) -> bool {
        self.module.no_std.unwrap_or(DEFAULT_NO_STD)
    }
}
