use crate::{
    coder::idents,
    errors::{kinds::parser::*, TbError},
    spec::Spec,
    util::{get_non_unique_names, open_spec_file},
};
use regex::Regex;
use std::path::PathBuf;

#[cfg(not(any(feature = "yaml", feature = "toml")))]
compile_error!("At least one of the markup language features ['yaml', 'toml'] must be enabled.");

cfg_if::cfg_if! {
    if #[cfg(feature = "yaml")] {
        mod yaml;
        pub(crate) use yaml::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        mod toml;
        pub(crate) use self::toml::*;
    }
}

mod kws;

#[derive(Debug)]
pub enum ParseMode {
    Single,
    List,
}

pub fn from_path(path: PathBuf) -> Result<Spec, TbError> {
    match path.extension() {
        #[cfg(feature = "yaml")]
        Some(e) if e == "yaml" => YamlParser::parse_file(open_spec_file(&path)?),
        #[cfg(feature = "toml")]
        Some(e) if e == "toml" => TomlParser::parse_file(open_spec_file(&path)?),
        Some(e) => {
            log::error!(
                "specification file extension {:?} isn't supported: {:?}",
                e,
                path
            );
            BAD_SPEC_FILE_EXTENSION.into()
        }
        None => {
            log::error!(
                "specification file name must have a markup language extension: {:?}",
                path
            );
            BAD_SPEC_FILE_EXTENSION.into()
        }
    }
}

fn check_ident_chars(ident: &str, name: &str) -> Result<(), TbError> {
    let rg = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
    if !rg.is_match(ident) {
        log::error!(
            "`{}` contains unsupported characters. Only [A-Za-z0-9_] are allowed: {}",
            name,
            ident
        );
        BAD_IDENTIFIER_CHARACTERS.into()
    } else {
        Ok(())
    }
}

fn check_ident(ident: &str, name: &str) -> Result<(), TbError> {
    use convert_case::{Case, Casing};

    if ident.is_empty() {
        log::error!("`{}` cannot be an empty string", name);
        return EMPTY_IDENTIFIER.into();
    }

    check_ident_chars(ident, name)?;

    if !ident.is_case(Case::UpperCamel) {
        log::error!(
            "`{}` must be specified in UpperCamel case: {} -> {}",
            name,
            ident,
            ident.to_case(Case::UpperCamel)
        );
        return BAD_IDENTIFIER_CASE.into();
    }

    Ok(())
}

fn check_module_ident(ident: &str, kw: &str) -> Result<(), TbError> {
    crate::parser::check_ident(ident, kw)?;
    if kw == kws::ERR_NAME && ident == idents::ERROR {
        return Ok(());
    }
    if kw == kws::ERR_KIND_NAME && ident == idents::ERROR_KIND {
        return Ok(());
    }
    if kw == kws::ERR_CAT_NAME && ident == idents::ERROR_CATEGORY {
        return Ok(());
    }
    if idents::is_top_level_ident(ident) {
        log::error!("`{}` cannot be a reserved identifier: {}", kw, ident);
        BAD_MODULE_IDENTIFIER.into()
    } else {
        Ok(())
    }
}

fn check_name(name: &str) -> Result<(), TbError> {
    check_ident(name, kws::NAME)?;
    if kws::is_any_kw(name) {
        // double check, in case any logic above changes
        log::error!(
            "`{}` cannot be a reserved keyword: `{}`. Use camel case.",
            kws::NAME,
            name
        );
        BAD_NAME.into()
    } else {
        Ok(())
    }
}

fn check_name_uniqueness<'a, I>(item_name: &str, iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    let non_unique_names = get_non_unique_names(iter);
    for name in &non_unique_names {
        log::error!("{} names must be unique: {}", item_name, name);
    }
    non_unique_names
        .is_empty()
        .then_some(())
        .ok_or_else(|| NON_UNIQUE_NAME.into())
}

fn check_error_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("error", iter)
}

fn check_category_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("category", iter)
}

fn check_module_error_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("<flat_kinds> module error", iter)
}

#[cfg(test)]
mod testing {
    use crate::spec::{CategorySpec, ErrorSpec, MainSpec, ModuleSpec, Spec};

    pub const GENERAL_CAT: &str = "General";

    pub fn log_init() {
        env_logger::builder().is_test(true).try_init().ok();
    }

    pub fn spec_from_err(err: ErrorSpec) -> Spec {
        let cat = CategorySpec {
            name: GENERAL_CAT.into(),
            errors: vec![err],
            ..Default::default()
        };

        let module = ModuleSpec {
            categories: vec![cat],
            ..Default::default()
        };

        Spec {
            module,
            ..Default::default()
        }
    }

    pub fn spec_from_err_iter(iter: impl IntoIterator<Item = ErrorSpec>) -> Spec {
        let cat = CategorySpec {
            name: GENERAL_CAT.into(),
            errors: Vec::from_iter(iter),
            ..Default::default()
        };

        let module = ModuleSpec {
            categories: vec![cat],
            ..Default::default()
        };

        Spec {
            module,
            ..Default::default()
        }
    }

    pub fn spec_from_module(mut module: ModuleSpec) -> Spec {
        let err = ErrorSpec {
            name: "DummyErr".into(),
            ..Default::default()
        };

        let cat = CategorySpec {
            name: GENERAL_CAT.into(),
            errors: vec![err],
            ..Default::default()
        };

        module.categories = vec![cat];

        Spec {
            module,
            ..Default::default()
        }
    }

    pub fn spec_from_main(main: MainSpec) -> Spec {
        let err = ErrorSpec {
            name: "DummyErr".into(),
            ..Default::default()
        };

        let cat = CategorySpec {
            name: GENERAL_CAT.into(),
            errors: vec![err],
            ..Default::default()
        };

        let module = ModuleSpec {
            categories: vec![cat],
            ..Default::default()
        };

        Spec { main, module }
    }

    pub fn spec_from_category(mut cat: CategorySpec) -> Spec {
        let err = ErrorSpec {
            name: "DummyErr".into(),
            ..Default::default()
        };
        cat.errors = vec![err];

        let module = ModuleSpec {
            categories: vec![cat],
            ..Default::default()
        };

        Spec {
            module,
            ..Default::default()
        }
    }
}
