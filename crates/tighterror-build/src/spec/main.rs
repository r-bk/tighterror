use super::definitions::{DEFAULT_NO_STD, STDOUT_PATH};
use crate::errors::TbError;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MainSpec {
    /// Output file path: relative to the specification file, or an
    /// absolute path.
    pub output: Option<PathBuf>,
    /// Generate code for `no_std` environment
    pub no_std: Option<bool>,
    /// Write every module in a separate file
    pub separate_files: Option<bool>,
}

impl MainSpec {
    pub fn output(&self, spec_path: &Path, path: Option<&Path>) -> Result<PathBuf, TbError> {
        if let Some(p) = path {
            return Ok(p.to_owned());
        }
        match self.output {
            Some(ref o) if o.as_os_str() == STDOUT_PATH => Ok(STDOUT_PATH.into()),
            Some(ref o) => output_path(spec_path, o),
            None => Ok(STDOUT_PATH.into()),
        }
    }

    pub fn no_std(&self) -> bool {
        self.no_std.unwrap_or(DEFAULT_NO_STD)
    }
}

fn output_path(spec_path: &Path, output: &Path) -> Result<PathBuf, TbError> {
    if output.is_absolute() {
        return Ok(output.to_owned());
    }
    if let Some(sp) = spec_path.parent() {
        return Ok(sp.join(output));
    }
    Ok(output.to_owned())
}
