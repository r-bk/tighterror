use crate::common::casing;
use convert_case::Case;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OverridableErrorSpec {
    pub doc_from_display: Option<bool>,
    pub variant_type: Option<bool>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ErrorSpec {
    pub name: String,
    pub display: Option<String>,
    pub doc: Option<String>,
    pub variant_type_name: Option<String>,
    pub oes: OverridableErrorSpec,
}

impl ErrorSpec {
    pub fn variant_type_name(&self) -> String {
        if let Some(ref vtn) = self.variant_type_name {
            vtn.clone()
        } else {
            casing::convert_case(&self.name, Case::UpperSnake, Case::UpperCamel)
        }
    }
}
