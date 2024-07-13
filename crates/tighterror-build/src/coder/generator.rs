use crate::{
    coder::{formatter::pretty, FrozenOptions, ALL_MODULES},
    errors::TbError,
    spec::Spec,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod helpers;
mod module;
mod repr_type;

use module::ModuleGenerator;

#[derive(Debug)]
pub struct ModuleCode {
    /// The module name
    pub(crate) name: String,
    /// The module code
    pub(crate) code: String,
}

struct RustGenerator<'a> {
    opts: &'a FrozenOptions,
    spec: &'a Spec,
}

impl<'a> RustGenerator<'a> {
    fn new(opts: &'a FrozenOptions, spec: &'a Spec) -> RustGenerator<'a> {
        Self { opts, spec }
    }

    fn rust(&self) -> Result<Vec<ModuleCode>, TbError> {
        let mut ret = Vec::new();
        let mut ts = TokenStream::default();
        for m in &self.spec.modules {
            let mod_doc = self.opts.separate_files || self.spec.modules.len() == 1;
            let tokens = ModuleGenerator::new(self.opts, self.spec, m, mod_doc)?.rust()?;
            if self.spec.modules.len() > 1 && !self.opts.separate_files {
                let module_name = format_ident!("{}", m.name());
                let module_doc = helpers::doc_tokens(m.doc());
                ts = quote! {
                    #ts
                    #module_doc
                    pub mod #module_name {
                        #tokens
                    }
                };
            } else {
                ret.push(ModuleCode {
                    name: m.name().into(),
                    code: pretty(tokens)?,
                });
            }
        }
        if ret.is_empty() {
            let name = if self.spec.modules.len() > 1 {
                ALL_MODULES.to_owned()
            } else {
                self.spec
                    .modules
                    .first()
                    .expect("at least one module is expected to exist at this point")
                    .name()
                    .to_owned()
            };
            ret.push(ModuleCode {
                name,
                code: pretty(ts)?,
            });
        }
        Ok(ret)
    }
}

pub fn spec_to_rust(opts: &FrozenOptions, spec: &Spec) -> Result<Vec<ModuleCode>, TbError> {
    RustGenerator::new(opts, spec).rust()
}
