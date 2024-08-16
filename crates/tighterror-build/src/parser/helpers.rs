use crate::{
    coder::idents,
    common::casing,
    errors::{kind::parser::*, TbError},
    parser::kws,
};
use convert_case::Case;
use regex::Regex;
use std::collections::HashSet;

fn check_ident_chars(ident: &str, desc: &str, case: Case) -> Result<(), TbError> {
    let rgs = match case {
        Case::Pascal | Case::UpperCamel => r"^[A-Za-z0-9]+$",
        _ => r"^[A-Za-z0-9_]+$",
    };
    let rg = Regex::new(rgs).unwrap();
    if !rg.is_match(ident) {
        log::error!(
            "`{desc}` contains unsupported characters. \
            Only {rgs} is allowed with {case:?} case: {ident}",
        );
        BAD_IDENTIFIER_CHARACTERS.into()
    } else {
        Ok(())
    }
}

fn check_ident(ident: &str, desc: &str, case: Case) -> Result<(), TbError> {
    if ident.is_empty() {
        log::error!("`{desc}` cannot be an empty string");
        return EMPTY_IDENTIFIER.into();
    }

    check_ident_chars(ident, desc, case)?;

    if !casing::is_case(ident, case) {
        log::error!(
            "`{desc}` must be specified in {case:?} case: {ident} -> {}",
            casing::convert_case(ident, case, case)
        );
        return BAD_IDENTIFIER_CASE.into();
    }

    Ok(())
}

pub fn check_module_ident(ident: &str, kw: &str) -> Result<(), TbError> {
    check_ident(ident, kw, Case::UpperCamel)?;
    if kw == kws::ERR_NAME && ident == idents::ERROR {
        return Ok(());
    }
    if kw == kws::ERR_KIND_NAME && ident == idents::ERROR_KIND {
        return Ok(());
    }
    if kw == kws::ERR_CAT_NAME && ident == idents::ERROR_CATEGORY {
        return Ok(());
    }
    if idents::is_root_level_ident(ident) {
        log::error!("`{}` cannot be a reserved identifier: {}", kw, ident);
        BAD_MODULE_IDENTIFIER.into()
    } else {
        Ok(())
    }
}

fn check_name(name: &str, desc: &str, case: Case) -> Result<(), TbError> {
    check_ident(name, desc, case)?;
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

pub fn check_error_name(name: &str) -> Result<(), TbError> {
    check_name(name, "ErrorObject::name", Case::UpperSnake)
}

pub fn check_category_name(name: &str) -> Result<(), TbError> {
    check_name(name, "CategoryObject::name", Case::UpperCamel)
}

pub fn check_module_name(name: &str) -> Result<(), TbError> {
    if name.is_empty() {
        log::error!("module name cannot be an empty string");
        BAD_NAME.into()
    } else if !casing::is_case(name, Case::Snake) {
        log::error!("module name must be specified in lower_snake_case: {name}");
        BAD_NAME.into()
    } else {
        Ok(())
    }
}

fn get_non_unique_names<'a, I>(iter: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut ans = HashSet::new();
    let mut hs = HashSet::new();

    for n in iter {
        let lower = n.to_lowercase();
        if hs.contains(&lower) {
            ans.insert(n.to_owned());
        } else {
            hs.insert(lower);
        }
    }

    Vec::from_iter(ans)
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

pub fn check_error_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("error", iter)
}

pub fn check_category_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("category", iter)
}

pub fn check_module_error_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("<flat_kinds> module error", iter)
}

pub fn check_module_name_uniqueness<'a, I>(iter: I) -> Result<(), TbError>
where
    I: IntoIterator<Item = &'a str>,
{
    check_name_uniqueness("module", iter)
}
