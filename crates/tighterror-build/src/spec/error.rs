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
