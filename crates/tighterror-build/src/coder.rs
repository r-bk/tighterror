use crate::{
    errors::{kinds::FAILED_TO_WRITE_TO_DST_FILE, TebError},
    parser,
    spec::STDOUT_DST,
    DEFAULT_SPEC_PATH,
};
use log::error;
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, Write},
    path::Path,
};

mod formatter;
mod generator;
pub(crate) mod idents;
mod options;
pub use options::*;

/// Generates Rust source code from a specification file.
///
/// See [CodegenOptions] for more information about function parameters.
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
    let code = generator::spec_to_rust(opts, &spec)?;

    match spec.dst(opts.dst.as_deref()) {
        p if p == STDOUT_DST => {
            if let Err(e) = io::stdout().lock().write_all(code.as_bytes()) {
                error!("failed to write to stdout: {e}");
                FAILED_TO_WRITE_TO_DST_FILE.into()
            } else {
                Ok(())
            }
        }
        p => write_code(code, p),
    }
}

fn write_code<P>(code: String, path: P) -> Result<(), TebError>
where
    P: AsRef<Path> + AsRef<OsStr> + std::fmt::Debug,
{
    let file = match File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
    {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open the destination file {:?}: {e}", path);
            return FAILED_TO_WRITE_TO_DST_FILE.into();
        }
    };

    write_and_format(code, path, file)
}

fn write_and_format<P>(code: String, path: P, mut file: File) -> Result<(), TebError>
where
    P: AsRef<OsStr> + std::fmt::Debug,
{
    if let Err(e) = file.write_all(code.as_bytes()) {
        error!("failed to write to the destination file {:?}: {e}", path);
        return FAILED_TO_WRITE_TO_DST_FILE.into();
    }
    file.flush().ok();
    drop(file);
    formatter::rustfmt(path).ok();
    Ok(())
}
