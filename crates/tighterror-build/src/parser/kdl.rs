use crate::{
    errors::{TbError, kind::*},
    parser::{helpers::*, kws::*},
    spec::{CategorySpec, ErrorSpec, MainSpec, ModuleSpec, ReprType, Spec, WrappedSpec},
};
use ::kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use log::error;
use std::{collections::HashSet, fs::File, io::Read, path::Path};

#[derive(Debug)]
pub struct KdlParser<'a> {
    path: &'a str,
    src: &'a str,
}

impl<'a> KdlParser<'a> {
    pub fn parse_file(mut file: File, path: &Path) -> Result<Spec, TbError> {
        let mut src = String::new();
        if let Err(e) = file.read_to_string(&mut src) {
            error!("failed to read the specification file {path:?}: {e}");
            return BAD_KDL.into();
        }
        Self::parse_str_at(&src, &path.display().to_string())
    }

    #[cfg(test)]
    pub fn parse_str(src: &str) -> Result<Spec, TbError> {
        Self::parse_str_at(src, "<string>")
    }

    fn parse_str_at(src: &str, path: &str) -> Result<Spec, TbError> {
        let parser = KdlParser { path, src };
        match src.parse::<KdlDocument>() {
            Ok(doc) => parser.document(&doc),
            Err(e) => {
                for d in &e.diagnostics {
                    let what = d
                        .message
                        .as_deref()
                        .or(d.label.as_deref())
                        .unwrap_or("invalid KDL");
                    error!("{}: {what}", parser.pos(d.span.offset()));
                    if let Some(help) = d.help.as_deref() {
                        error!("{}: help: {help}", parser.pos(d.span.offset()));
                    }
                }
                if e.diagnostics.is_empty() {
                    error!("{path}: failed to parse KDL");
                }
                BAD_KDL.into()
            }
        }
    }

    fn pos(&self, offset: usize) -> String {
        let mut end = offset.min(self.src.len());
        while !self.src.is_char_boundary(end) {
            end -= 1;
        }
        let before = &self.src[..end];
        let line = before.matches('\n').count() + 1;
        let col = before.chars().rev().take_while(|c| *c != '\n').count() + 1;
        format!("{}:{line}:{col}", self.path)
    }

    fn node_pos(&self, n: &KdlNode) -> String {
        self.pos(n.span().offset())
    }

    fn entry_pos(&self, e: &KdlEntry) -> String {
        self.pos(e.span().offset())
    }

    fn at<T>(&self, n: &KdlNode, r: Result<T, TbError>) -> Result<T, TbError> {
        r.inspect_err(|_| error!("{}: in this `{}` node", self.node_pos(n), n.name().value()))
    }

    fn document(&self, doc: &KdlDocument) -> Result<Spec, TbError> {
        let mut spec = Spec::default();

        for (i, n) in doc.nodes().iter().enumerate() {
            self.check_no_type(n)?;
            match n.name().value() {
                SPEC if i == 0 => spec.main = self.root_spec(n)?,
                SPEC => {
                    error!(
                        "{}: the root `spec` must come first, and only once",
                        self.node_pos(n)
                    );
                    return BAD_ROOT_LEVEL_KEYWORD.into();
                }
                MODULE => spec.modules.push(self.module(n)?),
                other => {
                    error!(
                        "{}: the root holds `spec` and `module` nodes, not `{other}`",
                        self.node_pos(n)
                    );
                    return BAD_ROOT_LEVEL_KEYWORD.into();
                }
            }
        }

        if spec.modules.is_empty() {
            error!("{}: a specification needs at least one `module`", self.path);
            return MISSING_ATTRIBUTE.into();
        }
        check_module_name_uniqueness(spec.modules.iter().map(|m| m.name()))?;
        for m in &spec.modules {
            check_module(m)?;
        }

        Ok(spec)
    }

    fn root_spec(&self, n: &KdlNode) -> Result<MainSpec, TbError> {
        let mut main = MainSpec::default();
        for (key, sn, v) in self.settings(n)? {
            match key {
                OUTPUT => main.output = Some(self.string(key, sn, v)?.into()),
                NO_STD => main.no_std = Some(self.boolean(key, sn, v)?),
                _ => return self.unknown_setting("root", key, sn),
            }
        }
        Ok(main)
    }

    fn module(&self, n: &KdlNode) -> Result<ModuleSpec, TbError> {
        let (name, _) = self.name_arg(n, MODULE, &[])?;
        self.at(n, check_module_name(&name))?;
        let mut m = ModuleSpec {
            name,
            ..Default::default()
        };

        let (spec, children) = self.split_children(n, MODULE)?;
        if let Some(spec) = spec {
            for (key, sn, v) in self.settings(spec)? {
                match key {
                    DOC => m.doc = Some(self.string(key, sn, v)?),
                    ERR_DOC => m.err_doc = Some(self.string(key, sn, v)?),
                    ERR_KIND_DOC => m.err_kind_doc = Some(self.string(key, sn, v)?),
                    ERR_CAT_DOC => m.err_cat_doc = Some(self.string(key, sn, v)?),
                    ERR_NAME => m.err_name = Some(self.ident(key, sn, v)?),
                    ERR_KIND_NAME => m.err_kind_name = Some(self.ident(key, sn, v)?),
                    ERR_CAT_NAME => m.err_cat_name = Some(self.ident(key, sn, v)?),
                    RESULT_FROM_ERR => m.result_from_err = Some(self.boolean(key, sn, v)?),
                    RESULT_FROM_ERR_KIND => {
                        m.result_from_err_kind = Some(self.boolean(key, sn, v)?)
                    }
                    ERROR_TRAIT => m.error_trait = Some(self.boolean(key, sn, v)?),
                    FLAT_KINDS => m.flat_kinds = Some(self.boolean(key, sn, v)?),
                    DOC_FROM_DISPLAY => m.oes.doc_from_display = Some(self.boolean(key, sn, v)?),
                    VARIANT_TYPE => m.oes.variant_type = Some(self.boolean(key, sn, v)?),
                    REPR => m.repr = Some(self.repr(key, sn, v)?),
                    _ => return self.unknown_setting(MODULE, key, sn),
                }
            }
        }

        for c in children {
            if c.name().value() != CATEGORY {
                error!(
                    "{}: a module holds `spec` and `category` nodes, not `{}`",
                    self.node_pos(c),
                    c.name().value()
                );
                return BAD_OBJECT_ATTRIBUTE.into();
            }
            m.categories.push(self.category(c)?);
        }

        if m.categories.is_empty() {
            error!(
                "{}: module `{}` has no categories",
                self.node_pos(n),
                m.name()
            );
            return EMPTY_LIST.into();
        }
        if let [c] = m.categories.as_slice()
            && c.wrapped.is_some()
        {
            error!(
                "{}: module `{}` wraps another error in its only category `{}`",
                self.node_pos(n),
                m.name(),
                c.name
            );
            return BAD_OBJECT_ATTRIBUTE.into();
        }
        self.at(
            n,
            check_category_name_uniqueness(m.categories.iter().map(|c| c.name.as_str())),
        )?;
        let wrapped = || m.categories.iter().filter_map(|c| c.wrapped.as_ref());
        self.at(
            n,
            check_wrapped_type_uniqueness(wrapped().map(|w| w.error_type.as_str())),
        )?;
        self.at(
            n,
            check_wrapped_type_uniqueness(wrapped().map(|w| w.kind_type.as_str())),
        )?;

        Ok(m)
    }

    fn category(&self, n: &KdlNode) -> Result<CategorySpec, TbError> {
        let (name, _) = self.name_arg(n, CATEGORY, &[])?;
        self.at(n, check_category_name(&name))?;
        let mut c = CategorySpec {
            name,
            ..Default::default()
        };

        let (spec, children) = self.split_children(n, CATEGORY)?;
        let mut error_type = None;
        let mut kind_type = None;
        if let Some(spec) = spec {
            for (key, sn, v) in self.settings(spec)? {
                match key {
                    DOC => c.doc = Some(self.string(key, sn, v)?),
                    DOC_FROM_DISPLAY => c.oes.doc_from_display = Some(self.boolean(key, sn, v)?),
                    VARIANT_TYPE => c.oes.variant_type = Some(self.boolean(key, sn, v)?),
                    WRAP_ERROR_TYPE => error_type = Some(self.type_path(key, sn, v)?),
                    WRAP_KIND_TYPE => kind_type = Some(self.type_path(key, sn, v)?),
                    _ => return self.unknown_setting(CATEGORY, key, sn),
                }
            }
        }

        c.wrapped = match (error_type, kind_type) {
            (Some(error_type), Some(kind_type)) => Some(WrappedSpec {
                error_type,
                kind_type,
            }),
            (None, None) => None,
            (Some(_), None) | (None, Some(_)) => {
                error!(
                    "{}: category `{}` sets one of `{WRAP_ERROR_TYPE}` and `{WRAP_KIND_TYPE}` without the other",
                    self.node_pos(n),
                    c.name
                );
                return MISSING_ATTRIBUTE.into();
            }
        };
        if c.wrapped.is_some() {
            if c.oes.doc_from_display.is_some() || c.oes.variant_type.is_some() {
                error!(
                    "{}: category `{}` wraps another error, so `{DOC_FROM_DISPLAY}` and `{VARIANT_TYPE}` don't apply",
                    self.node_pos(n),
                    c.name
                );
                return BAD_OBJECT_ATTRIBUTE.into();
            }
            if let Some(e) = children.first() {
                error!(
                    "{}: category `{}` wraps another error and holds no `{}` nodes",
                    self.node_pos(e),
                    c.name,
                    e.name().value()
                );
                return BAD_OBJECT_ATTRIBUTE.into();
            }
            return Ok(c);
        }

        for e in children {
            if e.name().value() != ERROR {
                error!(
                    "{}: a category holds `spec` and `error` nodes, not `{}`",
                    self.node_pos(e),
                    e.name().value()
                );
                return BAD_OBJECT_ATTRIBUTE.into();
            }
            c.errors.push(self.error(e)?);
        }

        if c.errors.is_empty() {
            error!("{}: category `{}` has no errors", self.node_pos(n), c.name);
            return EMPTY_LIST.into();
        }
        self.at(
            n,
            check_error_name_uniqueness(c.errors.iter().map(|e| e.name.as_str())),
        )?;

        Ok(c)
    }

    fn error(&self, n: &KdlNode) -> Result<ErrorSpec, TbError> {
        let (name, props) = self.name_arg(n, ERROR, &[DISPLAY])?;
        self.at(n, check_error_name(&name))?;
        let mut e = ErrorSpec {
            name,
            ..Default::default()
        };

        for p in props {
            e.display = Some(self.string_entry(DISPLAY, p)?);
        }

        let (spec, children) = self.split_children(n, ERROR)?;
        if let Some(c) = children.first() {
            error!(
                "{}: an error holds only a `spec` node, not `{}`",
                self.node_pos(c),
                c.name().value()
            );
            return BAD_OBJECT_ATTRIBUTE.into();
        }
        if let Some(spec) = spec {
            for (key, sn, v) in self.settings(spec)? {
                match key {
                    DISPLAY => e.display = Some(self.string(key, sn, v)?),
                    DOC => e.doc = Some(self.string(key, sn, v)?),
                    DOC_FROM_DISPLAY => e.oes.doc_from_display = Some(self.boolean(key, sn, v)?),
                    VARIANT_TYPE => match v {
                        KdlValue::Bool(b) => e.oes.variant_type = Some(*b),
                        KdlValue::String(s) => {
                            self.at(sn, check_variant_type_name(s))?;
                            e.oes.variant_type = Some(true);
                            e.variant_type_name = Some(s.clone());
                        }
                        _ => {
                            error!(
                                "{}: `{key}` must be a bool or a type name: {v}",
                                self.node_pos(sn)
                            );
                            return BAD_VALUE_TYPE.into();
                        }
                    },
                    _ => return self.unknown_setting(ERROR, key, sn),
                }
            }
        }

        Ok(e)
    }

    fn check_no_type(&self, n: &KdlNode) -> Result<(), TbError> {
        if n.ty().is_some() {
            error!(
                "{}: type annotations are not supported: `{}`",
                self.node_pos(n),
                n.name().value()
            );
            return BAD_VALUE_TYPE.into();
        }
        if let Some(e) = n.entries().iter().find(|e| e.ty().is_some()) {
            error!("{}: type annotations are not supported", self.entry_pos(e));
            return BAD_VALUE_TYPE.into();
        }
        Ok(())
    }

    fn name_arg<'n>(
        &self,
        n: &'n KdlNode,
        what: &str,
        line_props: &[&str],
    ) -> Result<(String, Vec<&'n KdlEntry>), TbError> {
        let mut name = None;
        let mut props: Vec<&KdlEntry> = Vec::new();

        for e in n.entries() {
            match e.name() {
                None if name.is_some() => {
                    error!("{}: {what} takes a single name", self.entry_pos(e));
                    return BAD_OBJECT_ATTRIBUTE.into();
                }
                None => match e.value().as_string() {
                    Some(s) => name = Some(s.to_owned()),
                    None => {
                        error!(
                            "{}: {what} name must be a string: {}",
                            self.entry_pos(e),
                            e.value()
                        );
                        return BAD_VALUE_TYPE.into();
                    }
                },
                Some(k) if line_props.contains(&k.value()) => {
                    if props
                        .iter()
                        .any(|p| p.name().map(|n| n.value()) == Some(k.value()))
                    {
                        error!("{}: `{}` repeats", self.entry_pos(e), k.value());
                        return BAD_OBJECT_ATTRIBUTE.into();
                    }
                    props.push(e);
                }
                Some(k) => {
                    error!(
                        "{}: `{}` isn't allowed on the {what} line, settings go in its `spec` block",
                        self.entry_pos(e),
                        k.value()
                    );
                    return BAD_OBJECT_ATTRIBUTE.into();
                }
            }
        }

        match name {
            Some(name) => Ok((name, props)),
            None => {
                error!("{}: {what} needs a name", self.node_pos(n));
                MISSING_ATTRIBUTE.into()
            }
        }
    }

    fn split_children<'n>(
        &self,
        n: &'n KdlNode,
        what: &str,
    ) -> Result<(Option<&'n KdlNode>, Vec<&'n KdlNode>), TbError> {
        let mut spec = None;
        let mut rest = Vec::new();

        for (i, c) in n.children().iter().flat_map(|d| d.nodes()).enumerate() {
            self.check_no_type(c)?;
            if c.name().value() == SPEC {
                if i != 0 {
                    error!(
                        "{}: the `spec` of a {what} must come first, and only once",
                        self.node_pos(c)
                    );
                    return BAD_OBJECT_ATTRIBUTE.into();
                }
                spec = Some(c);
            } else {
                rest.push(c);
            }
        }

        Ok((spec, rest))
    }

    fn settings<'n>(
        &self,
        spec: &'n KdlNode,
    ) -> Result<Vec<(&'n str, &'n KdlNode, &'n KdlValue)>, TbError> {
        if let Some(e) = spec.entries().first() {
            error!(
                "{}: `spec` takes no values; put the settings in its block",
                self.entry_pos(e)
            );
            return BAD_OBJECT_ATTRIBUTE.into();
        }

        let mut seen = HashSet::new();
        let mut settings = Vec::new();
        for s in spec.children().iter().flat_map(|d| d.nodes()) {
            self.check_no_type(s)?;
            let key = s.name().value();
            if s.children().is_some() {
                error!("{}: setting `{key}` takes no block", self.node_pos(s));
                return BAD_OBJECT_ATTRIBUTE.into();
            }
            let [entry] = s.entries() else {
                error!("{}: setting `{key}` takes one value", self.node_pos(s));
                return BAD_VALUE_TYPE.into();
            };
            if entry.name().is_some() {
                error!(
                    "{}: setting `{key}` takes a value, not a property",
                    self.entry_pos(entry)
                );
                return BAD_VALUE_TYPE.into();
            }
            if !seen.insert(key) {
                error!("{}: setting `{key}` repeats", self.node_pos(s));
                return BAD_OBJECT_ATTRIBUTE.into();
            }
            settings.push((key, s, entry.value()));
        }

        Ok(settings)
    }

    fn unknown_setting<T>(&self, what: &str, key: &str, n: &KdlNode) -> Result<T, TbError> {
        error!("{}: unknown {what} setting `{key}`", self.node_pos(n));
        BAD_OBJECT_ATTRIBUTE.into()
    }

    fn string(&self, key: &str, n: &KdlNode, v: &KdlValue) -> Result<String, TbError> {
        match v.as_string() {
            Some(s) => Ok(s.to_owned()),
            None => {
                error!("{}: `{key}` must be a string: {v}", self.node_pos(n));
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn string_entry(&self, key: &str, e: &KdlEntry) -> Result<String, TbError> {
        match e.value().as_string() {
            Some(s) => Ok(s.to_owned()),
            None => {
                error!(
                    "{}: `{key}` must be a string: {}",
                    self.entry_pos(e),
                    e.value()
                );
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn repr(&self, key: &str, n: &KdlNode, v: &KdlValue) -> Result<ReprType, TbError> {
        let name = self.string(key, n, v)?;
        match ReprType::from_name(&name) {
            Some(r) => Ok(r),
            None => {
                error!(
                    "{}: `{key}` must be `u8`, `u16`, `u32` or `u64`: {name}",
                    self.node_pos(n)
                );
                BAD_ENUM_VALUE.into()
            }
        }
    }

    fn type_path(&self, key: &str, n: &KdlNode, v: &KdlValue) -> Result<String, TbError> {
        let s = self.string(key, n, v)?;
        if syn::parse_str::<syn::Path>(&s).is_err() {
            error!("{}: `{key}` must be a type path: {s}", self.node_pos(n));
            return BAD_NAME.into();
        }
        Ok(s)
    }

    fn boolean(&self, key: &str, n: &KdlNode, v: &KdlValue) -> Result<bool, TbError> {
        match v.as_bool() {
            Some(b) => Ok(b),
            None => {
                error!("{}: `{key}` must be a bool: {v}", self.node_pos(n));
                BAD_VALUE_TYPE.into()
            }
        }
    }

    fn ident(&self, key: &str, n: &KdlNode, v: &KdlValue) -> Result<String, TbError> {
        let s = self.string(key, n, v)?;
        self.at(n, check_module_ident(&s, key))?;
        Ok(s)
    }
}

#[cfg(test)]
mod test_kdl_parser;
