use crate::{errors::TebError, linter::LintReport};
use std::fs::File;

#[derive(Debug)]
pub struct TomlLinter;

impl TomlLinter {
    pub fn from_file(_file: File) -> Result<LintReport, TebError> {
        todo!()
    }
}
