use crate::{
    errors::{
        kinds::coder::{FAILED_TO_READ_OUTPUT_FILE, FAILED_TO_WRITE_OUTPUT_FILE},
        TbError,
    },
    parser,
    spec::definitions::STDOUT_PATH,
};
use log::error;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

mod formatter;
mod frozen_options;
pub(crate) use frozen_options::*;
mod generator;
use generator::ModuleCode;
pub(crate) mod idents;
mod options;
pub use options::*;

const TMP_FILE_PFX: &str = "tighterror.";
const TMP_FILE_SFX: &str = ".rs";
const RUST_FILE_EXTENSION: &str = "rs";
const ALL_MODULES: &str = "*";

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
    debug_assert!(!spec.modules.is_empty());

    let frozen = FrozenOptions::new(opts, &spec)?;
    let modules = generator::spec_to_rust(&frozen, &spec)?;

    match frozen.output {
        p if p.as_os_str() == STDOUT_PATH => {
            debug_assert_eq!(modules.len(), 1);
            let code = modules[0].code.as_bytes();
            if let Err(e) = io::stdout().lock().write_all(code) {
                error!("failed to write to stdout: {e}");
                FAILED_TO_WRITE_OUTPUT_FILE.into()
            } else {
                Ok(())
            }
        }
        _ if frozen.update => update_modules(&frozen, &modules),
        _ => write_modules(&frozen, &modules),
    }
}

fn write_modules(frozen: &FrozenOptions, modules: &[ModuleCode]) -> Result<(), TbError> {
    if frozen.separate_files {
        let dir = frozen.output.as_path();
        for m in modules {
            let mut path = dir.join(&m.name);
            path.set_extension(RUST_FILE_EXTENSION);
            write_code(&m.code, &path)?;
        }
    } else {
        debug_assert_eq!(modules.len(), 1);
        write_code(&modules[0].code, frozen.output.as_path())?;
    }

    Ok(())
}

fn write_code(code: &str, path: &Path) -> Result<(), TbError> {
    let file = match File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open the output file {:?}: {e}", path);
            return FAILED_TO_WRITE_OUTPUT_FILE.into();
        }
    };

    write_and_format(code, path, file)
}

fn write_and_format(code: &str, path: &Path, mut file: File) -> Result<(), TbError> {
    if let Err(e) = file.write_all(code.as_bytes()) {
        error!("failed to write to the output file {:?}: {e}", path);
        return FAILED_TO_WRITE_OUTPUT_FILE.into();
    }
    file.flush().ok();
    drop(file);
    formatter::rustfmt(path).ok();
    Ok(())
}

fn read_code(path: &Path) -> Result<String, TbError> {
    let mut file = match File::options().read(true).open(path) {
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

fn update_modules(frozen: &FrozenOptions, modules: &[ModuleCode]) -> Result<(), TbError> {
    if frozen.separate_files {
        let dir = frozen.output.as_path();
        for m in modules {
            let mut path = dir.join(&m.name);
            path.set_extension(RUST_FILE_EXTENSION);
            update_module(&m.code, &path)?;
        }
    } else {
        debug_assert_eq!(modules.len(), 1);
        update_module(&modules[0].code, frozen.output.as_path())?;
    }

    Ok(())
}

fn update_module(code: &str, path: &Path) -> Result<(), TbError> {
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
