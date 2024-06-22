use crate::{
    errors::{
        kinds::coder::{BAD_PATH, FAILED_TO_READ_OUTPUT_FILE, FAILED_TO_WRITE_OUTPUT_FILE},
        TbError,
    },
    parser,
    spec::{
        definitions::{DEFAULT_UPDATE_MODE, IMPLICIT_FILENAME, STDOUT_PATH},
        Spec,
    },
};
use log::error;
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

mod formatter;
mod generator;
pub(crate) mod idents;
mod options;
pub use options::*;

const TMP_FILE_PFX: &str = "tighterror.";
const TMP_FILE_SFX: &str = ".rs";

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
/// # use tighterror_build::{CodegenOptions, errors::TbError, codegen};
/// # pub fn foo() -> Result<(), TbError> {
/// let mut opts = CodegenOptions::new();
/// opts.spec("tighterror.yaml".to_owned());
/// codegen(&opts)?;
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
pub fn codegen(opts: &CodegenOptions) -> Result<(), TbError> {
    let spec = parser::parse(opts.spec.as_deref())?;
    let code = generator::spec_to_rust(opts, &spec)?;

    match output_path(opts, &spec)? {
        p if p == STDOUT_PATH => {
            if let Err(e) = io::stdout().lock().write_all(code.as_bytes()) {
                error!("failed to write to stdout: {e}");
                FAILED_TO_WRITE_OUTPUT_FILE.into()
            } else {
                Ok(())
            }
        }
        p => {
            if opts.update.unwrap_or(DEFAULT_UPDATE_MODE) {
                update_code(code, &p)
            } else {
                write_code(code, &p)
            }
        }
    }
}

fn output_path(opts: &CodegenOptions, spec: &Spec) -> Result<String, TbError> {
    let op = spec.main.output(&spec.path, opts.output.as_deref())?;
    if op == STDOUT_PATH {
        return Ok(op);
    }
    let op_path = Path::new(op.as_str());
    if let Ok(md) = std::fs::metadata(op_path) {
        if md.is_dir() {
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
    }
    Ok(op)
}

fn write_code<P>(code: String, path: P) -> Result<(), TbError>
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
            error!("failed to open the output file {:?}: {e}", path);
            return FAILED_TO_WRITE_OUTPUT_FILE.into();
        }
    };

    write_and_format(code, path, file)
}

fn write_and_format<P>(code: String, path: P, mut file: File) -> Result<(), TbError>
where
    P: AsRef<OsStr> + std::fmt::Debug,
{
    if let Err(e) = file.write_all(code.as_bytes()) {
        error!("failed to write to the output file {:?}: {e}", path);
        return FAILED_TO_WRITE_OUTPUT_FILE.into();
    }
    file.flush().ok();
    drop(file);
    formatter::rustfmt(path).ok();
    Ok(())
}

fn read_code<P>(path: P) -> Result<String, TbError>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut file = match File::options().read(true).open(&path) {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open the output file {:?}: {e}", path);
            return FAILED_TO_READ_OUTPUT_FILE.into();
        }
    };

    let mut data = String::with_capacity(4096);
    file.read_to_string(&mut data).map_err(|e| {
        error!("failed to read the output file {:?}: {e}", path);
        TbError::from(FAILED_TO_WRITE_OUTPUT_FILE)
    })?;

    Ok(data)
}

fn update_code(code: String, path: &str) -> Result<(), TbError> {
    let path = Path::new(path);
    if !path.exists() {
        return write_code(code, path);
    }

    let existing_data = read_code(path)?;

    let dir = match path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => Path::new("."),
    };

    let tmp_file = tempfile::Builder::new()
        .prefix(TMP_FILE_PFX)
        .suffix(TMP_FILE_SFX)
        .tempfile_in(dir)
        .map_err(|e| {
            error!("failed to create a temporary file [dir={:?}]: {e}", dir);
            TbError::from(FAILED_TO_WRITE_OUTPUT_FILE)
        })?;

    let (tmp_file, tmp_path) = tmp_file.keep().map_err(|e| {
        error!("failed to keep the temporary file: {e}");
        TbError::from(FAILED_TO_WRITE_OUTPUT_FILE)
    })?;

    write_and_format(code, &tmp_path, tmp_file)?;

    let new_data = read_code(&tmp_path)?;

    if existing_data != new_data {
        std::fs::rename(&tmp_path, path).map_err(|e| {
            error!(
                "failed to rename updated file {:?} to output file path {:?}: {e}",
                tmp_path, path
            );
            TbError::from(FAILED_TO_WRITE_OUTPUT_FILE)
        })
    } else {
        std::fs::remove_file(&tmp_path).map_err(|e| {
            error!("failed to unlink temporary file {:?}: {e}", tmp_path);
            TbError::from(FAILED_TO_WRITE_OUTPUT_FILE)
        })
    }
}
