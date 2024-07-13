use convert_case::{Case::UpperSnake, Casing};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OverridableErrorSpec {
    pub doc_from_display: Option<bool>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ErrorSpec {
    pub name: String,
    pub display: Option<String>,
    pub doc: Option<String>,
    pub oes: OverridableErrorSpec,
}

impl ErrorSpec {
    pub fn ident_name(&self) -> String {
        self.name.to_case(UpperSnake)
    }
}
