use crate::{
    errors::{
        kinds::general::{BAD_SPEC, BAD_YAML},
        TbError,
    },
    parser::{
        check_category_name_uniqueness, check_error_name_uniqueness,
        check_module_error_name_uniqueness, check_module_ident, check_name, kws, ParseMode,
    },
    spec::{
        definitions::DEFAULT_FLAT_KINDS, CategorySpec, ErrorSpec, MainSpec, ModuleSpec, Spec,
        IMPLICIT_CATEGORY_NAME,
    },
};
use log::error;
use serde_yaml::{Mapping, Sequence, Value};
use std::fs::File;

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct YamlParser;

impl YamlParser {
    pub fn parse_file(file: File) -> Result<Spec, TbError> {
        match serde_yaml::from_reader(file) {
            Ok(v) => Self::value(v),
            Err(e) => {
                log::error!("failed to deserialize YAML: {e}");
                BAD_YAML.into()
            }
        }
    }

    #[cfg(test)]
    pub fn parse_str(s: &str) -> Result<Spec, TbError> {
        match serde_yaml::from_str(s) {
            Ok(v) => Self::value(v),
            Err(e) => {
                log::error!("failed to deserialize YAML: {e}");
                BAD_YAML.into()
            }
        }
    }

    fn value(value: Value) -> Result<Spec, TbError> {
        match value {
            Value::Mapping(m) => Self::mapping(m),
            v => {
                error!(
                    "specification YAML document must be a Mapping: deserialized a {}",
                    value_type_name(&v)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn mapping(mut m: Mapping) -> Result<Spec, TbError> {
        Self::check_toplevel_attributes(&m)?;

        let mut spec = Spec::default();

        if let Some(v) = m.remove(kws::MAIN) {
            spec.main = MainParser::value(v)?;
        }

        if let Some(v) = m.remove(kws::MODULE) {
            spec.module = ModuleParser::value(v)?;
        }

        if let Some(v) = m.remove(kws::CATEGORY) {
            let parser = CategoryParser(ParseMode::Single);
            let cat_spec = parser.value(v)?;
            spec.module.categories.push(cat_spec);
        }

        if let Some(v) = m.remove(kws::CATEGORIES) {
            spec.module.categories = CategoriesParser::value(v)?;
        }

        if let Some(v) = m.remove(kws::ERRORS) {
            let errors = ErrorsParser::value(v)?;
            if let Some(cat) = spec.module.categories.first_mut() {
                cat.errors = errors;
            } else {
                spec.module.categories.push(CategorySpec {
                    name: IMPLICIT_CATEGORY_NAME.into(),
                    errors,
                    ..Default::default()
                });
            }
        }

        if spec.module.flat_kinds.unwrap_or(DEFAULT_FLAT_KINDS) {
            check_module_error_name_uniqueness(spec.module.errors_iter().map(|e| e.name.as_str()))?;
        }

        Ok(spec)
    }

    fn check_toplevel_attributes(m: &Mapping) -> Result<(), TbError> {
        for k in m.keys() {
            match k {
                Value::String(s) => {
                    if !kws::is_root_kw(s) {
                        error!("invalid top-level keyword: {}", s);
                        return BAD_SPEC.into();
                    }
                }
                ov => {
                    error!("a Mapping key must be a String: deserialized {:?}", ov);
                    return BAD_SPEC.into();
                }
            }
        }

        for (k1, k2) in [
            (kws::ERRORS, kws::CATEGORIES),
            (kws::CATEGORY, kws::CATEGORIES),
        ] {
            if m.contains_key(k1) && m.contains_key(k2) {
                error!("top-level attributes '{k1}' and '{k2}' are mutually exclusive");
                return BAD_SPEC.into();
            }
        }

        for (k1, k2) in [(kws::ERRORS, kws::CATEGORIES)] {
            if !(m.contains_key(k1) || m.contains_key(k2)) {
                error!("one of '{k1}' or '{k2}' must be specified");
                return BAD_SPEC.into();
            }
        }

        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct MainParser;

impl MainParser {
    fn value(v: Value) -> Result<MainSpec, TbError> {
        match v {
            Value::Mapping(m) => Self::mapping(m),
            ref ov => {
                error!(
                    "MainObject must be a Mapping: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn mapping(m: Mapping) -> Result<MainSpec, TbError> {
        let mut main_spec = MainSpec::default();

        for (k, v) in m.into_iter() {
            let key = v2key(k)?;

            if !kws::is_main_kw(&key) {
                error!("invalid MainObject attribute: {}", key);
                return BAD_SPEC.into();
            }

            match key.as_str() {
                kws::OUTPUT => main_spec.output = Some(v2string(v, kws::OUTPUT)?),
                kws::NO_STD => main_spec.no_std = Some(v2bool(v, kws::NO_STD)?),
                _ => panic!("internal error: unhandled MainObject attribute: {}", key),
            }
        }

        Ok(main_spec)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct ModuleParser;

impl ModuleParser {
    fn value(v: Value) -> Result<ModuleSpec, TbError> {
        match v {
            Value::Mapping(m) => Self::mapping(m),
            ref ov => {
                error!(
                    "ModuleObject must be a Mapping: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn mapping(m: Mapping) -> Result<ModuleSpec, TbError> {
        let mut mod_spec = ModuleSpec::default();

        for (k, v) in m.into_iter() {
            let key = v2key(k)?;

            if !kws::is_mod_kw(&key) {
                error!("invalid ModuleObject attribute: {}", key);
                return BAD_SPEC.into();
            }

            match key.as_str() {
                kws::DOC_FROM_DISPLAY => {
                    mod_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?)
                }
                kws::ERR_CAT_DOC => mod_spec.err_cat_doc = Some(v2string(v, kws::ERR_CAT_DOC)?),
                kws::ERR_KIND_DOC => mod_spec.err_kind_doc = Some(v2string(v, kws::ERR_KIND_DOC)?),
                kws::ERR_DOC => mod_spec.err_doc = Some(v2string(v, kws::ERR_DOC)?),
                kws::DOC => mod_spec.doc = Some(v2string(v, kws::DOC)?),
                kws::RESULT_FROM_ERR => {
                    mod_spec.result_from_err = Some(v2bool(v, kws::RESULT_FROM_ERR)?)
                }
                kws::RESULT_FROM_ERR_KIND => {
                    mod_spec.result_from_err_kind = Some(v2bool(v, kws::RESULT_FROM_ERR_KIND)?)
                }
                kws::ERROR_TRAIT => mod_spec.error_trait = Some(v2bool(v, kws::ERROR_TRAIT)?),
                kws::ERR_NAME => {
                    let err_name = v2string(v, kws::ERR_NAME)?;
                    check_module_ident(&err_name, kws::ERR_NAME)?;
                    mod_spec.err_name = Some(err_name);
                }
                kws::ERR_KIND_NAME => {
                    let err_kind_name = v2string(v, kws::ERR_KIND_NAME)?;
                    check_module_ident(&err_kind_name, kws::ERR_KIND_NAME)?;
                    mod_spec.err_kind_name = Some(err_kind_name);
                }
                kws::ERR_CAT_NAME => {
                    let err_cat_name = v2string(v, kws::ERR_CAT_NAME)?;
                    check_module_ident(&err_cat_name, kws::ERR_CAT_NAME)?;
                    mod_spec.err_cat_name = Some(err_cat_name);
                }
                kws::FLAT_KINDS => mod_spec.flat_kinds = Some(v2bool(v, kws::FLAT_KINDS)?),
                _ => panic!("internal error: unhandled ModuleObject attribute: {}", key),
            }
        }

        Ok(mod_spec)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct ErrorsParser;

impl ErrorsParser {
    fn value(v: Value) -> Result<Vec<ErrorSpec>, TbError> {
        match v {
            Value::Sequence(s) => Self::sequence(s),
            ref ov => {
                error!("ErrorsList must be a Sequence: deserialized {:?}", ov);
                BAD_SPEC.into()
            }
        }
    }

    fn sequence(s: Sequence) -> Result<Vec<ErrorSpec>, TbError> {
        let mut errors = Vec::new();
        for v in s.into_iter() {
            match v {
                Value::String(s) => errors.push(ErrorParser::string(s)?),
                Value::Mapping(m) => errors.push(ErrorParser::mapping(m)?),
                ov => {
                    error!(
                        "ErrorObject in ErrorsList must be a String or a Mapping: deserialized {:?}",
                        ov
                    );
                    return BAD_SPEC.into();
                }
            }
        }
        check_error_name_uniqueness(errors.iter().map(|e| e.name.as_str()))?;
        Ok(errors)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct ErrorParser;

impl ErrorParser {
    fn string(s: String) -> Result<ErrorSpec, TbError> {
        check_name(&s)?;
        Ok(ErrorSpec {
            name: s,
            ..Default::default()
        })
    }

    fn is_short_mapping(m: &Mapping) -> bool {
        if m.len() != 1 {
            return false;
        }
        if let Some(key) = m.keys().next() {
            if let Some(k) = key.as_str() {
                return !kws::is_any_kw(k);
            }
        }
        false
    }

    fn mapping(m: Mapping) -> Result<ErrorSpec, TbError> {
        match m.len() {
            0 => {
                error!(
                    "ErrorObject must be a non-empty Mapping: deserialized {:?}",
                    m
                );
                BAD_SPEC.into()
            }
            1 if Self::is_short_mapping(&m) => Self::short_mapping(m),
            _ => Self::long_mapping(m),
        }
    }

    fn short_mapping(m: Mapping) -> Result<ErrorSpec, TbError> {
        assert_eq!(m.len(), 1);
        let (k, v) = m.into_iter().next().unwrap();

        let name = match k {
            Value::String(s) => s,
            ov => {
                error!(
                    "name in name-display notation must be a String: deserialized {:?}",
                    ov
                );
                return BAD_SPEC.into();
            }
        };

        check_name(&name)?;

        let display = match v {
            Value::String(s) => s,
            ov => {
                error!(
                    "display in name-display notation must be a String: deserialized {:?}",
                    ov
                );
                return BAD_SPEC.into();
            }
        };

        Ok(ErrorSpec {
            name,
            display: Some(display),
            ..Default::default()
        })
    }

    fn long_mapping(m: Mapping) -> Result<ErrorSpec, TbError> {
        let mut err_spec = ErrorSpec::default();

        for (k, v) in m.into_iter() {
            let key = v2key(k)?;

            if !kws::is_err_kw(&key) {
                error!("invalid ErrorObject attribute: {}", key);
                return BAD_SPEC.into();
            }

            match key.as_str() {
                kws::NAME => err_spec.name = v2string(v, kws::NAME)?,
                kws::DISPLAY => err_spec.display = Some(v2string(v, kws::DISPLAY)?),
                kws::DOC => err_spec.doc = Some(v2string(v, kws::DOC)?),
                kws::DOC_FROM_DISPLAY => {
                    err_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?)
                }
                _ => panic!("internal error: unhandled ErrorObject attribute: {}", key),
            }
        }

        check_name(&err_spec.name)?;

        Ok(err_spec)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct CategoryParser(ParseMode);

impl CategoryParser {
    fn value(&self, v: Value) -> Result<CategorySpec, TbError> {
        match v {
            Value::Mapping(m) => self.mapping(m),
            ref ov => {
                error!(
                    "CategoryObject must be a Mapping: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn mapping(&self, mut m: Mapping) -> Result<CategorySpec, TbError> {
        for k in m.keys() {
            match k {
                Value::String(s) => {
                    if !kws::is_cat_kw(s) {
                        error!("invalid CategoryObject attribute: {}", s);
                        return BAD_SPEC.into();
                    }
                }
                ov => {
                    error!("a Mapping key must be a String: deserialized {:?}", ov);
                    return BAD_SPEC.into();
                }
            }
        }

        let mut cat_spec = CategorySpec::default();

        if let Some(v) = m.remove(kws::NAME) {
            let name = v2string(v, kws::NAME)?;
            check_name(&name)?;
            cat_spec.name = name;
        }

        if let Some(v) = m.remove(kws::DOC) {
            cat_spec.doc = Some(v2string(v, kws::DOC)?);
        }

        if let Some(v) = m.remove(kws::DOC_FROM_DISPLAY) {
            cat_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?);
        }

        if let Some(v) = m.remove(kws::ERRORS) {
            if matches!(self.0, ParseMode::Single) {
                error!(
                    "ErrorsList is not allowed in top-level '{}' attribute",
                    kws::CATEGORY
                );
                return BAD_SPEC.into();
            }
            cat_spec.errors = ErrorsParser::value(v)?;
        }

        match self.0 {
            ParseMode::Single => {
                if cat_spec.name.is_empty() {
                    IMPLICIT_CATEGORY_NAME.clone_into(&mut cat_spec.name);
                }
            }
            ParseMode::List => {
                if cat_spec.name.is_empty() {
                    error!("CategoryObject name is mandatory in CategoriesList");
                    return BAD_SPEC.into();
                }
                if cat_spec.errors.is_empty() {
                    error!("ErrorsList not found: category_name = {}", cat_spec.name);
                    return BAD_SPEC.into();
                }
            }
        }

        Ok(cat_spec)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct CategoriesParser;

impl CategoriesParser {
    fn value(v: Value) -> Result<Vec<CategorySpec>, TbError> {
        match v {
            Value::Sequence(s) => Self::sequence(s),
            ref ov => {
                error!("CategoriesList must be a Sequence: deserialized {:?}", ov);
                BAD_SPEC.into()
            }
        }
    }

    fn sequence(s: Sequence) -> Result<Vec<CategorySpec>, TbError> {
        let mut categories = Vec::new();
        for v in s.into_iter() {
            match v {
                Value::Mapping(m) => {
                    let parser = CategoryParser(ParseMode::List);
                    let cat_spec = parser.mapping(m)?;
                    categories.push(cat_spec);
                }
                ov => {
                    error!(
                        "CategoryObject in CategoriesList must be a Mapping: deserialized {:?}",
                        ov
                    );
                    return BAD_SPEC.into();
                }
            }
        }
        check_category_name_uniqueness(categories.iter().map(|c| c.name.as_str()))?;
        Ok(categories)
    }
}

// ----------------------------------------------------------------------------

fn value_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "Null",
        Value::Bool(_) => "Bool",
        Value::Mapping(_) => "Mapping",
        Value::Number(_) => "Number",
        Value::Sequence(_) => "Sequence",
        Value::String(_) => "String",
        Value::Tagged(_) => "Tagged",
    }
}

fn v2key(v: Value) -> Result<String, TbError> {
    let key = match v {
        Value::String(s) => s,
        ov => {
            error!("a Mapping key must be a String: deserialized {:?}", ov);
            return BAD_SPEC.into();
        }
    };

    if !kws::is_any_kw(&key) {
        error!("invalid Mapping key: {}", key);
        BAD_SPEC.into()
    } else {
        Ok(key)
    }
}

fn v2string(v: Value, kw: &str) -> Result<String, TbError> {
    match v {
        Value::String(s) => Ok(s),
        ov => {
            error!("`{}` must be a String: deserialized {:?}", kw, ov);
            BAD_SPEC.into()
        }
    }
}

fn v2bool(v: Value, kw: &str) -> Result<bool, TbError> {
    match v {
        Value::Bool(b) => Ok(b),
        ov => {
            error!("`{}` must be a Bool: deserialized {:?}", kw, ov);
            BAD_SPEC.into()
        }
    }
}

#[cfg(test)]
mod test_yaml_parser;
