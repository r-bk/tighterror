use crate::{
    common::casing,
    spec::{ErrorSpec, OverridableErrorSpec},
};
use convert_case::Case::{Snake, UpperCamel, UpperSnake};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CategorySpec {
    pub name: String,
    pub oes: OverridableErrorSpec,
    /// Category constant's documentation
    pub doc: Option<String>,
    pub errors: Vec<ErrorSpec>,
}

impl CategorySpec {
    pub fn ident_name(&self) -> String {
        casing::convert_case(&self.name, UpperCamel, UpperSnake)
    }

    pub fn module_name(&self) -> String {
        casing::convert_case(&self.name, UpperCamel, Snake)
    }
}
