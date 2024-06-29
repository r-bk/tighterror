use crate::{
    coder::{
        generator::{doc_tokens, module::ModuleGenerator},
        ALL_MODULES,
    },
    errors::TbError,
    spec::{MainSpec, ModuleSpec},
    FrozenOptions,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct ModuleTokenStream {
    /// The module name
    pub name: String,
    /// The module tokens
    pub tokens: TokenStream,
}

pub struct ModulesGenerator<'a> {
    opts: &'a FrozenOptions,
    main: &'a MainSpec,
    modules: &'a [ModuleSpec],
}

impl<'a> ModulesGenerator<'a> {
    pub fn new(opts: &'a FrozenOptions, main: &'a MainSpec, modules: &'a [ModuleSpec]) -> Self {
        ModulesGenerator {
            opts,
            main,
            modules,
        }
    }

    pub fn rust(&self) -> Result<Vec<ModuleTokenStream>, TbError> {
        let mut ret = Vec::new();
        let mut ts = TokenStream::default();
        for m in self.modules {
            let mod_doc = self.opts.separate_files || self.modules.len() == 1;
            let mg = ModuleGenerator::new(self.opts, self.main, m, mod_doc)?;
            let tokens = mg.rust()?;
            if self.modules.len() > 1 && !self.opts.separate_files {
                let module_name = format_ident!("{}", m.name());
                let module_doc = doc_tokens(m.doc());
                ts = quote! {
                    #ts
                    #module_doc
                    pub mod #module_name {
                        #tokens
                    }
                };
            } else {
                ret.push(ModuleTokenStream {
                    name: m.name().into(),
                    tokens,
                });
            }
        }
        if ret.is_empty() {
            let name = if self.modules.len() > 1 {
                ALL_MODULES.to_owned()
            } else {
                self.modules
                    .first()
                    .expect("at least one module is expected to exist at this point")
                    .name()
                    .to_owned()
            };
            ret.push(ModuleTokenStream { name, tokens: ts });
        }
        Ok(ret)
    }
}
