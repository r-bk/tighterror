pub const ERROR: &str = "Error";
pub const ERROR_CATEGORY: &str = "ErrorCategory";
pub const ERROR_KIND: &str = "ErrorKind";
pub const CATEGORY_NAMES_MOD: &str = "_cn";
pub const ERROR_NAMES_MOD: &str = "_n";
pub const ERROR_DISPLAYS_MOD: &str = "_d";
pub const PRIVATE_MOD: &str = "_p";
pub const CATEGORY_CONSTS_MOD: &str = "categories";
pub const ERROR_KINDS_MOD: &str = "kinds";
pub const TESTS_MOD: &str = "tests";

const TOP_LEVEL: [&str; 10] = [
    ERROR,
    ERROR_CATEGORY,
    ERROR_KIND,
    CATEGORY_NAMES_MOD,
    ERROR_NAMES_MOD,
    ERROR_DISPLAYS_MOD,
    PRIVATE_MOD,
    CATEGORY_CONSTS_MOD,
    ERROR_KINDS_MOD,
    TESTS_MOD,
];

pub fn is_top_level_ident(s: &str) -> bool {
    TOP_LEVEL.iter().any(|v| *v == s)
}
