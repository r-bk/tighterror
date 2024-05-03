use super::definitions::{DEFAULT_NO_STD, STDOUT_DST};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MainSpec {
    /// Output file path: relative to the specification file, or an
    /// absolute path.
    pub output: Option<String>,
    /// Generate code for `no_std` environment
    pub no_std: Option<bool>,
}

impl MainSpec {
    pub fn output<'a>(&'a self, path: Option<&'a str>) -> &'a str {
        path.or(self.output.as_deref()).unwrap_or(STDOUT_DST)
    }

    pub fn no_std(&self) -> bool {
        self.no_std.unwrap_or(DEFAULT_NO_STD)
    }
}
