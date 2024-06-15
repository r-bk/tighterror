use crate::{
    coder::idents,
    errors::{kinds::parser::*, TbErrorKind},
    parser::{
        testing::{
            log_init, spec_from_category, spec_from_err, spec_from_err_iter, spec_from_main,
            spec_from_module,
        },
        toml::*,
    },
    spec::{ErrorSpec, OverridableErrorSpec},
};

const GOOD_BOOLEANS: [(&str, bool); 2] = [("true", true), ("false", false)];
const BAD_BOOLEANS: [(&str, TbErrorKind); 8] = [
    ("yes", BAD_TOML),
    ("tr ue", BAD_TOML),
    ("1", BAD_VALUE_TYPE),
    ("on", BAD_TOML),
    ("null", BAD_TOML),
    ("True", BAD_TOML),
    ("False", BAD_TOML),
    ("no", BAD_TOML),
];
const BAD_IDENTS: [(&str, TbErrorKind); 7] = [
    ("\"notUpperCamelCase\"", BAD_IDENTIFIER_CASE),
    ("\"With Spaces\"", BAD_IDENTIFIER_CHARACTERS),
    ("\"\"", EMPTY_IDENTIFIER),
    ("\"  \"", BAD_IDENTIFIER_CHARACTERS),
    ("\" PaddedWithSpaces  \"", BAD_IDENTIFIER_CHARACTERS),
    ("\"Disallowed-Character-\"", BAD_IDENTIFIER_CHARACTERS),
    ("\"null\"", BAD_IDENTIFIER_CASE),
];

#[test]
fn test_minimal() {
    log_init();
    let s = "
errors = [\"SingleError\"]
";

    let err = ErrorSpec {
        name: "SingleError".into(),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_minimal_with_display() {
    log_init();
    let s1 = "
errors = [
    { name = \"SingleError\", display = \"The single error in the test.\" }
]
";
    let s2 = "
[[errors]]
name = \"SingleError\"
display = \"The single error in the test.\"
";

    let err = ErrorSpec {
        name: "SingleError".into(),
        display: Some("The single error in the test.".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);

    for s in [s1, s2] {
        let res = TomlParser::parse_str(s).unwrap();
        assert_eq!(res, spec);
    }
}

#[test]
fn test_minimal_explicit() {
    log_init();
    let s = "
[[errors]]
name = \"SingleError\"
";
    let err = ErrorSpec {
        name: "SingleError".into(),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_err_doc_from_display() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[[errors]]\nname = \"TestError\"\ndoc_from_display = {}",
            good.0
        );
        let err = ErrorSpec {
            name: "TestError".into(),
            oes: OverridableErrorSpec {
                doc_from_display: Some(good.1),
            },
            ..Default::default()
        };
        let spec = spec_from_err(err);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[[errors]]\nname = \"TestError\"\ndoc_from_display = {}",
            bad
        );

        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_err_display() {
    log_init();
    for good in [
        ("\"An error occurred.\"", "An error occurred."),
        ("\"\"", ""),
        ("\"1\"", "1"),
    ] {
        let s = format!("[[errors]]\nname = \"TestError\"\ndisplay = {}", good.0);
        let err = ErrorSpec {
            name: "TestError".into(),
            display: Some(good.1.into()),
            ..Default::default()
        };
        let spec = spec_from_err(err);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    #[allow(clippy::single_element_loop)]
    for bad in ["1"] {
        let s = format!("[[errors]]\nname = \"TestError\"\ndisplay = {}", bad);
        assert_eq!(
            TomlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_err_doc() {
    log_init();
    for good in [
        ("\"An error doc.\"", "An error doc."),
        ("\"\"", ""),
        ("\"1\"", "1"),
    ] {
        let s = format!("[[errors]]\nname = \"TestError\"\ndoc = {}", good.0);
        let err = ErrorSpec {
            name: "TestError".into(),
            doc: Some(good.1.into()),
            ..Default::default()
        };
        let spec = spec_from_err(err);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    let s = "
[[errors]]
name = \"TestError\"
doc = \"\"\"
A multiline doc.

Second line.
\"\"\"
";
    let err = ErrorSpec {
        name: "TestError".into(),
        doc: Some("A multiline doc.\n\nSecond line.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);

    #[allow(clippy::single_element_loop)]
    for bad in ["1"] {
        let s = format!("[[errors]]\nname = \"TestError\"\ndoc = {}", bad);
        assert_eq!(
            TomlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_err_name() {
    log_init();

    const BAD_NAMES: &[(&str, TbErrorKind)] = &[
        ("\"\"", EMPTY_IDENTIFIER),
        ("\"  \"", BAD_IDENTIFIER_CHARACTERS),
        ("\"camelCase\"", BAD_IDENTIFIER_CASE),
        ("1", BAD_VALUE_TYPE),
        ("\"With Spaces\"", BAD_IDENTIFIER_CHARACTERS),
        ("\"With-Dashes\"", BAD_IDENTIFIER_CHARACTERS),
        ("\"CAPITAL_LETTERS\"", BAD_IDENTIFIER_CASE),
        ("\"BadChars+\"", BAD_IDENTIFIER_CHARACTERS),
    ];

    for (bad, kind) in BAD_NAMES {
        let s = format!("[[errors]]\nname = {}", bad);
        let res = TomlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), *kind);
    }

    for (bad, kind) in BAD_NAMES {
        let s = format!("errors = [{}]", bad);
        let res = TomlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), *kind);
    }
}

#[test]
fn test_err() {
    log_init();

    let s1 = "
[[errors]]
name = \"TestError\"
doc = \"An error doc.\"
display = \"An error description.\"
doc_from_display = false
";

    let s2 = "
errors = [
    { name = \"TestError\", doc = \"An error doc.\", display = \"An error description.\", doc_from_display = false }
]
";
    let err = ErrorSpec {
        name: "TestError".into(),
        doc: Some("An error doc.".into()),
        display: Some("An error description.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(false),
        },
    };
    let spec = spec_from_err(err);

    for s in [s1, s2] {
        let res = TomlParser::parse_str(s).unwrap();
        assert_eq!(res, spec);
    }
}

#[test]
fn test_errs() {
    log_init();
    let s1 = "
errors = [
    { name = \"FirstErr\" },
    { name = \"SecondErr\", display = \"With display.\" },
    { name = \"TestError\", doc = \"An error doc.\", display = \"An error description.\", doc_from_display = false },
    { name = \"Err2\", doc = \"Another error.\" },
    { name = \"Err3\", display = \"A third one.\", doc_from_display = true }
]
";

    let s2 = "
[[errors]]
name = \"FirstErr\"

[[errors]]
name = \"SecondErr\"
display = \"With display.\"

[[errors]]
name = \"TestError\"
doc = \"An error doc.\"
display = \"An error description.\"
doc_from_display = false

[[errors]]
name = \"Err2\"
doc = \"Another error.\"

[[errors]]
name = \"Err3\"
display = \"A third one.\"
doc_from_display = true
";

    let err1 = ErrorSpec {
        name: "FirstErr".into(),
        ..Default::default()
    };
    let err2 = ErrorSpec {
        name: "SecondErr".into(),
        display: Some("With display.".into()),
        ..Default::default()
    };
    let err3 = ErrorSpec {
        name: "TestError".into(),
        doc: Some("An error doc.".into()),
        display: Some("An error description.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(false),
        },
    };
    let err4 = ErrorSpec {
        name: "Err2".into(),
        doc: Some("Another error.".into()),
        ..Default::default()
    };
    let err5 = ErrorSpec {
        name: "Err3".into(),
        display: Some("A third one.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(true),
        },
        ..Default::default()
    };
    let spec = spec_from_err_iter([err1, err2, err3, err4, err5]);
    for s in [s1, s2] {
        let res = TomlParser::parse_str(s).unwrap();
        assert_eq!(res, spec);
    }
}

#[test]
fn test_top_kws() {
    log_init();
    let s = "
my_errors = [\"BadError\"]
";
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        BAD_ROOT_LEVEL_KEYWORD
    );

    let s = "
[module]
doc_from_display = true

";
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );
}

#[test]
fn test_module_doc_from_display() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[module]\ndoc_from_display = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            oes: OverridableErrorSpec {
                doc_from_display: Some(good.1),
            },
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[module]\ndoc_from_display = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_module_name() {
    log_init();
    for good_name in ["errors", "my_errors"] {
        let s = format!("[module]\nname = \"{good_name}\"\n[[errors]]\nname = \"DummyErr\"");
        let module = ModuleSpec {
            name: Some(good_name.into()),
            ..Default::default()
        };
        assert_eq!(spec_from_module(module), TomlParser::parse_str(&s).unwrap());
    }

    for bad_name in ["\"\"", "\"My_Errors\"", "\"ERRORS\""] {
        let s = format!("[module]\nname = {bad_name}\n[[errors]]\nname = \"DummyErr\"");
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), BAD_NAME);
    }
}

#[test]
fn test_module_doc() {
    log_init();
    let s = "
[module]
doc = \"\"\"
Module documentation.

Multiline.
\"\"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        doc: Some("Module documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
[module]
doc = \"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_err_cat_doc() {
    log_init();
    let s = "
[module]
err_cat_doc = \"\"\"
Category documentation.

Multiline.
\"\"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        err_cat_doc: Some("Category documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
[module]
err_cat_doc = \"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        err_cat_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_err_doc() {
    log_init();
    let s = "
[module]
err_doc = \"\"\"
Error documentation.

Multiline.
\"\"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        err_doc: Some("Error documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
[module]
err_doc = \"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        err_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_err_kind_doc() {
    log_init();
    let s = "
[module]
err_kind_doc = \"\"\"
ErrorKind documentation.

Multiline.
\"\"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        err_kind_doc: Some("ErrorKind documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
[module]
err_kind_doc = \"\"

[[errors]]
name = \"DummyErr\"
";
    let module = ModuleSpec {
        err_kind_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_result_from_err() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[module]\nresult_from_err = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            result_from_err: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[module]\nresult_from_err = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_module_result_from_err_kind() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[module]\nresult_from_err_kind = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            result_from_err_kind: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[module]\nresult_from_err_kind = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_error_trait() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[module]\nerror_trait = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            error_trait: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[module]\nerror_trait = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_module_flat_kinds() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[module]\nflat_kinds = {}\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            flat_kinds: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[module]\nflat_kinds = {}\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_module_flat_kinds_error_name_uniqueness() {
    log_init();

    let s = r#"
[module]
flat_kinds = true

[[categories]]
name = "Cat1"

[[categories.errors]]
name = "Err1"
display = "first error"

[[categories]]
name = "Cat2"

[[categories.errors]]
name = "Err1"
display = "another first error"
"#;

    assert_eq!(TomlParser::parse_str(s), NON_UNIQUE_NAME.into());

    let s = r#"
[module]
flat_kinds = true

[[categories]]
name = "Cat1"

[[categories.errors]]
name = "Err1"
display = "first error"

[[categories]]
name = "Cat2"

[[categories.errors]]
name = "Err2"
display = "second error"
"#;

    assert!(TomlParser::parse_str(s).is_ok());
}

#[test]
fn test_module_categories_forbidden_in_single() {
    log_init();

    let s = r#"
[module]
name = "my_errors"

[[module.categories]]
name = "BadCategory"
errors = ["BadError"]

[[errors]]
name = "GoodError"
"#;

    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        BAD_OBJECT_ATTRIBUTE
    );

    let s = r#"
[module]
name = "my_errors"

[[errors]]
name = "GoodError"
"#;

    assert!(TomlParser::parse_str(s).is_ok());
}

#[test]
fn test_no_std() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[main]\nno_std = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let main = MainSpec {
            no_std: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_main(main);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[main]\nno_std = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_error_name() {
    log_init();
    for good in ["MyError", idents::ERROR] {
        let module = ModuleSpec {
            err_name: Some(good.into()),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&format!(
            "[module]\nerr_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        let s = format!(
            "[module]\nerr_name = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }

    for bad in [idents::ERROR_CATEGORY, idents::ERROR_KIND] {
        let s = format!(
            "[module]\nerr_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s), BAD_MODULE_IDENTIFIER.into());
    }
}

#[test]
fn test_error_kind_name() {
    log_init();
    for good in ["MyErrorKind", idents::ERROR_KIND] {
        let module = ModuleSpec {
            err_kind_name: Some(good.into()),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&format!(
            "[module]\nerr_kind_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        let s = format!(
            "[module]\nerr_kind_name = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }

    for bad in [idents::ERROR, idents::ERROR_CATEGORY] {
        let s = format!(
            "[module]\nerr_kind_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s), BAD_MODULE_IDENTIFIER.into());
    }
}

#[test]
fn test_error_cat_name() {
    log_init();
    for good in ["MyErrorCategory", idents::ERROR_CATEGORY] {
        let module = ModuleSpec {
            err_cat_name: Some(good.into()),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::parse_str(&format!(
            "[module]\nerr_cat_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        let s = format!(
            "[module]\nerr_cat_name = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }

    for bad in [idents::ERROR, idents::ERROR_KIND] {
        let s = format!(
            "[module]\nerr_cat_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s), BAD_MODULE_IDENTIFIER.into());
    }
}

#[test]
fn test_error_list_unique_names() {
    log_init();

    let s = "errors = [\"FirstError\",  \"FirstError\", \"SecondError\"]";
    assert_eq!(TomlParser::parse_str(s), NON_UNIQUE_NAME.into());

    let s = "errors = [\"FirstError\",  \"SecondError\"]";
    assert!(TomlParser::parse_str(s).is_ok());
}

#[test]
fn test_category_name() {
    log_init();

    for good in ["TheCategory", IMPLICIT_CATEGORY_NAME] {
        let category = CategorySpec {
            name: good.to_owned(),
            ..Default::default()
        };

        let spec = spec_from_category(category);
        let res = TomlParser::parse_str(&format!(
            "[category]\nname = \"{good}\"\n[[errors]]\nname = \"DummyErr\""
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        assert_eq!(
            TomlParser::parse_str(&format!(
                "[category]\nname = {bad}\n[[errors]]\nname = \"DummyErr\""
            ))
            .unwrap_err()
            .kind(),
            kind
        );
    }

    for bad in [kws::MAIN, kws::ERRORS] {
        assert_eq!(
            TomlParser::parse_str(&format!(
                "[category]\nname = \"{bad}\"\n[[errors]]\nname = \"DummyErr\""
            ))
            .unwrap_err()
            .kind(),
            BAD_IDENTIFIER_CASE
        );
    }
}

#[test]
fn test_category_doc() {
    let s = r#"
[category]
doc = """
Category long doc string.

Appears on multiple lines.
"""

[[errors]]
name = "DummyErr"
"#;
    let spec = spec_from_category(CategorySpec {
        name: IMPLICIT_CATEGORY_NAME.into(),
        doc: Some("Category long doc string.\n\nAppears on multiple lines.\n".into()),
        ..Default::default()
    });
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = r#"
[category]
doc = ""

[[errors]]
name = "DummyErr"
"#;
    let spec = spec_from_category(CategorySpec {
        name: IMPLICIT_CATEGORY_NAME.into(),
        doc: Some("".into()),
        ..Default::default()
    });
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_category_doc_from_display() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "[category]\ndoc_from_display = {}\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let cat = CategorySpec {
            name: IMPLICIT_CATEGORY_NAME.into(),
            oes: OverridableErrorSpec {
                doc_from_display: Some(good.1),
            },
            ..Default::default()
        };
        let spec = spec_from_category(cat);
        let res = TomlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_BOOLEANS {
        let s = format!(
            "[category]\ndoc_from_display = {}\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::parse_str(&s).unwrap_err().kind(), kind);
    }
}

#[test]
fn test_category_errors_list_in_single_category() {
    log_init();

    let s = r#"
errors = ["DummyErr"]

[category]
name = "Custom"
doc = "Custom category."
doc_from_display = false
errors = ["DummyErr"]
"#;

    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        BAD_OBJECT_ATTRIBUTE
    );
}

#[test]
fn test_category_errors_and_categories_missing() {
    log_init();

    let s = r#"
[category]
name = "Custom"
doc = "Custom category."
doc_from_display = false
"#;
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );
}

#[test]
fn test_category_errors_and_categories_mutual_exclusion() {
    log_init();

    let s = r#"
[[categories]]
name = "Custom"
doc = "Custom category."
doc_from_display = false
errors = ["DummyErr"]

[[errors]]
name = "DummyErr2"
"#;
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );
}

#[test]
fn test_categories_list_cannot_be_empty() {
    log_init();

    let s = r#"
categories = []
"#;

    assert_eq!(TomlParser::parse_str(s).unwrap_err().kind(), EMPTY_LIST);
}

#[test]
fn test_category_errors_list_mandatory_in_categories() {
    log_init();

    let s = r#"
[[categories]]
name = "Custom"
doc = "Custom category."
doc_from_display = false
"#;
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );

    let s = r#"
[[categories]]
name = "Custom"
doc = "Custom category."
doc_from_display = false
errors = []
"#;
    assert_eq!(TomlParser::parse_str(s).unwrap_err().kind(), EMPTY_LIST);
}

#[test]
fn test_category_name_mandatory_in_categories_list() {
    log_init();

    let s = r#"
[[categories]]
doc = "Category without name."
doc_from_display = true
errors = ["DummyErr"]
"#;
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );
}

#[test]
fn test_category_multiple_categories() {
    log_init();

    let s = r#"
[[categories]]
name = "Cat1"
doc = "First category."
doc_from_display = false
errors = ["DummyErr"]

[[categories]]
name = "Cat2"
doc_from_display = true
errors = ["DummyErr2"]
"#;

    let cat1 = CategorySpec {
        name: "Cat1".into(),
        doc: Some("First category.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(false),
        },
        errors: vec![ErrorSpec {
            name: "DummyErr".into(),
            ..Default::default()
        }],
    };

    let cat2 = CategorySpec {
        name: "Cat2".into(),
        oes: OverridableErrorSpec {
            doc_from_display: Some(true),
        },
        errors: vec![ErrorSpec {
            name: "DummyErr2".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let spec = Spec {
        modules: vec![ModuleSpec {
            categories: vec![cat1, cat2],
            ..Default::default()
        }],
        ..Default::default()
    };

    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_category_name_uniqueness() {
    log_init();

    let s = r#"
[[categories]]
name = "Cat1"
doc = "First category."
doc_from_display = false
errors = ["DummyErr1"]

[[categories]]
name = "Cat1"
doc_from_display = false
errors = ["DummyErr2"]

[[categories]]
name = "Cat2"
errors = ["DummyErr2"]

[[categories]]
name = "Cat2"
errors = ["DummyErr3"]

"#;
    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        NON_UNIQUE_NAME
    );
}

#[test]
fn test_multiple_modules() {
    log_init();

    let s = r#"
[[modules]]
name = "mod_1"

[[modules.categories]]
name = "M1C1"
errors = ["M1C1E1"]

[[modules.categories]]
name = "M1C2"
errors = ["M1C2E1"]

[[modules]]
name = "mod_2"

[[modules.categories]]
name = "M2C1"
errors = ["M2C1E1"]

[[modules.categories]]
name = "M2C2"
errors = ["M2C2E1", "M2C2E2"]
"#;

    let spec = Spec {
        modules: vec![
            ModuleSpec {
                name: Some("mod_1".into()),
                categories: vec![
                    CategorySpec {
                        name: "M1C1".into(),
                        errors: vec![ErrorSpec {
                            name: "M1C1E1".into(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    CategorySpec {
                        name: "M1C2".into(),
                        errors: vec![ErrorSpec {
                            name: "M1C2E1".into(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ModuleSpec {
                name: Some("mod_2".into()),
                categories: vec![
                    CategorySpec {
                        name: "M2C1".into(),
                        errors: vec![ErrorSpec {
                            name: "M2C1E1".into(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    CategorySpec {
                        name: "M2C2".into(),
                        errors: vec![
                            ErrorSpec {
                                name: "M2C2E1".into(),
                                ..Default::default()
                            },
                            ErrorSpec {
                                name: "M2C2E2".into(),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let res = TomlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_errors_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
[[modules]]
name = "my_module"

[[modules.categories]]
name = "General"
errors = ["MyError"]

[[errors]]
name = "MyOtherError"
"#;

    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
[[errors]]
name = "MyOtherError"
"#;

    assert!(TomlParser::parse_str(s).is_ok());
}

#[test]
fn test_category_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
[[modules]]
name = "my_module"

[[modules.categories]]
name = "General"
errors = ["MyError"]

[category]
name = "OtherCategory"
"#;

    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
[[modules]]
name = "my_module"

[[modules.categories]]
name = "General"
errors = ["MyError"]
"#;

    assert!(TomlParser::parse_str(s).is_ok());
}

#[test]
fn test_categories_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
[[modules]]
name = "my_module"

[[modules.categories]]
name = "General"
errors = ["MyError"]

[[categories]]
name = "OtherCategory"
errors = ["OtherError"]
"#;

    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
[[categories]]
name = "OtherCategory"
errors = ["OtherError"]
"#;

    assert!(TomlParser::parse_str(s).is_ok());
}

#[test]
fn test_module_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
[[modules]]
name = "my_module"

[[modules.categories]]
name = "General"
errors = ["MyError"]

[module]
name = "my_module"
"#;

    assert_eq!(
        TomlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
[[modules]]
name = "my_module"

[[modules.categories]]
name = "General"
errors = ["MyError"]
"#;

    assert!(TomlParser::parse_str(s).is_ok());
}
