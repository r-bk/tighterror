use crate::{
    errors::{
        TbError,
        kind::{FAILED_TO_READ_OUTPUT_FILE, FAILED_TO_WRITE_OUTPUT_FILE, OUT_DIR_NOT_SET},
    },
    parser,
    spec::definitions::STDOUT_PATH,
};
use log::error;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

mod build_options;
pub use build_options::*;

mod formatter;
mod frozen_options;
pub(crate) use frozen_options::*;
mod generator;
use generator::ModuleCode;
pub(crate) mod idents;
mod options;
pub use options::*;
#[cfg(test)]
mod test_golden;

const TMP_FILE_PFX: &str = "tighterror.";
const TMP_FILE_SFX: &str = ".rs";
const RUST_FILE_EXTENSION: &str = "rs";
const ALL_MODULES: &str = "*";
const OUT_DIR_VAR: &str = "OUT_DIR";

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
/// opts.spec("tighterror.kdl".to_owned());
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

/// Generates Rust source code in a build script with default options.
///
/// This is a shorthand for `BuildOptions::new().build()`.
/// See [BuildOptions::build] for details.
///
/// # Examples
/// ```no_run
/// // build.rs
/// fn main() -> Result<(), tighterror_build::errors::TbError> {
///     tighterror_build::build()
/// }
/// ```
pub fn build() -> Result<(), TbError> {
    BuildOptions::new().build()
}

fn build_script(opts: &BuildOptions) -> Result<(), TbError> {
    let spec = opts
        .codegen
        .spec
        .clone()
        .unwrap_or_else(|| crate::DEFAULT_SPEC_PATH_KDL.into());
    println!("cargo:rerun-if-changed={}", spec.display());

    let Some(out_dir) = env::var_os(OUT_DIR_VAR) else {
        error!("{OUT_DIR_VAR} isn't set; generate the code from a build script run by cargo");
        return OUT_DIR_NOT_SET.into();
    };

    opts.codegen.clone().spec(spec).output(out_dir).codegen()
}

fn write_modules(frozen: &FrozenOptions, modules: &[ModuleCode]) -> Result<(), TbError> {
    if frozen.separate_files {
        let dir = frozen.output.as_path();
        for m in modules {
            let mut path = dir.join(&m.name);
            path.set_extension(RUST_FILE_EXTENSION);
            write_code(&m.code, &path, frozen.rustfmt)?;
        }
    } else {
        debug_assert_eq!(modules.len(), 1);
        write_code(&modules[0].code, frozen.output.as_path(), frozen.rustfmt)?;
    }

    Ok(())
}

fn write_code(code: &str, path: &Path, rustfmt: bool) -> Result<(), TbError> {
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

    write_and_format(code, path, file, rustfmt)
}

fn write_and_format(code: &str, path: &Path, mut file: File, rustfmt: bool) -> Result<(), TbError> {
    if let Err(e) = file.write_all(code.as_bytes()) {
        error!("failed to write to the output file {:?}: {e}", path);
        return FAILED_TO_WRITE_OUTPUT_FILE.into();
    }
    file.flush().ok();
    drop(file);
    if rustfmt {
        formatter::rustfmt(path).ok();
    }
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
            update_module(&m.code, &path, frozen.rustfmt)?;
        }
    } else {
        debug_assert_eq!(modules.len(), 1);
        update_module(&modules[0].code, frozen.output.as_path(), frozen.rustfmt)?;
    }

    Ok(())
}

fn update_module(code: &str, path: &Path, rustfmt: bool) -> Result<(), TbError> {
    if !path.exists() {
        return write_code(code, path, rustfmt);
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

    write_and_format(code, &tmp_path, tmp_file, rustfmt)?;

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
