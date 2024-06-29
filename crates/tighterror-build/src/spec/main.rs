use super::definitions::{DEFAULT_NO_STD, STDOUT_PATH};
use crate::errors::{kinds::coder::BAD_PATH, TbError};
use std::path::Path;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MainSpec {
    /// Output file path: relative to the specification file, or an
    /// absolute path.
    pub output: Option<String>,
    /// Generate code for `no_std` environment
    pub no_std: Option<bool>,
    /// Write every module in a separate file
    pub separate_files: Option<bool>,
}

impl MainSpec {
    pub fn output(&self, spec_path: &str, path: Option<&str>) -> Result<String, TbError> {
        if let Some(p) = path {
            return Ok(p.to_owned());
        }
        match self.output {
            Some(ref o) if o == STDOUT_PATH => Ok(STDOUT_PATH.into()),
            Some(ref o) => output_path(spec_path, o),
            None => Ok(STDOUT_PATH.into()),
        }
    }

    pub fn no_std(&self) -> bool {
        self.no_std.unwrap_or(DEFAULT_NO_STD)
    }
}

fn output_path(spec_path: &str, output: &str) -> Result<String, TbError> {
    let op = Path::new(output);
    if op.is_absolute() {
        return Ok(output.to_owned());
    }
    if let Some(sp) = Path::new(spec_path).parent() {
        return sp
            .join(output)
            .as_os_str()
            .to_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| {
                log::error!("failed to build specification file relative path: spec={spec_path}, output={output}");
                BAD_PATH.into()
            });
    }
    Ok(output.to_owned())
}
