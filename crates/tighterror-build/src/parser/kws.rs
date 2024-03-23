pub const ERR_CAT_DOC: &str = "err_cat_doc";
pub const DISPLAY: &str = "display";
pub const DOC: &str = "doc";
pub const DOC_FROM_DISPLAY: &str = "doc_from_display";
pub const OUTPUT: &str = "output";
pub const ERR_DOC: &str = "err_doc";
pub const ERR_KIND_DOC: &str = "err_kind_doc";
pub const ERRORS: &str = "errors";
pub const MAIN: &str = "main";
pub const NAME: &str = "name";
pub const RESULT_FROM_ERR: &str = "result_from_err";
pub const RESULT_FROM_ERR_KIND: &str = "result_from_err_kind";
pub const ERROR_TRAIT: &str = "error_trait";
pub const ERR_NAME: &str = "err_name";
pub const ERR_KIND_NAME: &str = "err_kind_name";
pub const ERR_CAT_NAME: &str = "err_cat_name";
pub const NO_STD: &str = "no_std";
pub const MODULE: &str = "module";

pub const ERR_KWS: [&str; 4] = [NAME, DISPLAY, DOC, DOC_FROM_DISPLAY];
pub const MODULE_KWS: [&str; 11] = [
    DOC_FROM_DISPLAY,
    DOC,
    ERR_CAT_DOC,
    ERR_DOC,
    ERR_KIND_DOC,
    RESULT_FROM_ERR,
    RESULT_FROM_ERR_KIND,
    ERROR_TRAIT,
    ERR_NAME,
    ERR_KIND_NAME,
    ERR_CAT_NAME,
];
pub const MAIN_KWS: [&str; 2] = [OUTPUT, NO_STD];
pub const ROOT_KWS: [&str; 3] = [MAIN, ERRORS, MODULE];
pub const ALL_KWS: [&str; 18] = [
    ERR_CAT_DOC,
    DISPLAY,
    DOC,
    DOC_FROM_DISPLAY,
    OUTPUT,
    ERR_KIND_DOC,
    ERR_DOC,
    ERRORS,
    NAME,
    MAIN,
    RESULT_FROM_ERR,
    RESULT_FROM_ERR_KIND,
    ERROR_TRAIT,
    ERR_NAME,
    ERR_KIND_NAME,
    ERR_CAT_NAME,
    NO_STD,
    MODULE,
];

#[inline]
fn contains(arr: &[&str], s: &str) -> bool {
    arr.iter().any(|kw| *kw == s)
}

#[inline]
pub fn is_any_kw(s: &str) -> bool {
    contains(&ALL_KWS, s)
}

#[inline]
pub fn is_root_kw(s: &str) -> bool {
    contains(&ROOT_KWS, s)
}

#[inline]
pub fn is_err_kw(s: &str) -> bool {
    contains(&ERR_KWS, s)
}

#[inline]
pub fn is_mod_kw(s: &str) -> bool {
    contains(&MODULE_KWS, s)
}

pub fn is_main_kw(s: &str) -> bool {
    contains(&MAIN_KWS, s)
}
