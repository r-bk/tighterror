use crate::{errors::TebError, spec::Spec};
use std::fs::File;

#[derive(Debug)]
pub struct TomlParser;

impl TomlParser {
    pub fn from_file(_file: File) -> Result<Spec, TebError> {
        todo!()
    }
}
