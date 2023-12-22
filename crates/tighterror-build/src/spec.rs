mod error;
pub use error::*;

mod category;
pub use category::*;

pub const STDOUT_DST: &str = "-";
pub const DEFAULT_MODULE_DOC: &str = "";
pub const DEFAULT_ERROR_STRUCT_DOC: &str = "Error type.";
pub const DEFAULT_ERROR_CODE_STRUCT_DOC: &str = "Error code type.";
pub const DEFAULT_ERROR_CODE_CONST_DOC: &str = "";
pub const DEFAULT_CATEGORY_STRUCT_DOC: &str = "Error category type.";
pub const DEFAULT_GENERAL_CAT_DOC: &str = "General error category.";
pub const DEFAULT_DOC_FROM_DISPLAY: bool = false;
pub const DEFAULT_TEST: bool = false;
pub const DEFAULT_ERR_INTO_RESULT: bool = true;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MainSpec {
    /// Module documentation
    pub mod_doc: Option<String>,
    /// Error struct's documentation
    pub err_doc: Option<String>,
    /// ErrorCode struct's documentation
    pub err_code_doc: Option<String>,
    /// Category struct's documentation
    pub cat_doc: Option<String>,
    /// destination file path: relative to the specification file, or an
    /// absolute path.
    pub dst: Option<String>,
    /// Add `impl From<Error> for Result`
    pub err_into_result: Option<bool>,
    pub oes: OverrideableErrorSpec,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Spec {
    pub main: MainSpec,
    pub categories: Vec<CategorySpec>,
}

impl Spec {
    pub fn dst<'a>(&'a self, path: Option<&'a str>) -> &'a str {
        path.or(self.main.dst.as_deref()).unwrap_or(STDOUT_DST)
    }

    pub fn test(&self, test: Option<bool>) -> bool {
        test.unwrap_or(DEFAULT_TEST)
    }

    pub fn n_errors_in_largest_category(&self) -> Option<usize> {
        self.categories.iter().map(|c| c.errors.len()).max()
    }

    pub fn err_doc(&self) -> &str {
        self.main
            .err_doc
            .as_deref()
            .unwrap_or(DEFAULT_ERROR_STRUCT_DOC)
    }

    pub fn cat_doc(&self) -> &str {
        self.main
            .cat_doc
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

    pub fn err_code_const_doc<'a>(&self, c: &'a CategorySpec, e: &'a ErrorSpec) -> &'a str {
        if let Some(doc) = e.doc.as_deref() {
            return doc;
        }

        let dfd = e
            .oes
            .doc_from_display
            .or(c.oes.doc_from_display)
            .or(self.main.oes.doc_from_display)
            .unwrap_or(DEFAULT_DOC_FROM_DISPLAY);

        if dfd {
            e.display.as_deref().unwrap_or(DEFAULT_ERROR_CODE_CONST_DOC)
        } else {
            DEFAULT_ERROR_CODE_CONST_DOC
        }
    }

    pub fn error_display(&self, _c: &CategorySpec, e: &ErrorSpec) -> String {
        if let Some(disp) = e.display.as_ref() {
            return disp.clone();
        }
        e.ident_name()
    }

    pub fn err_code_doc(&self) -> &str {
        self.main
            .err_code_doc
            .as_deref()
            .unwrap_or(DEFAULT_ERROR_CODE_STRUCT_DOC)
    }

    pub fn mod_doc(&self) -> &str {
        self.main.mod_doc.as_deref().unwrap_or(DEFAULT_MODULE_DOC)
    }

    pub fn category_max(&self) -> usize {
        debug_assert!(!self.categories.is_empty());
        self.categories.len() - 1
    }

    pub fn err_into_result(&self) -> bool {
        self.main.err_into_result.unwrap_or(DEFAULT_ERR_INTO_RESULT)
    }
}
