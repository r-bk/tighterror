use crate::{
    errors::{codes::BAD_SPEC_FILE_EXTENSION, TebError},
    util::open_spec_file,
    DEFAULT_SPEC_PATH,
};
use std::path::PathBuf;

cfg_if::cfg_if! {
    if #[cfg(feature = "yaml")] {
        mod yaml;
        pub(crate) use yaml::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        mod toml;
        pub(crate) use self::toml::*;
    }
}

/// A lint level.
///
/// Denotes how harmful a lint message is.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum LintLevel {
    /// definitive error
    Error,
    /// not an immediate error but may become one in a close release
    Warning,
    /// something to notice
    Notice,
}

/// A lint message.
#[derive(Debug)]
#[non_exhaustive]
pub struct LintMsg {
    /// The level of the message.
    pub level: LintLevel,
    /// The lint message itself.
    pub msg: String,
}

impl LintMsg {
    #[inline]
    pub(crate) fn error(msg: String) -> LintMsg {
        LintMsg {
            level: LintLevel::Error,
            msg,
        }
    }
}

/// A lint report.
///
/// A lint report represents a list of lint messages generated during
/// parsing or linting of a specification file.
#[derive(Debug, Default)]
pub struct LintReport {
    msgs: Vec<LintMsg>,
}

impl LintReport {
    /// Returns the list of lint messages.
    #[inline]
    pub fn messages(&self) -> &[LintMsg] {
        &self.msgs
    }

    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }

    #[inline]
    pub(crate) fn error(&mut self, msg: String) {
        self.msgs.push(LintMsg::error(msg));
    }

    #[inline]
    pub(crate) fn from_error(msg: String) -> Self {
        Self {
            msgs: vec![LintMsg::error(msg)],
        }
    }
}

/// Checks a specification file for errors.
pub fn lint(spec: Option<PathBuf>) -> Result<LintReport, TebError> {
    let path = match spec {
        Some(pb) => pb,
        None => DEFAULT_SPEC_PATH.into(),
    };

    match path.extension() {
        #[cfg(feature = "yaml")]
        Some(e) if e == "yaml" => YamlLinter::from_file(open_spec_file(&path)?),
        #[cfg(feature = "toml")]
        Some(e) if e == "toml" => TomlLinter::from_file(open_spec_file(&path)?),
        Some(e) => {
            log::error!(
                "specification file extension {:?} isn't supported: {:?}",
                e,
                path
            );
            BAD_SPEC_FILE_EXTENSION.into()
        }
        None => {
            log::error!(
                "specification file name must have a markup language extension: {:?}",
                path
            );
            BAD_SPEC_FILE_EXTENSION.into()
        }
    }
}
