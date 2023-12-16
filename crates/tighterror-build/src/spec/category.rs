use crate::spec::{ErrorSpec, OverrideableErrorSpec};
use convert_case::{Case::UpperSnake, Casing};

pub const IMPLICIT_CATEGORY_NAME: &str = "General";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CategorySpec {
    pub name: String,
    pub oes: OverrideableErrorSpec,
    /// Category constant's documentation
    pub doc: Option<String>,
    pub errors: Vec<ErrorSpec>,
}

impl CategorySpec {
    pub fn ident_name(&self) -> String {
        self.name.to_case(UpperSnake)
    }
}
