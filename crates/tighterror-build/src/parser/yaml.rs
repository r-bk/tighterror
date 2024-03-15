use crate::{
    errors::{
        kinds::{BAD_SPEC, BAD_YAML},
        TebError,
    },
    parser::{check_module_ident, check_name, kws},
    spec::{CategorySpec, ErrorSpec, ModuleSpec, Spec, IMPLICIT_CATEGORY_NAME},
    util::get_non_unique_error_names,
};
use log::error;
use serde_yaml::{Mapping, Sequence, Value};
use std::fs::File;

#[derive(Debug)]
pub struct YamlParser;

#[derive(Debug)]
struct YamlErrorsParser;

#[derive(Debug)]
struct YamlErrorParser;

#[derive(Debug)]
struct ModuleSpecParser;

impl YamlParser {
    pub fn from_file(file: File) -> Result<Spec, TebError> {
        match serde_yaml::from_reader(file) {
            Ok(v) => Self::from_value(v),
            Err(e) => {
                log::error!("failed to deserialize YAML: {e}");
                BAD_YAML.into()
            }
        }
    }

    #[cfg(test)]
    pub fn from_str(s: &str) -> Result<Spec, TebError> {
        match serde_yaml::from_str(s) {
            Ok(v) => Self::from_value(v),
            Err(e) => {
                log::error!("failed to deserialize YAML: {e}");
                BAD_YAML.into()
            }
        }
    }

    pub fn from_value(value: Value) -> Result<Spec, TebError> {
        match value {
            Value::Mapping(m) => Self::from_mapping(m),
            v => {
                error!(
                    "spec document must be a Mapping: deserialized a {}",
                    value_type_name(&v)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn from_mapping(mut m: Mapping) -> Result<Spec, TebError> {
        for k in m.keys() {
            match k {
                Value::String(s) => {
                    if !kws::is_root_kw(s) {
                        error!("invalid top-level key: {}", s);
                        return BAD_SPEC.into();
                    }
                }
                ov => {
                    error!("a Mapping key must be a String: deserialized {:?}", ov);
                    return BAD_SPEC.into();
                }
            }
        }

        let mut spec = Spec::default();

        if let Some(v) = m.remove(kws::TIGHTERROR) {
            spec.module = ModuleSpecParser::from_value(v)?;
        }

        if let Some(v) = m.remove(kws::ERRORS) {
            let errors = YamlErrorsParser::from_value(v)?;
            spec.categories.push(CategorySpec {
                name: IMPLICIT_CATEGORY_NAME.into(),
                errors,
                ..Default::default()
            });
        } else {
            error!("'{}' key is missing", kws::ERRORS);
            return BAD_SPEC.into();
        };

        let non_unique_errors = get_non_unique_error_names(&spec);
        for name in &non_unique_errors {
            error!("error names must be unique: {}", name);
        }
        if !non_unique_errors.is_empty() {
            return BAD_SPEC.into();
        }

        Ok(spec)
    }
}

impl ModuleSpecParser {
    fn from_value(v: Value) -> Result<ModuleSpec, TebError> {
        match v {
            Value::Mapping(m) => Self::from_mapping(m),
            ref ov => {
                error!(
                    "`{}` must be a Mapping: deserialized a {}",
                    kws::TIGHTERROR,
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn from_mapping(m: Mapping) -> Result<ModuleSpec, TebError> {
        let mut mod_spec = ModuleSpec::default();

        for (k, v) in m.into_iter() {
            let key = v2key(k)?;

            if !kws::is_mod_kw(&key) {
                error!("invalid `{}` key: {}", kws::TIGHTERROR, key);
                return BAD_SPEC.into();
            }

            match key.as_str() {
                kws::OUTPUT => mod_spec.output = Some(v2string(v, kws::OUTPUT)?),
                kws::DOC_FROM_DISPLAY => {
                    mod_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?)
                }
                kws::ERR_CAT_DOC => mod_spec.err_cat_doc = Some(v2string(v, kws::ERR_CAT_DOC)?),
                kws::ERR_KIND_DOC => mod_spec.err_kind_doc = Some(v2string(v, kws::ERR_KIND_DOC)?),
                kws::ERR_DOC => mod_spec.err_doc = Some(v2string(v, kws::ERR_DOC)?),
                kws::MOD_DOC => mod_spec.mod_doc = Some(v2string(v, kws::MOD_DOC)?),
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
                kws::NO_STD => mod_spec.no_std = Some(v2bool(v, kws::NO_STD)?),
                _ => panic!("internal error: unhandled module key {}", key),
            }
        }

        Ok(mod_spec)
    }
}

impl YamlErrorsParser {
    fn from_value(v: Value) -> Result<Vec<ErrorSpec>, TebError> {
        match v {
            Value::Sequence(s) => Self::from_sequence(s),
            ref ov => {
                error!(
                    "`{}` must be a Sequence: deserialized a {}",
                    kws::ERRORS,
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn from_sequence(s: Sequence) -> Result<Vec<ErrorSpec>, TebError> {
        let mut errors = Vec::new();
        for v in s.into_iter() {
            match v {
                Value::String(s) => errors.push(YamlErrorParser::from_string(s)?),
                Value::Mapping(m) => errors.push(YamlErrorParser::from_mapping(m)?),
                ov => {
                    error!(
                        "an error must be a String or a Mapping: deserialized {:?}",
                        ov
                    );
                    return BAD_SPEC.into();
                }
            }
        }
        Ok(errors)
    }
}

impl YamlErrorParser {
    fn from_string(s: String) -> Result<ErrorSpec, TebError> {
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

    fn from_mapping(m: Mapping) -> Result<ErrorSpec, TebError> {
        match m.len() {
            0 => {
                error!("an error must be a non-empty Mapping: deserialized {:?}", m);
                BAD_SPEC.into()
            }
            1 if Self::is_short_mapping(&m) => Self::from_short_mapping(m),
            _ => Self::from_long_mapping(m),
        }
    }

    fn from_short_mapping(m: Mapping) -> Result<ErrorSpec, TebError> {
        assert_eq!(m.len(), 1);
        let (k, v) = m.into_iter().next().unwrap();

        let name = match k {
            Value::String(s) => s,
            ov => {
                error!("`{}` must be a String: deserialized {:?}", kws::NAME, ov);
                return BAD_SPEC.into();
            }
        };

        check_name(&name)?;

        let display = match v {
            Value::String(s) => s,
            ov => {
                error!("`{}` must be a String: deserialized {:?}", kws::DISPLAY, ov);
                return BAD_SPEC.into();
            }
        };

        Ok(ErrorSpec {
            name,
            display: Some(display),
            ..Default::default()
        })
    }

    fn from_long_mapping(m: Mapping) -> Result<ErrorSpec, TebError> {
        let mut err_spec = ErrorSpec::default();

        for (k, v) in m.into_iter() {
            let key = v2key(k)?;

            if !kws::is_err_kw(&key) {
                error!("invalid error Mapping key: {}", key);
                return BAD_SPEC.into();
            }

            match key.as_str() {
                kws::NAME => err_spec.name = v2string(v, kws::NAME)?,
                kws::DISPLAY => err_spec.display = Some(v2string(v, kws::DISPLAY)?),
                kws::DOC => err_spec.doc = Some(v2string(v, kws::DOC)?),
                kws::DOC_FROM_DISPLAY => {
                    err_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?)
                }
                _ => panic!("internal error: unhandled error keyword {}", key),
            }
        }

        check_name(&err_spec.name)?;

        Ok(err_spec)
    }
}

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

fn v2key(v: Value) -> Result<String, TebError> {
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

fn v2string(v: Value, kw: &str) -> Result<String, TebError> {
    match v {
        Value::String(s) => Ok(s),
        ov => {
            error!("`{}` must be a String: deserialized {:?}", kw, ov);
            BAD_SPEC.into()
        }
    }
}

fn v2bool(v: Value, kw: &str) -> Result<bool, TebError> {
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
