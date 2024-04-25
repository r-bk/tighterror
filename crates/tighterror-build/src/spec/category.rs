use crate::spec::{ErrorSpec, OverridableErrorSpec};
use convert_case::{
    Case::{Snake, UpperSnake},
    Casing,
};

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
        self.name.to_case(UpperSnake)
    }

    pub fn kinds_module_name(&self) -> String {
        self.name.to_case(Snake)
    }
}
