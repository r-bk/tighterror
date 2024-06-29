use crate::{
    coder::CodegenOptions,
    errors::{kinds::coder::OUTPUT_PATH_NOT_DIRECTORY, TbError},
    spec::{definitions::*, Spec},
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FrozenOptions {
    pub(crate) output: PathBuf,
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
        if output.as_os_str() == STDOUT_PATH {
            return Ok(OutputPath {
                path: output,
                separate_files: false,
            });
        }

        let separate_files = opts.separate_files.unwrap_or(DEFAULT_SEPARATE_FILES);
        let is_dir = output.is_dir();

        if separate_files && !is_dir {
            log::error!("output path must be a directory in separate-files mode: {output:?}");
            return OUTPUT_PATH_NOT_DIRECTORY.into();
        }

        let op = if is_dir && !separate_files {
            output.join(IMPLICIT_FILENAME)
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
    path: PathBuf,
    separate_files: bool,
}
