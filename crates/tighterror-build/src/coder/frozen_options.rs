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
}

impl FrozenOptions {
    pub fn new(opts: &CodegenOptions, spec: &Spec) -> Result<Self, TbError> {
        Ok(Self {
            output: Self::output_path(opts, spec)?,
            test: opts.test.unwrap_or(DEFAULT_TEST),
            update: opts.update.unwrap_or(DEFAULT_UPDATE_MODE),
        })
    }

    fn output_path(opts: &CodegenOptions, spec: &Spec) -> Result<String, TbError> {
        let op = spec.main.output(&spec.path, opts.output.as_deref())?;
        if op == STDOUT_PATH {
            return Ok(op);
        }
        let op_path = Path::new(op.as_str());
        if op_path.is_dir() {
            return op_path
                .join(IMPLICIT_FILENAME)
                .as_os_str()
                .to_str()
                .map(|s| s.to_owned())
                .ok_or_else(|| {
                    log::error!("failed to build implicit specification file path: output={op}");
                    BAD_PATH.into()
                });
        }

        Ok(op)
    }
}
