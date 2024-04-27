#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MainSpec {
    /// Output file path: relative to the specification file, or an
    /// absolute path.
    pub output: Option<String>,
    /// Generate code for `no_std` environment
    pub no_std: Option<bool>,
}
