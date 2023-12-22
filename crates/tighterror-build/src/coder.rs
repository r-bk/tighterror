use crate::{
    errors::{TebError, FAILED_TO_WRITE_TO_DST_FILE},
    parser,
    spec::STDOUT_DST,
    DEFAULT_SPEC_PATH,
};
use log::error;
use std::{
    fs::File,
    io::{self, Write},
};

mod formatter;
mod generator;
mod options;
pub use options::*;

/// Generates code from a specification file.
///
/// See [CodegenOptions] for the information about function parameters.
///
/// # Examples
///
/// This example shows how the [codegen] function may be called directly.
/// However, the recommended and a shorter way to invoke the function
/// is using [CodegenOptions::codegen] method. See [CodegenOptions] for
/// a full example.
///
/// ```no_run
/// # use tighterror_build::{CodegenOptions, errors::TebError, codegen};
/// # pub fn foo() -> Result<(), TebError> {
/// let mut opts = CodegenOptions::new();
/// opts.spec("tighterror.yaml".to_owned());
/// codegen(&opts)?;
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
pub fn codegen(opts: &CodegenOptions) -> Result<(), TebError> {
    let path = opts.spec.as_deref().unwrap_or(DEFAULT_SPEC_PATH);
    let spec = parser::from_path(path.into())?;
    let code = generator::spec2code(opts, &spec)?;

    match spec.dst(opts.dst.as_deref()) {
        p if p == STDOUT_DST => {
            if let Err(e) = io::stdout().lock().write_all(code.as_bytes()) {
                error!("failed to write to stdout: {e}");
                return FAILED_TO_WRITE_TO_DST_FILE.into();
            }
        }
        p => {
            let mut file = match File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(p)
            {
                Ok(f) => f,
                Err(e) => {
                    error!("failed to open the destination file {:?}: {e}", p);
                    return FAILED_TO_WRITE_TO_DST_FILE.into();
                }
            };
            if let Err(e) = file.write_all(code.as_bytes()) {
                error!("failed to write to the destination file {:?}: {e}", p);
                return FAILED_TO_WRITE_TO_DST_FILE.into();
            }
            file.flush().ok();
            drop(file);
            formatter::rustfmt(p).ok();
        }
    }

    Ok(())
}
