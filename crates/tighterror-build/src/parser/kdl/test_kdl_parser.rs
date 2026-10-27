use crate::{
    errors::TbErrorKind,
    parser::{kdl::*, testing::log_init},
    spec::{CategorySpec, ErrorSpec, MainSpec, ModuleSpec, OverridableErrorSpec, Spec},
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
        name: Some(name.into()),
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
