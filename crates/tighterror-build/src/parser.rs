use crate::{
    coder::idents,
    errors::{
        codes::{BAD_SPEC, BAD_SPEC_FILE_EXTENSION},
        TebError,
    },
    spec::Spec,
    util::open_spec_file,
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

pub fn from_path(path: PathBuf) -> Result<Spec, TebError> {
    match path.extension() {
        #[cfg(feature = "yaml")]
        Some(e) if e == "yaml" => YamlParser::from_file(open_spec_file(&path)?),
        #[cfg(feature = "toml")]
        Some(e) if e == "toml" => TomlParser::from_file(open_spec_file(&path)?),
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

fn check_ident_chars(ident: &str, name: &str) -> Result<(), TebError> {
    let rg = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
    if !rg.is_match(ident) {
        log::error!(
            "`{}` contains invalid characters. Only [A-Za-z0-9_] are allowed: {}",
            name,
            ident
        );
        BAD_SPEC.into()
    } else {
        Ok(())
    }
}

fn check_ident(ident: &str, name: &str) -> Result<(), TebError> {
    use convert_case::{Case, Casing};

    if ident.is_empty() {
        log::error!("`{}` cannot be an empty string", name);
        return BAD_SPEC.into();
    } else if !ident.is_case(Case::UpperCamel) {
        log::error!(
            "`{}` must be specified in UpperCamel case: {} -> {}",
            name,
            ident,
            ident.to_case(Case::UpperCamel)
        );
        return BAD_SPEC.into();
    }

    check_ident_chars(ident, name)?;

    Ok(())
}

fn check_main_ident(ident: &str, kw: &str) -> Result<(), TebError> {
    crate::parser::check_ident(ident, kw)?;
    if kw == kws::ERR_NAME && ident == idents::ERROR {
        return Ok(());
    }
    if kw == kws::ERR_CODE_NAME && ident == idents::ERROR_CODE {
        return Ok(());
    }
    if kw == kws::ERR_CAT_NAME && ident == idents::ERROR_CATEGORY {
        return Ok(());
    }
    if idents::is_top_level_ident(ident) {
        log::error!("`{}` cannot be a reserved identifier: {}", kw, ident);
        BAD_SPEC.into()
    } else {
        Ok(())
    }
}

fn check_name(name: &str) -> Result<(), TebError> {
    check_ident(name, kws::NAME)?;
    if kws::is_any_kw(name) {
        // double check, in case any logic above changes
        log::error!(
            "`{}` cannot be a reserved keyword: `{}`. Use camel case.",
            kws::NAME,
            name
        );
        BAD_SPEC.into()
    } else {
        Ok(())
    }
}
