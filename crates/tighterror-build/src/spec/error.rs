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

#[allow(dead_code)]
impl OverridableErrorSpec {
    pub const fn default_spec() -> Self {
        Self {
            doc_from_display: Some(false),
        }
    }

    /// Calculates the effective OES.
    ///
    /// `oes` should be a less specific OES than `self`.
    /// I.e. error -> category -> global.
    pub fn or(&self, oes: &OverridableErrorSpec) -> Self {
        Self {
            doc_from_display: self.doc_from_display.or(oes.doc_from_display),
        }
    }
}
