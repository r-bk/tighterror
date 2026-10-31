use crate::{
    coder::idents,
    errors::TbErrorKind,
    parser::{kdl::*, testing::log_init},
    spec::{
        CategorySpec, ErrorSpec, MainSpec, ModuleSpec, OverridableErrorSpec, ReprType, Spec,
        WrappedSpec,
    },
};

fn error(name: &str) -> ErrorSpec {
    ErrorSpec {
        name: name.into(),
        ..Default::default()
    }
}

fn category(name: &str, errors: Vec<ErrorSpec>) -> CategorySpec {
    CategorySpec {
        name: name.into(),
        errors,
        ..Default::default()
    }
}

fn module(name: &str, categories: Vec<CategorySpec>) -> ModuleSpec {
    ModuleSpec {
        name: name.into(),
        categories,
        ..Default::default()
    }
}

fn spec(modules: Vec<ModuleSpec>) -> Spec {
    Spec {
        modules,
        ..Default::default()
    }
}

fn kind(s: &str) -> TbErrorKind {
    KdlParser::parse_str(s).unwrap_err().kind()
}

fn in_category(category_spec: &str, errors: &str) -> String {
    format!("module errors {{\n category General {{\n{category_spec}\n{errors}\n }}\n}}\n")
}

// Identifiers that fail the UpperCamel rules of type names, each with the
// error it must raise.
const BAD_TYPE_NAMES: [(&str, TbErrorKind); 7] = [
    ("notUpperCamelCase", BAD_IDENTIFIER_CASE),
    ("\"With Spaces\"", BAD_IDENTIFIER_CHARACTERS),
    ("\"\"", EMPTY_IDENTIFIER),
    ("\"  \"", BAD_IDENTIFIER_CHARACTERS),
    ("\" PaddedWithSpaces  \"", BAD_IDENTIFIER_CHARACTERS),
    ("Disallowed-Character-", BAD_IDENTIFIER_CHARACTERS),
    ("With_Underscore", BAD_IDENTIFIER_CHARACTERS),
];

fn with_module_spec(module_spec: &str) -> String {
    format!(
        "module errors {{\n spec {{ {module_spec} }}\n category General {{ error A_ERR }}\n}}\n"
    )
}

// Checks that a minimal specification, one module with one category of
// errors without settings, maps to the expected spec model.
#[test]
fn test_minimal() {
    log_init();

    // parse a spec with a single error node per error
    let s = "
module errors {
    category General {
        error BAD_FILE
        error BAD_ARG
    }
}
";
    let expected = spec(vec![module(
        "errors",
        vec![category(
            "General",
            vec![error("BAD_FILE"), error("BAD_ARG")],
        )],
    )]);

    // the names land in the module, category and errors
    assert_eq!(KdlParser::parse_str(s).unwrap(), expected);
}

// Checks the root `spec` settings and the rules for root nodes. It
// simulates file-wide settings, a misplaced or repeated root `spec`, and an
// unknown root node.
#[test]
fn test_root_spec() {
    log_init();

    // both root settings map to the main spec
    let s = "spec { output \"src/errors.rs\"; no-std #true }\n\
             module errors { category General { error BAD_FILE } }";
    let parsed = KdlParser::parse_str(s).unwrap();
    assert_eq!(
        parsed.main,
        MainSpec {
            output: Some("src/errors.rs".into()),
            no_std: Some(true),
            ..Default::default()
        }
    );

    // the root spec must come first and only once
    let s = "module errors { category General { error BAD_FILE } }\nspec { no-std #true }";
    assert_eq!(kind(s), BAD_ROOT_LEVEL_KEYWORD);

    // an unknown root setting and an unknown root node are rejected
    let s = "spec { name \"x\" }\nmodule errors { category General { error BAD_FILE } }";
    assert_eq!(kind(s), BAD_OBJECT_ATTRIBUTE);
    assert_eq!(kind("errors { error BAD_FILE }"), BAD_ROOT_LEVEL_KEYWORD);

    // a specification needs a module
    assert_eq!(kind("spec { no-std #true }"), MISSING_ATTRIBUTE);
    assert_eq!(kind(""), MISSING_ATTRIBUTE);
}

// Checks that every module setting maps to its field. It simulates a module
// that sets all of them in its `spec` block.
#[test]
fn test_module_settings() {
    log_init();

    // set every module setting once
    let s = "
module errors {
    spec {
        doc \"Module doc.\"
        err-doc \"Error doc.\"
        err-kind-doc \"Kind doc.\"
        err-cat-doc \"Category doc.\"
        err-name MyError
        err-kind-name MyErrorKind
        err-cat-name MyErrorCategory
        result-from-err #false
        result-from-err-kind #false
        error-trait #false
        flat-kinds #false
        doc-from-display #true
        variant-type #true
        repr u32
    }
    category General {
        error BAD_FILE
    }
}
";
    let m = &KdlParser::parse_str(s).unwrap().modules[0];

    // each setting landed in its field
    assert_eq!(m.doc.as_deref(), Some("Module doc."));
    assert_eq!(m.err_doc.as_deref(), Some("Error doc."));
    assert_eq!(m.err_kind_doc.as_deref(), Some("Kind doc."));
    assert_eq!(m.err_cat_doc.as_deref(), Some("Category doc."));
    assert_eq!(m.err_name.as_deref(), Some("MyError"));
    assert_eq!(m.err_kind_name.as_deref(), Some("MyErrorKind"));
    assert_eq!(m.err_cat_name.as_deref(), Some("MyErrorCategory"));
    assert_eq!(m.result_from_err, Some(false));
    assert_eq!(m.result_from_err_kind, Some(false));
    assert_eq!(m.error_trait, Some(false));
    assert_eq!(m.flat_kinds, Some(false));
    assert_eq!(m.oes.doc_from_display, Some(true));
    assert_eq!(m.oes.variant_type, Some(true));
    assert_eq!(m.repr, Some(ReprType::U32));
}

// Checks the values of the `repr` setting: the four unsigned integer types
// are accepted, quoted or not, and anything else is rejected.
#[test]
fn test_repr() {
    log_init();

    // every unsigned integer type is accepted
    for (name, repr) in [
        ("u8", ReprType::U8),
        ("u16", ReprType::U16),
        ("u32", ReprType::U32),
        ("u64", ReprType::U64),
    ] {
        let s = with_module_spec(&format!("repr {name}"));
        assert_eq!(
            KdlParser::parse_str(&s).unwrap().modules[0].repr,
            Some(repr)
        );
    }

    // a quoted name is the same string
    let s = with_module_spec("repr \"u16\"");
    assert_eq!(
        KdlParser::parse_str(&s).unwrap().modules[0].repr,
        Some(ReprType::U16)
    );

    // other types and other spellings are rejected
    for bad in ["i32", "u128", "usize", "U8"] {
        let s = with_module_spec(&format!("repr {bad}"));
        assert_eq!(kind(&s), BAD_ENUM_VALUE, "repr {bad}");
    }

    // values that are not strings are rejected
    for bad in ["8", "#true"] {
        let s = with_module_spec(&format!("repr {bad}"));
        assert_eq!(kind(&s), BAD_VALUE_TYPE, "repr {bad}");
    }
}

// Checks the category settings and that a category holds only errors.
#[test]
fn test_category_settings() {
    log_init();

    // set every category setting once
    let s = in_category(
        "spec { doc \"Category doc.\"; doc-from-display #true; variant-type #true }",
        "error BAD_FILE",
    );
    let c = &KdlParser::parse_str(&s).unwrap().modules[0].categories[0];
    assert_eq!(c.doc.as_deref(), Some("Category doc."));
    assert_eq!(c.oes.doc_from_display, Some(true));
    assert_eq!(c.oes.variant_type, Some(true));

    // a module setting is unknown in a category
    let s = in_category("spec { flat-kinds #true }", "error BAD_FILE");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);

    // a category without errors, or with a foreign node, is rejected
    assert_eq!(kind(&in_category("", "")), EMPTY_LIST);
    assert_eq!(kind(&in_category("", "module x")), BAD_OBJECT_ATTRIBUTE);
}

// A module with a category of its own errors, and a category `Text` with
// the given spec and child nodes.
fn with_wrapping_category(category_spec: &str, children: &str) -> String {
    format!(
        "module errors {{\n category General {{ error BAD_FILE }}\n category Text {{\n{category_spec}\n{children}\n }}\n}}\n"
    )
}

// Checks a category that wraps the error of another crate: it names both
// types, holds no errors, and has a category next to it. It simulates the
// specs that a crate wrapping the errors of another one writes.
#[test]
fn test_wrapped_category() {
    log_init();

    // both types, bare and quoted
    let s = with_wrapping_category(
        "spec { wrap-error-type text::Error; wrap-kind-type \"crate::text::ErrorKind\" }",
        "",
    );
    let m = &KdlParser::parse_str(&s).unwrap().modules[0];
    let expected = WrappedSpec {
        error_type: "text::Error".into(),
        kind_type: "crate::text::ErrorKind".into(),
    };
    assert_eq!(m.categories[1].wrapped, Some(expected));
    assert!(m.categories[1].errors.is_empty());
    assert_eq!(m.categories[0].wrapped, None);

    // one type without the other is rejected
    for spec in [
        "spec { wrap-error-type text::Error }",
        "spec { wrap-kind-type text::ErrorKind }",
    ] {
        assert_eq!(
            kind(&with_wrapping_category(spec, "")),
            MISSING_ATTRIBUTE,
            "{spec}"
        );
    }

    // a value that is not a type path is rejected
    for bad in ["\"text::\"", "\"text Error\"", "\"\"", "#true"] {
        let spec = format!("spec {{ wrap-error-type {bad}; wrap-kind-type text::ErrorKind }}");
        let expected = if bad == "#true" {
            BAD_VALUE_TYPE
        } else {
            BAD_NAME
        };
        assert_eq!(kind(&with_wrapping_category(&spec, "")), expected, "{bad}");
    }

    // errors, and defaults for errors, are rejected in a wrapping category
    let spec = "spec { wrap-error-type text::Error; wrap-kind-type text::ErrorKind }";
    let s = with_wrapping_category(spec, "error BAD_GLYPH");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);
    for default in ["doc-from-display #true", "variant-type #false"] {
        let spec = format!(
            "spec {{ wrap-error-type text::Error; wrap-kind-type text::ErrorKind; {default} }}"
        );
        assert_eq!(
            kind(&with_wrapping_category(&spec, "")),
            BAD_OBJECT_ATTRIBUTE,
            "{default}"
        );
    }

    // a module whose only category wraps is rejected
    let s = format!("module errors {{\n category Text {{ {spec} }}\n}}\n");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);

    // a module of wrapping categories alone is accepted
    let s = format!(
        "module errors {{\n category Text {{ {spec} }}\n category Font {{ spec {{ wrap-error-type font::Error; wrap-kind-type font::ErrorKind }} }}\n}}\n"
    );
    assert_eq!(
        KdlParser::parse_str(&s).unwrap().modules[0]
            .categories
            .len(),
        2
    );

    // a type that two categories wrap is rejected
    for other in [
        "wrap-error-type text::Error; wrap-kind-type font::ErrorKind",
        "wrap-error-type font::Error; wrap-kind-type text::ErrorKind",
    ] {
        let s = format!(
            "module errors {{\n category Text {{ {spec} }}\n category Font {{ spec {{ {other} }} }}\n}}\n"
        );
        assert_eq!(kind(&s), NON_UNIQUE_NAME, "{other}");
    }
}

// Checks the error settings: `display` on the line or in `spec`, with
// `spec` taking precedence, a multi-line `doc`, and `variant-type` as a bool
// or as a type name.
#[test]
fn test_error_settings() {
    log_init();

    // display on the error's line
    let s = in_category("", "error BAD_FILE display=\"Bad file.\"");
    let e = &KdlParser::parse_str(&s).unwrap().modules[0].categories[0].errors[0];
    assert_eq!(e.display.as_deref(), Some("Bad file."));

    // display in spec wins over display on the line
    let s = in_category(
        "",
        "error BAD_FILE display=\"Line.\" { spec { display \"Spec.\" } }",
    );
    let e = &KdlParser::parse_str(&s).unwrap().modules[0].categories[0].errors[0];
    assert_eq!(e.display.as_deref(), Some("Spec."));

    // a multi-line doc keeps its lines without the indentation
    let s = in_category(
        "",
        "error BAD_FILE {\n spec {\n  doc \"\"\"\n   First line.\n\n   Second line.\n   \"\"\"\n }\n}",
    );
    let e = &KdlParser::parse_str(&s).unwrap().modules[0].categories[0].errors[0];
    assert_eq!(e.doc.as_deref(), Some("First line.\n\nSecond line."));

    // variant-type as a bool and as a type name
    let s = in_category(
        "",
        "error A_ERR { spec { variant-type #false; doc-from-display #true } }\n\
         error B_ERR { spec { variant-type CustomName } }",
    );
    let errors = &KdlParser::parse_str(&s).unwrap().modules[0].categories[0].errors;
    assert_eq!(
        errors[0].oes,
        OverridableErrorSpec {
            variant_type: Some(false),
            doc_from_display: Some(true),
        }
    );
    assert_eq!(errors[1].oes.variant_type, Some(true));
    assert_eq!(errors[1].variant_type_name.as_deref(), Some("CustomName"));

    // a bad type name, a wrong value type and a repeated display are rejected
    let s = in_category("", "error A_ERR { spec { variant-type badName } }");
    assert_eq!(kind(&s), BAD_IDENTIFIER_CASE);
    let s = in_category("", "error A_ERR { spec { variant-type 1 } }");
    assert_eq!(kind(&s), BAD_VALUE_TYPE);
    let s = in_category("", "error A_ERR display=\"a\" display=\"b\"");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);

    // a repeated display is rejected when one of the keys is quoted too
    let s = in_category("", "error A_ERR display=\"a\" \"display\"=\"b\"");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);
}

// Checks the rules of `spec` blocks. It simulates the mistakes a writer can
// make with them.
#[test]
fn test_spec_rules() {
    log_init();

    // spec must be the first child, and only once
    let s = in_category("", "error A_ERR\nspec { doc \"x\" }");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);
    let s = in_category("spec { doc \"x\" }\nspec { doc \"y\" }", "error A_ERR");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);

    // spec takes no values of its own
    let s = in_category("spec doc=\"x\"", "error A_ERR");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);

    // a setting takes exactly one plain value
    let s = in_category("spec { doc \"x\" \"y\" }", "error A_ERR");
    assert_eq!(kind(&s), BAD_VALUE_TYPE);
    let s = in_category("spec { doc }", "error A_ERR");
    assert_eq!(kind(&s), BAD_VALUE_TYPE);
    let s = in_category("spec { doc value=\"x\" }", "error A_ERR");
    assert_eq!(kind(&s), BAD_VALUE_TYPE);
    let s = in_category("spec { doc \"x\" { more } }", "error A_ERR");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);

    // a setting repeated in one spec, or of the wrong type, is rejected
    let s = in_category("spec { doc \"x\"; doc \"y\" }", "error A_ERR");
    assert_eq!(kind(&s), BAD_OBJECT_ATTRIBUTE);
    let s = in_category("spec { doc-from-display \"yes\" }", "error A_ERR");
    assert_eq!(kind(&s), BAD_VALUE_TYPE);
}

// Checks the rules of node lines: one string name, and no properties other
// than an error's `display`. It also checks that type annotations are
// rejected.
#[test]
fn test_node_rules() {
    log_init();

    // a missing, repeated or non-string name is rejected
    assert_eq!(
        kind("module { category General { error A_ERR } }"),
        MISSING_ATTRIBUTE
    );
    assert_eq!(
        kind("module errors other { category General { error A_ERR } }"),
        BAD_OBJECT_ATTRIBUTE
    );
    assert_eq!(kind(&in_category("", "error 5")), BAD_VALUE_TYPE);

    // settings belong in spec, not on the node line
    assert_eq!(
        kind("module errors flat-kinds=#true { category General { error A_ERR } }"),
        BAD_OBJECT_ATTRIBUTE
    );
    assert_eq!(
        kind(&in_category("", "error A_ERR doc=\"x\"")),
        BAD_OBJECT_ATTRIBUTE
    );

    // type annotations on nodes and values are rejected
    assert_eq!(kind(&in_category("", "(t)error A_ERR")), BAD_VALUE_TYPE);
    assert_eq!(kind(&in_category("", "error (t)A_ERR")), BAD_VALUE_TYPE);

    // a module without categories, or with a foreign node, is rejected
    assert_eq!(kind("module errors"), EMPTY_LIST);
    assert_eq!(kind("module errors { error A_ERR }"), BAD_OBJECT_ATTRIBUTE);
}

// Checks that names follow their case rules and are unique where the
// layout requires it.
#[test]
fn test_names() {
    log_init();

    // each kind of name has its case
    assert_eq!(
        kind("module Errors { category General { error A_ERR } }"),
        BAD_NAME
    );
    assert_eq!(
        kind("module errors { category general { error A_ERR } }"),
        BAD_IDENTIFIER_CASE
    );
    assert_eq!(kind(&in_category("", "error a_err")), BAD_IDENTIFIER_CASE);
    assert_eq!(
        kind("module errors { spec { err-name myError }\n category General { error A_ERR } }"),
        BAD_IDENTIFIER_CASE
    );

    // module, category and error names must be unique
    let s = "module errors { category General { error A_ERR } }\n\
             module errors { category General { error A_ERR } }";
    assert_eq!(kind(s), NON_UNIQUE_NAME);
    let s =
        "module errors {\n category General { error A_ERR }\n category General { error B_ERR }\n}";
    assert_eq!(kind(s), NON_UNIQUE_NAME);
    assert_eq!(
        kind(&in_category("", "error A_ERR\nerror A_ERR")),
        NON_UNIQUE_NAME
    );

    // flat kinds, the default, need error names unique per module
    let s = "module errors {\n category One { error A_ERR }\n category Two { error A_ERR }\n}";
    assert_eq!(kind(s), NON_UNIQUE_NAME);
    let s = "module errors {\n spec { flat-kinds #false }\n category One { error A_ERR }\n category Two { error A_ERR }\n}";
    assert!(KdlParser::parse_str(s).is_ok());
}

// Checks that text which is not valid KDL is rejected as such.
#[test]
fn test_invalid_kdl() {
    log_init();

    // an unclosed block and an unclosed string
    assert_eq!(kind("module errors {"), BAD_KDL);
    assert_eq!(
        kind(&in_category("", "error A_ERR display=\"open")),
        BAD_KDL
    );
}

// Checks the rules of module, category and error names beyond their case:
// empty names, foreign characters and the module name's lower snake case.
#[test]
fn test_name_rules() {
    log_init();

    // a module name is lower snake case and not empty
    for good in ["errors", "my_errors"] {
        let s = format!("module {good} {{ category General {{ error A_ERR }} }}");
        assert_eq!(KdlParser::parse_str(&s).unwrap().modules[0].name(), good);
    }
    for bad in ["\"\"", "My_Errors", "ERRORS"] {
        let s = format!("module {bad} {{ category General {{ error A_ERR }} }}");
        assert_eq!(kind(&s), BAD_NAME, "{bad}");
    }

    // a category name follows the type name rules
    for (bad, k) in BAD_TYPE_NAMES {
        let s = format!("module errors {{ category {bad} {{ error A_ERR }} }}");
        assert_eq!(kind(&s), k, "{bad}");
    }

    // an error name is upper snake case without foreign characters
    for (bad, k) in [
        ("\"\"", EMPTY_IDENTIFIER),
        ("\"  \"", BAD_IDENTIFIER_CHARACTERS),
        ("camelCase", BAD_IDENTIFIER_CASE),
        ("\"With Spaces\"", BAD_IDENTIFIER_CHARACTERS),
        ("With-Dashes", BAD_IDENTIFIER_CHARACTERS),
        ("UpperCamelCase", BAD_IDENTIFIER_CASE),
        ("\"BadChars+\"", BAD_IDENTIFIER_CHARACTERS),
    ] {
        assert_eq!(kind(&in_category("", &format!("error {bad}"))), k, "{bad}");
    }
}

// Checks the names of a module's error, kind and category types. It
// simulates custom names, the defaults set explicitly, malformed names,
// a name that another type reserves, and two types given the same name.
#[test]
fn test_type_names() {
    log_init();

    // custom names and the defaults are accepted
    for (key, good) in [
        ("err-name", "MyError"),
        ("err-name", idents::ERROR),
        ("err-kind-name", "MyErrorKind"),
        ("err-kind-name", idents::ERROR_KIND),
        ("err-cat-name", "MyErrorCategory"),
        ("err-cat-name", idents::ERROR_CATEGORY),
    ] {
        let s = with_module_spec(&format!("{key} {good}"));
        assert!(KdlParser::parse_str(&s).is_ok(), "{key} {good}");
    }

    // each type name follows the type name rules
    for key in ["err-name", "err-kind-name", "err-cat-name"] {
        for (bad, k) in BAD_TYPE_NAMES {
            let s = with_module_spec(&format!("{key} {bad}"));
            assert_eq!(kind(&s), k, "{key} {bad}");
        }
    }

    // a type cannot take the default name of another type
    for (key, bad) in [
        ("err-name", idents::ERROR_KIND),
        ("err-name", idents::ERROR_CATEGORY),
        ("err-kind-name", idents::ERROR),
        ("err-kind-name", idents::ERROR_CATEGORY),
        ("err-cat-name", idents::ERROR),
        ("err-cat-name", idents::ERROR_KIND),
    ] {
        let s = with_module_spec(&format!("{key} {bad}"));
        assert_eq!(kind(&s), BAD_MODULE_IDENTIFIER, "{key} {bad}");
    }

    // two types cannot share a custom name
    for pair in [
        "err-name Same; err-kind-name Same",
        "err-name Same; err-cat-name Same",
        "err-kind-name Same; err-cat-name Same",
    ] {
        assert_eq!(kind(&with_module_spec(pair)), NAME_COLLISION, "{pair}");
    }
}

// Checks the rules of variant type names: their case, collisions with the
// module's type names, and uniqueness. It simulates both explicit names and
// names derived from error names.
#[test]
fn test_variant_type_names() {
    log_init();

    // a variant type name follows the type name rules, avoids the module's
    // type names and is unique
    let pfx = "module errors {\n spec { err-name CustomErr; err-cat-name CustomErrCat; err-kind-name CustomErrKind }\n category General {\n  error EXISTING_ERR { spec { variant-type ExistingErr } }\n";
    for (bad, k) in [
        ("MY_ERR", BAD_IDENTIFIER_CHARACTERS),
        ("my_err", BAD_IDENTIFIER_CHARACTERS),
        ("myErr", BAD_IDENTIFIER_CASE),
        ("myerr", BAD_IDENTIFIER_CASE),
        ("CustomErr", NAME_COLLISION),
        ("CustomErrCat", NAME_COLLISION),
        ("CustomErrKind", NAME_COLLISION),
        ("ExistingErr", NON_UNIQUE_NAME),
    ] {
        let s = format!("{pfx}  error MY_ERR {{ spec {{ variant-type {bad} }} }}\n }}\n}}");
        assert_eq!(kind(&s), k, "{bad}");
    }

    // with flat kinds, variant type names are unique across categories
    let s = "module errors {\n spec { variant-type #true }\n category CatOne { error ONE { spec { variant-type Same } } }\n category CatTwo { error TWO { spec { variant-type Same } } }\n}";
    assert_eq!(kind(s), NON_UNIQUE_NAME);

    // without flat kinds, the same name in two categories does not clash
    let s = "module errors {\n spec { variant-type #true; flat-kinds #false }\n category CatOne { error ONE { spec { variant-type Same } } }\n category CatTwo { error TWO { spec { variant-type Same } } }\n}";
    assert!(KdlParser::parse_str(s).is_ok());
}

// Checks that variant type names collide or repeat only among the errors
// that get a variant type. The errors simulate names whose derived variant
// type names are `Error`, `ErrorKind`, `ErrorCategory` and `FooBar`.
#[test]
fn test_variant_type_names_need_variant_type() {
    log_init();

    // without variant types the derived names collide with nothing
    let s = in_category("", "error ERROR\nerror ERROR_KIND\nerror ERROR_CATEGORY");
    assert!(KdlParser::parse_str(&s).is_ok());

    // a module default gives each error a variant type that collides
    for name in ["ERROR", "ERROR_KIND", "ERROR_CATEGORY"] {
        let s = format!(
            "module errors {{\n spec {{ variant-type #true }}\n category General {{ error {name} }}\n}}"
        );
        assert_eq!(kind(&s), NAME_COLLISION, "{name}");
    }

    // an error that turns the module default off collides with nothing
    let s = "module errors {\n spec { variant-type #true }\n category General { error ERROR_KIND { spec { variant-type #false } } }\n}";
    assert!(KdlParser::parse_str(s).is_ok());

    // only BAZ gets a variant type, so FOO_BAR's derived name is not taken
    let s = in_category(
        "",
        "error FOO_BAR\nerror BAZ { spec { variant-type FooBar } }",
    );
    assert!(KdlParser::parse_str(&s).is_ok());

    // both errors get a variant type named FooBar
    let s = in_category(
        "",
        "error FOO_BAR { spec { variant-type #true } }\nerror BAZ { spec { variant-type FooBar } }",
    );
    assert_eq!(kind(&s), NON_UNIQUE_NAME);

    // a category default gives FOO_BAR its derived variant type FooBar
    let s = in_category(
        "spec { variant-type #true }",
        "error FOO_BAR\nerror BAZ { spec { variant-type FooBar } }",
    );
    assert_eq!(kind(&s), NON_UNIQUE_NAME);

    // across categories with flat kinds, only BAZ gets a variant type
    let s = "module errors {\n category CatOne { error FOO_BAR }\n category CatTwo { error BAZ { spec { variant-type FooBar } } }\n}";
    assert!(KdlParser::parse_str(s).is_ok());
}
