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
    /// The error of another crate that the category wraps
    pub wrapped: Option<WrappedSpec>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WrappedSpec {
    /// The path of the wrapped Error type
    pub error_type: String,
    /// The path of the wrapped ErrorKind type
    pub kind_type: String,
}

impl CategorySpec {
    pub fn ident_name(&self) -> String {
        casing::convert_case(&self.name, UpperCamel, UpperSnake)
    }

    pub fn module_name(&self) -> String {
        casing::convert_case(&self.name, UpperCamel, Snake)
    }
}
