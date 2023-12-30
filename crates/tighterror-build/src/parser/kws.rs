pub const CAT_DOC: &str = "cat_doc";
pub const DISPLAY: &str = "display";
pub const DOC: &str = "doc";
pub const DOC_FROM_DISPLAY: &str = "doc_from_display";
pub const DST: &str = "dst";
pub const ERR_DOC: &str = "err_doc";
pub const ERR_CODE_DOC: &str = "err_code_doc";
pub const ERRORS: &str = "errors";
pub const MOD_DOC: &str = "mod_doc";
pub const NAME: &str = "name";
pub const TIGHTERROR: &str = "tighterror";
pub const ERR_INTO_RESULT: &str = "err_into_result";
pub const ERR_CODE_INTO_RESULT: &str = "err_code_into_result";
pub const ERROR_TRAIT: &str = "error_trait";
pub const ERR_NAME: &str = "err_name";
pub const ERR_CODE_NAME: &str = "err_code_name";

pub const ERR_KWS: [&str; 4] = [NAME, DISPLAY, DOC, DOC_FROM_DISPLAY];
pub const MAIN_KWS: [&str; 11] = [
    DST,
    DOC_FROM_DISPLAY,
    MOD_DOC,
    CAT_DOC,
    ERR_DOC,
    ERR_CODE_DOC,
    ERR_INTO_RESULT,
    ERR_CODE_INTO_RESULT,
    ERROR_TRAIT,
    ERR_NAME,
    ERR_CODE_NAME,
];
pub const ROOT_KWS: [&str; 2] = [TIGHTERROR, ERRORS];
pub const ALL_KWS: [&str; 16] = [
    CAT_DOC,
    DISPLAY,
    DOC,
    DOC_FROM_DISPLAY,
    DST,
    ERR_CODE_DOC,
    ERR_DOC,
    ERRORS,
    NAME,
    TIGHTERROR,
    MOD_DOC,
    ERR_INTO_RESULT,
    ERR_CODE_INTO_RESULT,
    ERROR_TRAIT,
    ERR_NAME,
    ERR_CODE_NAME,
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
pub fn is_main_kw(s: &str) -> bool {
    contains(&MAIN_KWS, s)
}
