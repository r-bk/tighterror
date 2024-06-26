use crate::coder::idents;
use std::path::PathBuf;

mod error;
pub use error::*;

mod category;
pub use category::*;

pub mod definitions;

mod main;
pub use main::*;

mod module;
pub use module::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Spec {
    /// The specification file path
    pub path: PathBuf,
    /// The main spec
    pub main: MainSpec,
    /// A list of tighterror module specs
    pub modules: Vec<ModuleSpec>,
}
