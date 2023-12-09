use crate::{
    errors::{TebError, BAD_SPEC_FILE_EXTENSION},
    spec::Spec,
    util::open_spec_file,
};
use std::path::PathBuf;

#[cfg(not(any(feature = "yaml", feature = "toml")))]
compile_error!("At least one of the markup language features ['yaml', 'toml'] must be enabled.");

cfg_if::cfg_if! {
    if #[cfg(feature = "yaml")] {
        mod yaml;
        pub(crate) use yaml::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        mod toml;
        pub(crate) use self::toml::*;
    }
}

mod kws;

pub fn from_path(path: PathBuf) -> Result<Spec, TebError> {
    match path.extension() {
        #[cfg(feature = "yaml")]
        Some(e) if e == "yaml" => YamlParser::from_file(open_spec_file(&path)?),
        #[cfg(feature = "toml")]
        Some(e) if e == "toml" => TomlParser::from_file(open_spec_file(&path)?),
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
