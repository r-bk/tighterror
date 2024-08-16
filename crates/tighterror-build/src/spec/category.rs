use crate::{
    common::casing,
    spec::{ErrorSpec, OverridableErrorSpec},
};
use convert_case::Case::{Snake, UpperCamel, UpperSnake};

pub const IMPLICIT_CATEGORY_NAME: &str = "General";

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

    pub fn implicit_with_errors(errors: Vec<ErrorSpec>) -> Self {
        Self {
            name: IMPLICIT_CATEGORY_NAME.into(),
            errors,
            ..Default::default()
        }
    }
}
