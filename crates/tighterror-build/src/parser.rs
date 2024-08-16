use crate::{
    errors::{kind::parser::*, TbError},
    spec::Spec,
};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[cfg(not(any(feature = "yaml", feature = "toml")))]
compile_error!("At least one of the markup language features ['yaml', 'toml'] must be enabled.");

cfg_if::cfg_if! {
    if #[cfg(feature = "yaml")] {
        mod yaml;
        use yaml::YamlParser;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        mod toml;
        use self::toml::TomlParser;
    }
}

mod helpers;
mod kws;
#[cfg(test)]
mod testing;

#[derive(Debug)]
pub enum ParseMode {
    Single,
    List,
}

pub fn parse(spec: Option<&Path>) -> Result<Spec, TbError> {
    let path = spec_file_path(spec)?;
    let mut spec = parse_path(path.into())?;
    spec.path = path.into();
    Ok(spec)
}

fn parse_path(path: PathBuf) -> Result<Spec, TbError> {
    match path.extension() {
        #[cfg(feature = "yaml")]
        Some(e) if e == "yaml" => YamlParser::parse_file(open_spec_file(&path)?),
        #[cfg(feature = "toml")]
        Some(e) if e == "toml" => TomlParser::parse_file(open_spec_file(&path)?),
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
                "specification file name must have a markup language extension: {:?}",
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

    #[cfg(feature = "yaml")]
    if Path::new(crate::DEFAULT_SPEC_PATH_YAML).is_file() {
        return Ok(Path::new(crate::DEFAULT_SPEC_PATH_YAML));
    }

    #[cfg(feature = "toml")]
    if Path::new(crate::DEFAULT_SPEC_PATH_TOML).is_file() {
        return Ok(Path::new(crate::DEFAULT_SPEC_PATH_TOML));
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
