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
pub const MODULES: &str = "modules";
pub const CATEGORY: &str = "category";
pub const CATEGORIES: &str = "categories";
pub const FLAT_KINDS: &str = "flat_kinds";
pub const VARIANT_TYPE: &str = "variant_type";

pub const ALL_KWS: [&str; 23] = [
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
    MODULES,
    CATEGORY,
    CATEGORIES,
    FLAT_KINDS,
    VARIANT_TYPE,
];

#[inline]
fn contains(arr: &[&str], s: &str) -> bool {
    arr.contains(&s)
}

#[inline]
pub fn is_any_kw(s: &str) -> bool {
    contains(&ALL_KWS, s)
}
