use self::kdl::KdlParser;
use crate::{
    errors::{TbError, kind::*},
    spec::Spec,
};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

mod helpers;
mod kdl;
mod kws;
#[cfg(test)]
mod testing;

pub fn parse(spec: Option<&Path>) -> Result<Spec, TbError> {
    let path = spec_file_path(spec)?;
    let mut spec = parse_path(path.into())?;
    spec.path = path.into();
    Ok(spec)
}

fn parse_path(path: PathBuf) -> Result<Spec, TbError> {
    match path.extension() {
        Some(e) if e == "kdl" => KdlParser::parse_file(open_spec_file(&path)?, &path),
        Some(e) => {
            log::error!(
                "specification file extension {:?} isn't supported: {:?}",
                e,
                path
            );
            BAD_SPEC_FILE_EXTENSION.into()
        }
        None => {
            log::error!(
                "specification file name must have the `.kdl` extension: {:?}",
                path
            );
            BAD_SPEC_FILE_EXTENSION.into()
        }
    }
}

fn spec_file_path(spec: Option<&Path>) -> Result<&Path, TbError> {
    if let Some(p) = spec {
        return Ok(p);
    }

    if Path::new(crate::DEFAULT_SPEC_PATH_KDL).is_file() {
        return Ok(Path::new(crate::DEFAULT_SPEC_PATH_KDL));
    }

    SPEC_FILE_NOT_FOUND.into()
}

fn open_spec_file(path: &PathBuf) -> Result<File, TbError> {
    match File::options().read(true).open(path) {
        Ok(f) => Ok(f),
        Err(e) => {
            log::error!("Failed to open the spec file {:?}: {e}", path);
            FAILED_TO_OPEN_SPEC_FILE.into()
        }
    }
}
