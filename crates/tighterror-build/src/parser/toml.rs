use crate::{
    errors::{kinds::parser::*, TbError},
    parser::{
        check_category_name_uniqueness, check_error_name_uniqueness,
        check_module_error_name_uniqueness, check_module_ident, check_module_name,
        check_module_name_uniqueness, check_name, kws, ParseMode,
    },
    spec::{
        definitions::DEFAULT_FLAT_KINDS, CategorySpec, ErrorSpec, MainSpec, ModuleSpec, Spec,
        IMPLICIT_CATEGORY_NAME,
    },
};
use std::fs::File;
use toml::{value::Array, Table, Value};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct TomlParser;

impl TomlParser {
    pub fn parse_file(mut file: File) -> Result<Spec, TbError> {
        use std::io::Read;

        let mut s = String::new();
        if let Err(e) = file.read_to_string(&mut s) {
            log::error!("failed to read the specification file: {e}");
            return BAD_TOML.into();
        }

        Self::parse_str(&s)
    }

    pub fn parse_str(s: &str) -> Result<Spec, TbError> {
        match toml::from_str(s) {
            Ok(v) => Self::value(v),
            Err(e) => {
                log::error!("failed to deserialize TOML: {e}");
                BAD_TOML.into()
            }
        }
    }

    fn value(value: toml::Value) -> Result<Spec, TbError> {
        match value {
            Value::Table(t) => Self::table(t),
            v => {
                log::error!(
                    "specification document must be a Table: deserialized a {}",
                    value_type_name(&v)
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn table(mut table: toml::Table) -> Result<Spec, TbError> {
        Self::check_toplevel_attributes(&table)?;

        let mut spec = Spec::default();

        if let Some(v) = table.remove(kws::MAIN) {
            spec.main = MainParser::value(v)?;
        }

        if let Some(v) = table.remove(kws::MODULES) {
            spec.modules = ModulesParser::value(v)?;
        }

        if let Some(v) = table.remove(kws::MODULE) {
            let mp = ModuleParser(ParseMode::Single);
            spec.modules.push(mp.value(v)?);
        }

        if let Some(v) = table.remove(kws::CATEGORY) {
            let parser = CategoryParser(ParseMode::Single);
            let cat_spec = parser.value(v)?;
            if let Some(m) = spec.modules.first_mut() {
                m.categories.push(cat_spec);
            } else {
                spec.modules
                    .push(ModuleSpec::implicit_with_categories(vec![cat_spec]));
            }
        }

        if let Some(v) = table.remove(kws::CATEGORIES) {
            let categories = CategoriesParser::value(v)?;
            if let Some(m) = spec.modules.first_mut() {
                m.categories = categories;
            } else {
                spec.modules
                    .push(ModuleSpec::implicit_with_categories(categories));
            }
        }

        if let Some(v) = table.remove(kws::ERRORS) {
            let errors = ErrorsParser::value(v)?;
            if let Some(m) = spec.modules.first_mut() {
                if let Some(c) = m.categories.first_mut() {
                    c.errors = errors;
                } else {
                    m.categories
                        .push(CategorySpec::implicit_with_errors(errors));
                }
            } else {
                spec.modules.push(ModuleSpec::implicit_with_categories(vec![
                    CategorySpec::implicit_with_errors(errors),
                ]));
            }
        }

        for m in &spec.modules {
            if m.flat_kinds.unwrap_or(DEFAULT_FLAT_KINDS) {
                check_module_error_name_uniqueness(m.errors_iter().map(|e| e.name.as_str()))?;
            }
        }

        Ok(spec)
    }

    fn check_toplevel_attributes(table: &toml::Table) -> Result<(), TbError> {
        for k in table.keys() {
            if !kws::is_root_kw(k) {
                log::error!("invalid root-level keyword: {}", k);
                return BAD_ROOT_LEVEL_KEYWORD.into();
            }
        }

        for (k1, k2) in kws::MUTUALLY_EXCLUSIVE_ROOT_KWS {
            if table.contains_key(k1) && table.contains_key(k2) {
                log::error!("root-level attributes '{k1}' and '{k2}' are mutually exclusive");
                return MUTUALLY_EXCLUSIVE_KEYWORDS.into();
            }
        }

        if !table
            .keys()
            .any(|k| kws::REQUIRED_ROOT_KWS.iter().any(|req| req == k))
        {
            log::error!(
                "one of {:?} root-level attributes must be specified",
                kws::REQUIRED_ROOT_KWS
            );
            return MISSING_ATTRIBUTE.into();
        }

        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct MainParser;

impl MainParser {
    fn value(v: Value) -> Result<MainSpec, TbError> {
        match v {
            Value::Table(t) => Self::table(t),
            ref ov => {
                log::error!(
                    "MainObject must be a Table: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn table(t: toml::Table) -> Result<MainSpec, TbError> {
        let mut main_spec = MainSpec::default();

        for (k, v) in t.into_iter() {
            let key = check_key(&k)?;

            if !kws::is_main_kw(key) {
                log::error!("invalid MainObject attribute: {}", key);
                return BAD_OBJECT_ATTRIBUTE.into();
            }

            match key {
                kws::OUTPUT => main_spec.output = Some(v2string(v, kws::OUTPUT)?.into()),
                kws::NO_STD => main_spec.no_std = Some(v2bool(v, kws::NO_STD)?),
                _ => panic!("internal error: unhandled MainObject attribute: {}", key),
            }
        }

        Ok(main_spec)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ModuleParser(ParseMode);

impl ModuleParser {
    fn value(&self, v: Value) -> Result<ModuleSpec, TbError> {
        match v {
            Value::Table(t) => self.table(t),
            ref ov => {
                log::error!(
                    "ModuleObject must be a Table: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn table(&self, t: toml::Table) -> Result<ModuleSpec, TbError> {
        let mut mod_spec = ModuleSpec::default();

        for (k, v) in t.into_iter() {
            let key = check_key(&k)?;

            if !kws::is_mod_kw(key) {
                log::error!("invalid ModuleObject attribute: {}", key);
                return BAD_OBJECT_ATTRIBUTE.into();
            }

            match key {
                kws::NAME => mod_spec.name = Some(v2string(v, kws::NAME)?),
                kws::CATEGORIES => {
                    if let ParseMode::Single = self.0 {
                        log::error!(
                            "CategoriesList is not allowed in root-level `{}` attribute",
                            kws::MODULE
                        );
                        return BAD_OBJECT_ATTRIBUTE.into();
                    }
                    mod_spec.categories = CategoriesParser::value(v)?;
                }
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

        if let Some(ref n) = mod_spec.name {
            check_module_name(n)?;
        }

        if let ParseMode::List = self.0 {
            if let Some(ref name) = mod_spec.name {
                if mod_spec.categories.is_empty() {
                    log::error!("CategoriesList is missing: module = {name}");
                    return MISSING_ATTRIBUTE.into();
                }
            } else {
                log::error!("ModuleObject name is mandatory in ModulesList");
                return MISSING_ATTRIBUTE.into();
            }
        }

        Ok(mod_spec)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct ModulesParser;

impl ModulesParser {
    fn value(v: Value) -> Result<Vec<ModuleSpec>, TbError> {
        match v {
            Value::Array(a) => Self::array(a),
            ref ov => {
                log::error!("ModulesList must be an Array: deserialized {:?}", ov);
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn array(s: Array) -> Result<Vec<ModuleSpec>, TbError> {
        let mut modules = Vec::new();
        for v in s.into_iter() {
            let mp = ModuleParser(ParseMode::List);
            modules.push(mp.value(v)?);
        }
        if modules.is_empty() {
            log::error!("Empty ModulesList is not allowed");
            return EMPTY_LIST.into();
        }
        check_module_name_uniqueness(modules.iter().map(|m| m.name()))?;
        Ok(modules)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ErrorsParser;

impl ErrorsParser {
    fn value(v: Value) -> Result<Vec<ErrorSpec>, TbError> {
        match v {
            Value::Array(a) => Self::array(a),
            ref ov => {
                log::error!(
                    "`{}` must be an Array: deserialized a {}",
                    kws::ERRORS,
                    value_type_name(ov)
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn array(a: toml::value::Array) -> Result<Vec<ErrorSpec>, TbError> {
        let mut errors = Vec::new();
        for v in a.into_iter() {
            match v {
                Value::String(s) => errors.push(ErrorParser::string(s)?),
                Value::Table(t) => errors.push(ErrorParser::table(t)?),
                ov => {
                    log::error!(
                        "ErrorObject must be a String or a Table: deserialized {:?}",
                        ov
                    );
                    return BAD_VALUE_TYPE.into();
                }
            }
        }
        if errors.is_empty() {
            log::error!("Empty ErrorsList is not allowed");
            return EMPTY_LIST.into();
        }
        check_error_name_uniqueness(errors.iter().map(|e| e.name.as_str()))?;
        Ok(errors)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ErrorParser;

impl ErrorParser {
    fn string(s: String) -> Result<ErrorSpec, TbError> {
        check_name(&s)?;
        Ok(ErrorSpec {
            name: s,
            ..Default::default()
        })
    }

    fn table(t: toml::Table) -> Result<ErrorSpec, TbError> {
        let mut err_spec = ErrorSpec::default();

        for (k, v) in t.into_iter() {
            let key = check_key(&k)?;

            if !kws::is_err_kw(key) {
                log::error!("invalid ErrorObject attribute: {}", key);
                return BAD_OBJECT_ATTRIBUTE.into();
            }

            match key {
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
            Value::Table(t) => self.table(t),
            ref ov => {
                log::error!(
                    "ModuleObject must be a Table: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn table(&self, mut t: Table) -> Result<CategorySpec, TbError> {
        for k in t.keys() {
            if !kws::is_cat_kw(k) {
                log::error!("invalid CategoryObject attribute: {}", k);
                return BAD_OBJECT_ATTRIBUTE.into();
            }
        }

        let mut cat_spec = CategorySpec::default();

        if let Some(v) = t.remove(kws::NAME) {
            let name = v2string(v, kws::NAME)?;
            check_name(&name)?;
            cat_spec.name = name;
        }

        if let Some(v) = t.remove(kws::DOC) {
            cat_spec.doc = Some(v2string(v, kws::DOC)?);
        }

        if let Some(v) = t.remove(kws::DOC_FROM_DISPLAY) {
            cat_spec.oes.doc_from_display = Some(v2bool(v, kws::DOC_FROM_DISPLAY)?);
        }

        if let Some(v) = t.remove(kws::ERRORS) {
            if matches!(self.0, ParseMode::Single) {
                log::error!(
                    "ErrorsList is not allowed in root-level '{}' attribute",
                    kws::CATEGORY
                );
                return BAD_OBJECT_ATTRIBUTE.into();
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
                    log::error!("CategoryObject name is mandatory in CategoriesList");
                    return MISSING_ATTRIBUTE.into();
                }
                if cat_spec.errors.is_empty() {
                    log::error!("ErrorsList not found: category_name = {}", cat_spec.name);
                    return MISSING_ATTRIBUTE.into();
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
            Value::Array(a) => Self::array(a),
            ref ov => {
                log::error!(
                    "CategoryList must be an Array: deserialized a {}",
                    value_type_name(ov)
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn array(a: Vec<Value>) -> Result<Vec<CategorySpec>, TbError> {
        let mut categories = Vec::new();
        for v in a.into_iter() {
            match v {
                Value::Table(t) => {
                    let parser = CategoryParser(ParseMode::List);
                    let cat_spec = parser.table(t)?;
                    categories.push(cat_spec);
                }
                ov => {
                    log::error!(
                        "CategoryObject in CategoriesList must be a Table: deserialized {:?}",
                        ov
                    );
                    return BAD_VALUE_TYPE.into();
                }
            }
        }
        if categories.is_empty() {
            log::error!("Empty CategoriesList is not allowed");
            return EMPTY_LIST.into();
        }
        check_category_name_uniqueness(categories.iter().map(|c| c.name.as_str()))?;
        Ok(categories)
    }
}

// ----------------------------------------------------------------------------

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

fn check_key(k: &str) -> Result<&str, TbError> {
    if !kws::is_any_kw(k) {
        log::error!("invalid Table key: {}", k);
        BAD_OBJECT_ATTRIBUTE.into()
    } else {
        Ok(k)
    }
}

fn v2string(v: Value, kw: &str) -> Result<String, TbError> {
    match v {
        Value::String(s) => Ok(s),
        ov => {
            log::error!("`{}` must be a String: deserialized {:?}", kw, ov);
            BAD_VALUE_TYPE.into()
        }
    }
}

fn v2bool(v: Value, kw: &str) -> Result<bool, TbError> {
    match v {
        Value::Boolean(b) => Ok(b),
        ov => {
            log::error!("`{}` must be a Boolean: deserialized {:?}", kw, ov);
            BAD_VALUE_TYPE.into()
        }
    }
}

#[cfg(test)]
mod test_toml_parser;
