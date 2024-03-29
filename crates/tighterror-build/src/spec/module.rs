use super::{CategorySpec, ErrorSpec, OverridableErrorSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModuleSpec {
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
    /// Module categories
    pub categories: Vec<CategorySpec>,
}

impl ModuleSpec {
    #[allow(dead_code)]
    pub fn errors_iter(&self) -> ModuleSpecErrorIter {
        ModuleSpecErrorIter {
            categories: self.categories.iter(),
            errors: [].iter(),
        }
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
