use crate::{
    coder::generator::{doc_tokens, module::ModuleGenerator},
    errors::TbError,
    spec::{MainSpec, ModuleSpec},
    FrozenOptions,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

    pub fn rust(&self) -> Result<TokenStream, TbError> {
        let mut ts = TokenStream::default();
        for m in self.modules {
            let mod_doc = self.modules.len() == 1;
            let mg = ModuleGenerator::new(self.opts, self.main, m, mod_doc)?;
            let tokens = mg.rust()?;
            if self.modules.len() > 1 {
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
                ts = tokens;
            }
        }
        Ok(ts)
    }
}
