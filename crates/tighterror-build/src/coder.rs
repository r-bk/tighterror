use crate::{
    errors::{
        kinds::{FAILED_TO_READ_DST_FILE, FAILED_TO_WRITE_DST_FILE, SPEC_FILE_NOT_FOUND},
        TebError,
    },
    parser,
    spec::{DEFAULT_UPDATE_MODE, STDOUT_DST},
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

fn spec_file_path(opts: &CodegenOptions) -> Result<&str, TebError> {
    if let Some(ref p) = opts.spec {
        return Ok(p.as_str());
    }

    #[cfg(feature = "yaml")]
    if Path::new(crate::DEFAULT_SPEC_PATH_YAML).is_file() {
        return Ok(crate::DEFAULT_SPEC_PATH_YAML);
    }

    #[cfg(feature = "toml")]
    if Path::new(crate::DEFAULT_SPEC_PATH_TOML).is_file() {
        return Ok(crate::DEFAULT_SPEC_PATH_TOML);
    }

    SPEC_FILE_NOT_FOUND.into()
}

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
    let path = spec_file_path(opts)?;
    let spec = parser::from_path(path.into())?;
    let code = generator::spec_to_rust(opts, &spec)?;

    match spec.dst(opts.dst.as_deref()) {
        p if p == STDOUT_DST => {
            if let Err(e) = io::stdout().lock().write_all(code.as_bytes()) {
                error!("failed to write to stdout: {e}");
                FAILED_TO_WRITE_DST_FILE.into()
            } else {
                Ok(())
            }
        }
        p => {
            if opts.update.unwrap_or(DEFAULT_UPDATE_MODE) {
                update_code(code, p)
            } else {
                write_code(code, p)
            }
        }
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
            return FAILED_TO_WRITE_DST_FILE.into();
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
        return FAILED_TO_WRITE_DST_FILE.into();
    }
    file.flush().ok();
    drop(file);
    formatter::rustfmt(path).ok();
    Ok(())
}

fn read_code<P>(path: P) -> Result<String, TebError>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut file = match File::options().read(true).open(&path) {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open destination file {:?}: {e}", path);
            return FAILED_TO_READ_DST_FILE.into();
        }
    };

    let mut data = String::with_capacity(4096);
    file.read_to_string(&mut data).map_err(|e| {
        error!("failed to read destination file {:?}: {e}", path);
        TebError::from(FAILED_TO_WRITE_DST_FILE)
    })?;

    Ok(data)
}

fn update_code(code: String, path: &str) -> Result<(), TebError> {
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
            TebError::from(FAILED_TO_WRITE_DST_FILE)
        })?;

    let (tmp_file, tmp_path) = tmp_file.keep().map_err(|e| {
        error!("failed to keep the temporary file: {e}");
        TebError::from(FAILED_TO_WRITE_DST_FILE)
    })?;

    write_and_format(code, &tmp_path, tmp_file)?;

    let new_data = read_code(&tmp_path)?;

    if existing_data != new_data {
        std::fs::rename(&tmp_path, path).map_err(|e| {
            error!(
                "failed to rename updated file {:?} to destination path {:?}: {e}",
                tmp_path, path
            );
            TebError::from(FAILED_TO_WRITE_DST_FILE)
        })
    } else {
        std::fs::remove_file(&tmp_path).map_err(|e| {
            error!("failed to unlink temporary file {:?}: {e}", tmp_path);
            TebError::from(FAILED_TO_WRITE_DST_FILE)
        })
    }
}
