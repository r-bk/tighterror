use crate::{errors::TebError, linter::LintReport, parser::YamlParser};
use std::fs::File;

#[derive(Debug)]
pub struct YamlLinter;

impl YamlLinter {
    pub fn from_file(file: File) -> Result<LintReport, TebError> {
        let _config = YamlParser::from_file(file)?;
        // trivial implementation for now
        // in future the linter won't stop on first error
        // and will return all the errors in the LintReport
        // instead of tracing them
        Ok(LintReport::default())
    }
}
