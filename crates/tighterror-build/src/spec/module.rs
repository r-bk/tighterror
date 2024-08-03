use super::{definitions::*, idents, CategorySpec, ErrorSpec, OverridableErrorSpec};

pub const IMPLICIT_MODULE_NAME: &str = "errors";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModuleSpec {
    /// The name of the module
    pub name: Option<String>,
    /// Module documentation
    pub doc: Option<String>,
    /// Error struct's documentation
    pub err_doc: Option<String>,
    /// ErrorKind struct's documentation
    pub err_kind_doc: Option<String>,
    /// A doc string for the ErrorCategory struct
    pub err_cat_doc: Option<String>,
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
    /// Place the error kind consts under `mod kinds`
    /// and not under `mod kinds::<category_mod>`.
    /// Requires all error names to be unique per module.
    pub flat_kinds: Option<bool>,
    /// Module categories
    pub categories: Vec<CategorySpec>,
}

impl ModuleSpec {
    pub fn implicit_with_categories(categories: Vec<CategorySpec>) -> Self {
        Self {
            categories,
            ..Default::default()
        }
    }

    pub fn errors_iter(&self) -> ModuleSpecErrorIter {
        ModuleSpecErrorIter {
            categories: self.categories.iter(),
            errors: [].iter(),
        }
    }

    pub fn n_errors_in_largest_category(&self) -> Option<usize> {
        self.categories.iter().map(|c| c.errors.len()).max()
    }

    pub fn err_doc(&self) -> &str {
        self.err_doc.as_deref().unwrap_or(DEFAULT_ERROR_STRUCT_DOC)
    }

    pub fn err_cat_doc(&self) -> &str {
        self.err_cat_doc
            .as_deref()
            .unwrap_or(DEFAULT_CATEGORY_STRUCT_DOC)
    }

    pub fn cat_const_doc<'a>(&'a self, c: &'a CategorySpec) -> &'a str {
        let implicit_value = if self.categories.len() == 1 {
            DEFAULT_GENERAL_CAT_DOC
        } else {
            ""
        };
        c.doc.as_deref().unwrap_or(implicit_value)
    }

    pub fn err_kind_const_doc<'a>(&self, c: &'a CategorySpec, e: &'a ErrorSpec) -> &'a str {
        if let Some(doc) = e.doc.as_deref() {
            return doc;
        }

        let dfd = e
            .oes
            .doc_from_display
            .or(c.oes.doc_from_display)
            .or(self.oes.doc_from_display)
            .unwrap_or(DEFAULT_DOC_FROM_DISPLAY);

        if dfd {
            e.display.as_deref().unwrap_or(DEFAULT_ERROR_KIND_CONST_DOC)
        } else {
            DEFAULT_ERROR_KIND_CONST_DOC
        }
    }

    pub fn err_kind_doc(&self) -> &str {
        self.err_kind_doc
            .as_deref()
            .unwrap_or(DEFAULT_ERROR_KIND_STRUCT_DOC)
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or(IMPLICIT_MODULE_NAME)
    }

    pub fn doc(&self) -> &str {
        self.doc.as_deref().unwrap_or(DEFAULT_MODULE_DOC)
    }

    pub fn category_max(&self) -> usize {
        debug_assert!(!self.categories.is_empty());
        self.categories.len() - 1
    }

    pub fn result_from_err(&self) -> bool {
        self.result_from_err.unwrap_or(DEFAULT_ERR_INTO_RESULT)
    }

    pub fn result_from_err_kind(&self) -> bool {
        self.result_from_err_kind
            .unwrap_or(DEFAULT_ERR_KIND_INTO_RESULT)
    }

    pub fn error_trait(&self, no_std: Option<bool>) -> bool {
        no_std
            .map(|v| !v)
            .or(self.error_trait)
            .unwrap_or(DEFAULT_ERROR_TRAIT)
    }

    pub fn err_name(&self) -> &str {
        self.err_name.as_deref().unwrap_or(idents::ERROR)
    }

    pub fn err_kind_name(&self) -> &str {
        self.err_kind_name.as_deref().unwrap_or(idents::ERROR_KIND)
    }

    pub fn err_cat_name(&self) -> &str {
        self.err_cat_name
            .as_deref()
            .unwrap_or(idents::ERROR_CATEGORY)
    }

    pub fn flat_kinds(&self) -> bool {
        self.flat_kinds.unwrap_or(DEFAULT_FLAT_KINDS)
    }
}

pub struct ModuleSpecErrorIter<'a> {
    categories: std::slice::Iter<'a, CategorySpec>,
    errors: std::slice::Iter<'a, ErrorSpec>,
}

impl<'a> Iterator for ModuleSpecErrorIter<'a> {
    type Item = &'a ErrorSpec;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.errors.next() {
            return Some(e);
        }
        loop {
            if let Some(c) = self.categories.next() {
                if c.errors.is_empty() {
                    continue;
                }
                self.errors = c.errors.iter();
                break self.errors.next();
            } else {
                break None;
            }
        }
    }
}
