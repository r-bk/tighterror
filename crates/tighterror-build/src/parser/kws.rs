pub const SPEC: &str = "spec";
pub const MODULE: &str = "module";
pub const CATEGORY: &str = "category";
pub const ERROR: &str = "error";

pub const OUTPUT: &str = "output";
pub const NO_STD: &str = "no-std";
pub const DOC: &str = "doc";
pub const DISPLAY: &str = "display";
pub const DOC_FROM_DISPLAY: &str = "doc-from-display";
pub const VARIANT_TYPE: &str = "variant-type";
pub const FLAT_KINDS: &str = "flat-kinds";
pub const ERR_DOC: &str = "err-doc";
pub const ERR_KIND_DOC: &str = "err-kind-doc";
pub const ERR_CAT_DOC: &str = "err-cat-doc";
pub const ERR_NAME: &str = "err-name";
pub const ERR_KIND_NAME: &str = "err-kind-name";
pub const ERR_CAT_NAME: &str = "err-cat-name";
pub const RESULT_FROM_ERR: &str = "result-from-err";
pub const RESULT_FROM_ERR_KIND: &str = "result-from-err-kind";
pub const ERROR_TRAIT: &str = "error-trait";

pub const ALL_KWS: [&str; 20] = [
    SPEC,
    MODULE,
    CATEGORY,
    ERROR,
    OUTPUT,
    NO_STD,
    DOC,
    DISPLAY,
    DOC_FROM_DISPLAY,
    VARIANT_TYPE,
    FLAT_KINDS,
    ERR_DOC,
    ERR_KIND_DOC,
    ERR_CAT_DOC,
    ERR_NAME,
    ERR_KIND_NAME,
    ERR_CAT_NAME,
    RESULT_FROM_ERR,
    RESULT_FROM_ERR_KIND,
    ERROR_TRAIT,
];

#[inline]
fn contains(arr: &[&str], s: &str) -> bool {
    arr.contains(&s)
}

#[inline]
pub fn is_any_kw(s: &str) -> bool {
    contains(&ALL_KWS, s)
}
