pub const ERROR: &str = "Error";
pub const ERROR_CATEGORY: &str = "ErrorCategory";
pub const ERROR_KIND: &str = "ErrorKind";
pub const CATEGORY_NAMES_MOD: &str = "_cn";
pub const ERROR_NAMES_MOD: &str = "_n";
pub const ERROR_DISPLAYS_MOD: &str = "_d";
pub const PRIVATE_MOD: &str = "_p";
pub const CATEGORY_CONSTS_MOD: &str = "category";
pub const ERROR_KINDS_MOD: &str = "kind";
pub const VARIANTS_MOD: &str = "variant";
pub const TYPES_MOD: &str = "types"; // singular `type` is rust-reserved
pub const TESTS_MOD: &str = "test";

const ROOT_LEVEL: [&str; 11] = [
    ERROR,
    ERROR_CATEGORY,
    ERROR_KIND,
    CATEGORY_NAMES_MOD,
    ERROR_NAMES_MOD,
    ERROR_DISPLAYS_MOD,
    PRIVATE_MOD,
    CATEGORY_CONSTS_MOD,
    ERROR_KINDS_MOD,
    VARIANTS_MOD,
    TESTS_MOD,
];

pub fn is_root_level_ident(s: &str) -> bool {
    ROOT_LEVEL.iter().any(|v| *v == s)
}
