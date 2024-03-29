use crate::{
    coder::idents,
    errors::kinds::{BAD_SPEC, BAD_TOML},
    parser::{
        testing::{log_init, spec_from_err, spec_from_err_iter, spec_from_main, spec_from_module},
        toml::*,
    },
    spec::{ErrorSpec, OverridableErrorSpec},
};

const GOOD_BOOLS: [(&str, bool); 2] = [("true", true), ("false", false)];
const BAD_BOOLS: [&str; 8] = ["yes", "tr ue", "1", "on", "null", "True", "False", "no"];
const BAD_IDENTS: [&str; 7] = [
    "\"notUpperCamelCase\"",
    "\"With Spaces\"",
    "\"\"",
    "\"  \"",
    "\" PaddedWithSpaces  \"",
    "\"Disallowed-Character-\"",
    "\"null\"",
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
    let res = TomlParser::from_str(s).unwrap();
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
        let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_err_doc_from_display() {
    log_init();

    for good in GOOD_BOOLS {
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
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    for bad in BAD_BOOLS {
        let s = format!(
            "[[errors]]\nname = \"TestError\"\ndoc_from_display = {}",
            bad
        );

        let res = TomlParser::from_str(&s);
        assert!(res == BAD_TOML.into() || res == BAD_SPEC.into());
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
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    #[allow(clippy::single_element_loop)]
    for bad in ["1"] {
        let s = format!("[[errors]]\nname = \"TestError\"\ndisplay = {}", bad);
        let res = TomlParser::from_str(&s);
        assert_eq!(res, BAD_SPEC.into());
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
        let res = TomlParser::from_str(&s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    #[allow(clippy::single_element_loop)]
    for bad in ["1"] {
        let s = format!("[[errors]]\nname = \"TestError\"\ndoc = {}", bad);
        let res = TomlParser::from_str(&s);
        assert_eq!(res, BAD_SPEC.into());
    }
}

#[test]
fn test_err_name() {
    log_init();

    const BAD_NAMES: &[&str] = &[
        "\"\"",
        "\"  \"",
        "\"camelCase\"",
        "1",
        "\"With Spaces\"",
        "\"With-Dashes\"",
        "\"CAPITAL_LETTERS\"",
    ];

    for bad in BAD_NAMES {
        let s = format!("[[errors]]\nname = {}", bad);
        let res = TomlParser::from_str(&s);
        assert_eq!(res, BAD_SPEC.into());
    }

    for bad in BAD_NAMES {
        let s = format!("errors = [{}]", bad);
        let res = TomlParser::from_str(&s);
        assert_eq!(res, BAD_SPEC.into());
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
        let res = TomlParser::from_str(s).unwrap();
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
        let res = TomlParser::from_str(s).unwrap();
        assert_eq!(res, spec);
    }
}

#[test]
fn test_top_kws() {
    log_init();
    let s = "
my_errors = [\"BadError\"]
";
    assert_eq!(TomlParser::from_str(s).unwrap_err(), BAD_SPEC.into());

    let s = "
[module]
doc_from_display = true

";
    assert_eq!(TomlParser::from_str(s).unwrap_err(), BAD_SPEC.into());
}

#[test]
fn test_module_doc_from_display() {
    log_init();

    for good in GOOD_BOOLS {
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
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLS {
        let s = format!(
            "[module]\ndoc_from_display = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        let err = TomlParser::from_str(&s).unwrap_err();
        assert!(matches!(err.kind(), BAD_SPEC | BAD_TOML));
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
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
    let res = TomlParser::from_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_result_from_err() {
    log_init();

    for good in GOOD_BOOLS {
        let s = format!(
            "[module]\nresult_from_err = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            result_from_err: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLS {
        let s = format!(
            "[module]\nresult_from_err = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        let err = TomlParser::from_str(&s).unwrap_err();
        assert!(matches!(err.kind(), BAD_SPEC | BAD_TOML));
    }
}

#[test]
fn test_module_result_from_err_kind() {
    log_init();

    for good in GOOD_BOOLS {
        let s = format!(
            "[module]\nresult_from_err_kind = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            result_from_err_kind: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLS {
        let s = format!(
            "[module]\nresult_from_err_kind = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        let err = TomlParser::from_str(&s).unwrap_err();
        assert!(matches!(err.kind(), BAD_SPEC | BAD_TOML));
    }
}

#[test]
fn test_error_trait() {
    log_init();

    for good in GOOD_BOOLS {
        let s = format!(
            "[module]\nerror_trait = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let module = ModuleSpec {
            error_trait: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLS {
        let s = format!(
            "[module]\nerror_trait = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        let err = TomlParser::from_str(&s).unwrap_err();
        assert!(matches!(err.kind(), BAD_SPEC | BAD_TOML));
    }
}

#[test]
fn test_no_std() {
    log_init();

    for good in GOOD_BOOLS {
        let s = format!(
            "[main]\nno_std = {}\n\n[[errors]]\nname = \"DummyErr\"",
            good.0
        );
        let main = MainSpec {
            no_std: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_main(main);
        let res = TomlParser::from_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLS {
        let s = format!(
            "[main]\nno_std = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        let err = TomlParser::from_str(&s).unwrap_err();
        assert!(matches!(err.kind(), BAD_SPEC | BAD_TOML));
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
        let res = TomlParser::from_str(&format!(
            "[module]\nerr_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_IDENTS {
        let s = format!(
            "[module]\nerr_name = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::from_str(&s), BAD_SPEC.into());
    }

    for bad in [idents::ERROR_CATEGORY, idents::ERROR_KIND] {
        let s = format!(
            "[module]\nerr_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::from_str(&s), BAD_SPEC.into());
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
        let res = TomlParser::from_str(&format!(
            "[module]\nerr_kind_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_IDENTS {
        let s = format!(
            "[module]\nerr_kind_name = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::from_str(&s), BAD_SPEC.into());
    }

    for bad in [idents::ERROR, idents::ERROR_CATEGORY] {
        let s = format!(
            "[module]\nerr_kind_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::from_str(&s), BAD_SPEC.into());
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
        let res = TomlParser::from_str(&format!(
            "[module]\nerr_cat_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_IDENTS {
        let s = format!(
            "[module]\nerr_cat_name = {}\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::from_str(&s), BAD_SPEC.into());
    }

    for bad in [idents::ERROR, idents::ERROR_KIND] {
        let s = format!(
            "[module]\nerr_cat_name = \"{}\"\n\n[[errors]]\nname = \"DummyErr\"",
            bad
        );
        assert_eq!(TomlParser::from_str(&s), BAD_SPEC.into());
    }
}

#[test]
fn test_error_list_unique_names() {
    log_init();

    let s = "errors = [\"FirstError\",  \"FirstError\", \"SecondError\"]";
    assert_eq!(TomlParser::from_str(s), BAD_SPEC.into());

    let s = "errors = [\"FirstError\",  \"SecondError\"]";
    assert!(TomlParser::from_str(s).is_ok());
}
