use crate::{
    errors::{
        kinds::{BAD_SPEC, BAD_TOML},
        TebError,
    },
    parser::{check_main_ident, check_name, kws},
    spec::{CategorySpec, ErrorSpec, MainSpec, Spec, IMPLICIT_CATEGORY_NAME},
    util::get_non_unique_error_names,
};
use std::fs::File;
use toml::Value;

#[derive(Debug)]
pub struct TomlParser;

impl TomlParser {
    pub fn from_file(mut file: File) -> Result<Spec, TebError> {
        use std::io::Read;

        let mut s = String::new();
        if let Err(e) = file.read_to_string(&mut s) {
            log::error!("failed to read the spec file: {e}");
            return BAD_TOML.into();
        }

        Self::from_str(&s)
    }

    pub fn from_str(s: &str) -> Result<Spec, TebError> {
        match toml::from_str(s) {
            Ok(v) => Self::from_value(v),
            Err(e) => {
                log::error!("failed to deserialize TOML: {e}");
                BAD_TOML.into()
            }
        }
    }

    fn from_value(value: toml::Value) -> Result<Spec, TebError> {
        match value {
            Value::Table(t) => Self::from_table(t),
            v => {
                log::error!(
                    "spec document must be a Table: deserialized a {}",
                    value_type_name(&v)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn from_table(mut table: toml::Table) -> Result<Spec, TebError> {
        for k in table.keys() {
            if !kws::is_root_kw(k) {
                log::error!("invalid top-level key: {}", k);
                return BAD_SPEC.into();
            }
        }

        let mut spec = Spec::default();

        if let Some(v) = table.remove(kws::TIGHTERROR) {
            spec.main = MainSpecParser::from_value(v)?;
        }

        if let Some(v) = table.remove(kws::ERRORS) {
            let errors = TomlErrorsParser::from_value(v)?;
            spec.categories.push(CategorySpec {
                name: IMPLICIT_CATEGORY_NAME.into(),
                errors,
                ..Default::default()
            });
        } else {
            log::error!("'{}' key is missing", kws::ERRORS);
            return BAD_SPEC.into();
        };

        let non_unique_errors = get_non_unique_error_names(&spec);
        for name in &non_unique_errors {
            log::error!("error names must be unique: {}", name);
        }
        if !non_unique_errors.is_empty() {
            return BAD_SPEC.into();
        }

        Ok(spec)
    }
}

#[derive(Debug)]
pub struct MainSpecParser;

impl MainSpecParser {
    fn from_value(v: Value) -> Result<MainSpec, TebError> {
        match v {
            Value::Table(t) => Self::from_table(t),
            ref ov => {
                log::error!(
                    "`{}` must be a Table: deserialized a {}",
                    kws::TIGHTERROR,
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn from_table(t: toml::Table) -> Result<MainSpec, TebError> {
        let mut main_spec = MainSpec::default();

        for (k, v) in t.into_iter() {
            let key = check_key(&k)?;

            if !kws::is_main_kw(key) {
                log::error!("invalid `{}` key: {}", kws::TIGHTERROR, key);
                return BAD_SPEC.into();
            }

            match key {
                kws::OUTPUT => main_spec.output = Some(v2string(v, kws::OUTPUT)?),
                kws::DOC_FROM_DISPLAY => {
                    main_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?)
                }
                kws::ERR_CAT_DOC => main_spec.err_cat_doc = Some(v2string(v, kws::ERR_CAT_DOC)?),
                kws::ERR_KIND_DOC => main_spec.err_kind_doc = Some(v2string(v, kws::ERR_KIND_DOC)?),
                kws::ERR_DOC => main_spec.err_doc = Some(v2string(v, kws::ERR_DOC)?),
                kws::MOD_DOC => main_spec.mod_doc = Some(v2string(v, kws::MOD_DOC)?),
                kws::RESULT_FROM_ERR => {
                    main_spec.result_from_err = Some(v2bool(v, kws::RESULT_FROM_ERR)?)
                }
                kws::RESULT_FROM_ERR_KIND => {
                    main_spec.result_from_err_kind = Some(v2bool(v, kws::RESULT_FROM_ERR_KIND)?)
                }
                kws::ERROR_TRAIT => main_spec.error_trait = Some(v2bool(v, kws::ERROR_TRAIT)?),
                kws::ERR_NAME => {
                    let err_name = v2string(v, kws::ERR_NAME)?;
                    check_main_ident(&err_name, kws::ERR_NAME)?;
                    main_spec.err_name = Some(err_name);
                }
                kws::ERR_KIND_NAME => {
                    let err_kind_name = v2string(v, kws::ERR_KIND_NAME)?;
                    check_main_ident(&err_kind_name, kws::ERR_KIND_NAME)?;
                    main_spec.err_kind_name = Some(err_kind_name);
                }
                kws::ERR_CAT_NAME => {
                    let err_cat_name = v2string(v, kws::ERR_CAT_NAME)?;
                    check_main_ident(&err_cat_name, kws::ERR_CAT_NAME)?;
                    main_spec.err_cat_name = Some(err_cat_name);
                }
                _ => panic!("internal error: unhandled main key {}", key),
            }
        }

        Ok(main_spec)
    }
}

#[derive(Debug)]
pub struct TomlErrorsParser;

impl TomlErrorsParser {
    fn from_value(v: Value) -> Result<Vec<ErrorSpec>, TebError> {
        match v {
            Value::Array(a) => Self::from_array(a),
            ref ov => {
                log::error!(
                    "`{}` must be an Array: deserialized a {}",
                    kws::ERRORS,
                    value_type_name(ov)
                );
                BAD_SPEC.into()
            }
        }
    }

    fn from_array(a: toml::value::Array) -> Result<Vec<ErrorSpec>, TebError> {
        let mut errors = Vec::new();
        for v in a.into_iter() {
            match v {
                Value::String(s) => errors.push(TomlErrorParser::from_string(s)?),
                Value::Table(t) => errors.push(TomlErrorParser::from_table(t)?),
                ov => {
                    log::error!(
                        "an error must be a String or a Table: deserialized {:?}",
                        ov
                    );
                    return BAD_SPEC.into();
                }
            }
        }
        Ok(errors)
    }
}

#[derive(Debug)]
pub struct TomlErrorParser;

impl TomlErrorParser {
    fn from_string(s: String) -> Result<ErrorSpec, TebError> {
        check_name(&s)?;
        Ok(ErrorSpec {
            name: s,
            ..Default::default()
        })
    }

    fn from_table(t: toml::Table) -> Result<ErrorSpec, TebError> {
        let mut err_spec = ErrorSpec::default();

        for (k, v) in t.into_iter() {
            let key = check_key(&k)?;

            if !kws::is_err_kw(key) {
                log::error!("invalid error Table key: {}", key);
                return BAD_SPEC.into();
            }

            match key {
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

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Array(_) => "Array",
        Value::Boolean(_) => "Boolean",
        Value::Datetime(_) => "Datetime",
        Value::Float(_) => "Float",
        Value::Integer(_) => "Integer",
        Value::String(_) => "String",
        Value::Table(_) => "Table",
    }
}

fn check_key(k: &str) -> Result<&str, TebError> {
    if !kws::is_any_kw(k) {
        log::error!("invalid Table key: {}", k);
        BAD_SPEC.into()
    } else {
        Ok(k)
    }
}

fn v2string(v: Value, kw: &str) -> Result<String, TebError> {
    match v {
        Value::String(s) => Ok(s),
        ov => {
            log::error!("`{}` must be a String: deserialized {:?}", kw, ov);
            BAD_SPEC.into()
        }
    }
}

fn v2bool(v: Value, kw: &str) -> Result<bool, TebError> {
    match v {
        Value::Boolean(b) => Ok(b),
        ov => {
            log::error!("`{}` must be a Boolean: deserialized {:?}", kw, ov);
            BAD_SPEC.into()
        }
    }
}

#[cfg(test)]
mod test_toml_parser;
