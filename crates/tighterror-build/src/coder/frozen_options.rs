use crate::{
    coder::CodegenOptions,
    errors::{kinds::coder::*, TbError},
    spec::{definitions::*, Spec},
};
use std::path::Path;

#[derive(Debug)]
pub struct FrozenOptions {
    pub(crate) output: String,
    pub(crate) test: bool,
    pub(crate) update: bool,
    pub(crate) separate_files: bool,
}

impl FrozenOptions {
    pub fn new(opts: &CodegenOptions, spec: &Spec) -> Result<Self, TbError> {
        let op = Self::output_path(opts, spec)?;
        Ok(Self {
            output: op.path,
            test: opts.test.unwrap_or(DEFAULT_TEST),
            update: opts.update.unwrap_or(DEFAULT_UPDATE_MODE),
            separate_files: op.separate_files,
        })
    }

    fn output_path(opts: &CodegenOptions, spec: &Spec) -> Result<OutputPath, TbError> {
        let output = spec.main.output(&spec.path, opts.output.as_deref())?;
        if output == STDOUT_PATH {
            return Ok(OutputPath {
                path: output,
                separate_files: false,
            });
        }

        let separate_files = opts.separate_files.unwrap_or(DEFAULT_SEPARATE_FILES);
        let path = Path::new(output.as_str());
        let is_dir = path.is_dir();

        if separate_files && !is_dir {
            log::error!("output path must be a directory in separate-files mode: {output}");
            return OUTPUT_PATH_NOT_DIRECTORY.into();
        }

        let op = if is_dir && !separate_files {
            path.join(IMPLICIT_FILENAME)
                .as_os_str()
                .to_str()
                .map(|s| s.to_owned())
                .ok_or_else(|| {
                    log::error!(
                        "failed to build implicit specification file path: output={output}"
                    );
                    TbError::from(BAD_PATH)
                })?
        } else {
            output
        };

        Ok(OutputPath {
            path: op,
            separate_files,
        })
    }
}

#[derive(Debug)]
struct OutputPath {
    path: String,
    separate_files: bool,
}
