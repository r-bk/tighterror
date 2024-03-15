pub const ERR_CAT_DOC: &str = "err_cat_doc";
pub const DISPLAY: &str = "display";
pub const DOC: &str = "doc";
pub const DOC_FROM_DISPLAY: &str = "doc_from_display";
pub const OUTPUT: &str = "output";
pub const ERR_DOC: &str = "err_doc";
pub const ERR_KIND_DOC: &str = "err_kind_doc";
pub const ERRORS: &str = "errors";
pub const MOD_DOC: &str = "mod_doc";
pub const NAME: &str = "name";
pub const TIGHTERROR: &str = "tighterror";
pub const RESULT_FROM_ERR: &str = "result_from_err";
pub const RESULT_FROM_ERR_KIND: &str = "result_from_err_kind";
pub const ERROR_TRAIT: &str = "error_trait";
pub const ERR_NAME: &str = "err_name";
pub const ERR_KIND_NAME: &str = "err_kind_name";
pub const ERR_CAT_NAME: &str = "err_cat_name";
pub const NO_STD: &str = "no_std";

pub const ERR_KWS: [&str; 4] = [NAME, DISPLAY, DOC, DOC_FROM_DISPLAY];
pub const MAIN_KWS: [&str; 13] = [
    OUTPUT,
    DOC_FROM_DISPLAY,
    MOD_DOC,
    ERR_CAT_DOC,
    ERR_DOC,
    ERR_KIND_DOC,
    RESULT_FROM_ERR,
    RESULT_FROM_ERR_KIND,
    ERROR_TRAIT,
    ERR_NAME,
    ERR_KIND_NAME,
    ERR_CAT_NAME,
    NO_STD,
];
pub const ROOT_KWS: [&str; 2] = [TIGHTERROR, ERRORS];
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
    TIGHTERROR,
    MOD_DOC,
    RESULT_FROM_ERR,
    RESULT_FROM_ERR_KIND,
    ERROR_TRAIT,
    ERR_NAME,
    ERR_KIND_NAME,
    ERR_CAT_NAME,
    NO_STD,
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
    contains(&MAIN_KWS, s)
}
