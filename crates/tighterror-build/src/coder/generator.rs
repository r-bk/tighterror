use crate::{
    coder::{formatter::pretty, FrozenOptions},
    errors::TbError,
    spec::Spec,
};

mod helpers;
mod module;
mod modules;
mod repr_type;
use modules::ModulesGenerator;

#[derive(Debug)]
pub struct ModuleCode {
    /// The module name
    pub(crate) name: String,
    /// The module code
    pub(crate) code: String,
}

#[allow(dead_code)]
struct RustGenerator<'a> {
    opts: &'a FrozenOptions,
    spec: &'a Spec,
}

impl<'a> RustGenerator<'a> {
    fn new(opts: &'a FrozenOptions, spec: &'a Spec) -> RustGenerator<'a> {
        Self { opts, spec }
    }

    fn rust(&self) -> Result<Vec<ModuleCode>, TbError> {
        let mg = ModulesGenerator::new(self.opts, &self.spec.main, &self.spec.modules);
        let tokens = mg.rust()?;
        let mut ret = Vec::new();
        for mt in tokens.into_iter() {
            ret.push(ModuleCode {
                name: mt.name,
                code: pretty(mt.tokens)?,
            });
        }
        Ok(ret)
    }
}

pub fn spec_to_rust(opts: &FrozenOptions, spec: &Spec) -> Result<Vec<ModuleCode>, TbError> {
    RustGenerator::new(opts, spec).rust()
}
