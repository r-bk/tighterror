use crate::{
    coder::idents,
    errors::{kinds::parser::*, TbErrorKind},
    parser::{
        testing::{
            log_init, spec_from_category, spec_from_err, spec_from_err_iter, spec_from_main,
            spec_from_module,
        },
        yaml::*,
    },
    spec::{ErrorSpec, OverridableErrorSpec, IMPLICIT_CATEGORY_NAME},
};

const GOOD_BOOLEANS: [(&str, bool); 4] = [
    ("true", true),
    ("false", false),
    ("True", true),
    ("False", false),
];
const BAD_BOOLEANS: [&str; 5] = ["yes", "tr ue", "1", "on", "null"];
const BAD_IDENTS: [(&str, TbErrorKind); 7] = [
    ("notUpperCamelCase", BAD_IDENTIFIER_CASE),
    ("With Spaces", BAD_IDENTIFIER_CHARACTERS),
    ("\"\"", EMPTY_IDENTIFIER),
    ("\"  \"", BAD_IDENTIFIER_CHARACTERS),
    ("\" PaddedWithSpaces  \"", BAD_IDENTIFIER_CHARACTERS),
    ("Disallowed-Character-", BAD_IDENTIFIER_CHARACTERS),
    ("null", BAD_VALUE_TYPE),
];

#[test]
fn test_multiple_documents_fails() {
    log_init();
    let s = "
---
meta:

---
errors:
";
    let res = YamlParser::parse_str(s);
    assert_eq!(res, BAD_YAML.into());
}

#[test]
fn test_minimal() {
    log_init();
    let s = "
---
errors:
    - SINGLE_ERROR
";

    let err = ErrorSpec {
        name: "SINGLE_ERROR".into(),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_minimal_with_display() {
    log_init();
    let s = "
---
errors:
    - SINGLE_ERROR: The single error in the test.
";

    let err = ErrorSpec {
        name: "SINGLE_ERROR".into(),
        display: Some("The single error in the test.".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_minimal_explicit() {
    log_init();
    let s = "
---
errors:
    - name: SINGLE_ERROR
";
    let err = ErrorSpec {
        name: "SINGLE_ERROR".into(),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_err_doc_from_display() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\nerrors:\n  - name: TEST_ERROR\n    doc_from_display: {}",
            good.0
        );
        let err = ErrorSpec {
            name: "TEST_ERROR".into(),
            oes: OverridableErrorSpec {
                doc_from_display: Some(good.1),
            },
            ..Default::default()
        };
        let spec = spec_from_err(err);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\nerrors:\n  - name: TEST_ERROR\n    doc_from_display: {}",
            bad
        );

        let res = YamlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), BAD_VALUE_TYPE);
    }
}

#[test]
fn test_err_display() {
    log_init();
    for good in [
        ("An error occurred.", "An error occurred."),
        ("\"\"", ""),
        ("\"1\"", "1"),
    ] {
        let s = format!(
            "---\nerrors:\n  - name: TEST_ERROR\n    display: {}",
            good.0
        );
        let err = ErrorSpec {
            name: "TEST_ERROR".into(),
            display: Some(good.1.into()),
            ..Default::default()
        };
        let spec = spec_from_err(err);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    for bad in ["null", "1"] {
        let s = format!(
            "---\nerrors:\n  - name: TEST_ERROR\n    doc_from_display: {}",
            bad
        );

        let res = YamlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), BAD_VALUE_TYPE);
    }
}

#[test]
fn test_err_doc() {
    log_init();
    for good in [
        ("An error doc.", "An error doc."),
        ("\"\"", ""),
        ("\"1\"", "1"),
    ] {
        let s = format!("---\nerrors:\n  - name: TEST_ERROR\n    doc: {}", good.0);
        let err = ErrorSpec {
            name: "TEST_ERROR".into(),
            doc: Some(good.1.into()),
            ..Default::default()
        };
        let spec = spec_from_err(err);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(res, spec);
    }

    let s = "
---
errors:
  - name: TEST_ERROR
    doc: |
      A multiline doc.

      Second line.
";
    let err = ErrorSpec {
        name: "TEST_ERROR".into(),
        doc: Some("A multiline doc.\n\nSecond line.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);

    for bad in ["null", "1"] {
        let s = format!("---\nerrors:\n  - name: TEST_ERROR\n    doc: {}", bad);

        let res = YamlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), BAD_VALUE_TYPE);
    }
}

#[test]
fn test_err_name() {
    log_init();

    const BAD_NAMES: &[(&str, TbErrorKind)] = &[
        ("\"\"", EMPTY_IDENTIFIER),
        ("\"  \"", BAD_IDENTIFIER_CHARACTERS),
        ("camelCase", BAD_IDENTIFIER_CASE),
        ("null", BAD_VALUE_TYPE),
        ("1", BAD_VALUE_TYPE),
        ("With Spaces", BAD_IDENTIFIER_CHARACTERS),
        ("With-Dashes", BAD_IDENTIFIER_CHARACTERS),
        ("UpperCamelCase", BAD_IDENTIFIER_CASE),
        ("BadChars+", BAD_IDENTIFIER_CHARACTERS),
    ];

    for (bad, kind) in BAD_NAMES {
        let s = format!("---\nerrors:\n  - name: {}", bad);
        let res = YamlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), *kind);
    }

    for (bad, kind) in BAD_NAMES {
        let s = format!("---\nerrors:\n  - {}", bad);
        let res = YamlParser::parse_str(&s);
        assert_eq!(res.unwrap_err().kind(), *kind);
    }
}

#[test]
fn test_err() {
    log_init();
    let s = "
---
errors:
    - name: TEST_ERROR
      doc: An error doc.
      display: An error description.
      doc_from_display: false
";
    let err = ErrorSpec {
        name: "TEST_ERROR".into(),
        doc: Some("An error doc.".into()),
        display: Some("An error description.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(false),
        },
    };
    let spec = spec_from_err(err);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_errs() {
    log_init();
    let s = "
---
errors:
    - FIRST_ERR
    - SECOND_ERR: With display.
    - name: TEST_ERROR
      doc: An error doc.
      display: An error description.
      doc_from_display: false
    - name: ERR2
      doc: Another error.
    - name: ERR3
      display: A third one.
      doc_from_display: true
";
    let err1 = ErrorSpec {
        name: "FIRST_ERR".into(),
        ..Default::default()
    };
    let err2 = ErrorSpec {
        name: "SECOND_ERR".into(),
        display: Some("With display.".into()),
        ..Default::default()
    };
    let err3 = ErrorSpec {
        name: "TEST_ERROR".into(),
        doc: Some("An error doc.".into()),
        display: Some("An error description.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(false),
        },
    };
    let err4 = ErrorSpec {
        name: "ERR2".into(),
        doc: Some("Another error.".into()),
        ..Default::default()
    };
    let err5 = ErrorSpec {
        name: "ERR3".into(),
        display: Some("A third one.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(true),
        },
        ..Default::default()
    };
    let spec = spec_from_err_iter([err1, err2, err3, err4, err5]);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_top_kws() {
    log_init();
    let s = "
---
my_errors:
    - BAD_ERROR
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err(),
        BAD_ROOT_LEVEL_KEYWORD.into()
    );

    let s = "
---
true:
    - BAD_ERROR
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err(),
        BAD_KEYWORD_TYPE.into()
    );

    let s = "
---
module:
  doc_from_display: true

";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err(),
        MISSING_ATTRIBUTE.into()
    );
}

#[test]
fn test_module_doc_from_display() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  doc_from_display: {}\n\nerrors:\n  - DUMMY_ERR",
            good.0
        );
        let module = ModuleSpec {
            oes: OverridableErrorSpec {
                doc_from_display: Some(good.1),
            },
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  doc_from_display: {}\n\nerrors:\n  - DUMMY_ERR",
            bad
        );
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_module_name() {
    log_init();
    for good_name in ["errors", "my_errors"] {
        let s = format!("---\nmodule:\n  name: {good_name}\nerrors:\n  - DUMMY_ERR");
        let module = ModuleSpec {
            name: Some(good_name.into()),
            ..Default::default()
        };
        assert_eq!(spec_from_module(module), YamlParser::parse_str(&s).unwrap());
    }

    for bad_name in ["\"\"", "My_Errors", "ERRORS"] {
        let s = format!("---\nmodule:\n  name: {bad_name}\nerrors:\n  - DUMMY_ERR");
        assert_eq!(YamlParser::parse_str(&s).unwrap_err().kind(), BAD_NAME);
    }
}

#[test]
fn test_module_doc() {
    log_init();
    let s = "
---
module:
  doc: |
    Module documentation.

    Multiline.

errors:
  - DUMMY_ERR
";
    let module = ModuleSpec {
        doc: Some("Module documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
module:
    doc: \"\"

errors:
    - DUMMY_ERR
";
    let module = ModuleSpec {
        doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_err_cat_doc() {
    log_init();
    let s = "
---
module:
  err_cat_doc: |
    Category documentation.

    Multiline.

errors:
  - DUMMY_ERR
";
    let module = ModuleSpec {
        err_cat_doc: Some("Category documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
module:
    err_cat_doc: \"\"

errors:
    - DUMMY_ERR
";
    let module = ModuleSpec {
        err_cat_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_err_doc() {
    log_init();
    let s = "
---
module:
  err_doc: |
    Error documentation.

    Multiline.

errors:
  - DUMMY_ERR
";
    let module = ModuleSpec {
        err_doc: Some("Error documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
module:
    err_doc: \"\"

errors:
    - DUMMY_ERR
";
    let module = ModuleSpec {
        err_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_err_kind_doc() {
    log_init();
    let s = "
---
module:
  err_kind_doc: |
    ErrorKind documentation.

    Multiline.

errors:
  - DUMMY_ERR
";
    let module = ModuleSpec {
        err_kind_doc: Some("ErrorKind documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
module:
    err_kind_doc: \"\"

errors:
    - DUMMY_ERR
";
    let module = ModuleSpec {
        err_kind_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_module(module);
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_module_result_from_err() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  result_from_err: {}\n\nerrors:\n  - DUMMY_ERR",
            good.0
        );
        let module = ModuleSpec {
            result_from_err: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  result_from_err: {}\n\nerrors:\n  - DUMMY_ERR",
            bad
        );
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_module_result_from_err_kind() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  result_from_err_kind: {}\n\nerrors:\n  - DUMMY_ERR",
            good.0
        );
        let module = ModuleSpec {
            result_from_err_kind: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  result_from_err_kind: {}\n\nerrors:\n  - DUMMY_ERR",
            bad
        );
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_error_trait() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  error_trait: {}\n\nerrors:\n  - DUMMY_ERR",
            good.0
        );
        let module = ModuleSpec {
            error_trait: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  error_trait: {}\n\nerrors:\n  - DUMMY_ERR",
            bad
        );
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_module_flat_kinds() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  flat_kinds: {}\n\nerrors:\n  - DUMMY_ERR",
            good.0
        );
        let module = ModuleSpec {
            flat_kinds: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_module(module);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\nmodule:\n  flat_kinds: {}\n\nerrors:\n  - DUMMY_ERR",
            bad
        );
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_module_flat_kinds_error_name_uniqueness() {
    log_init();

    let s = "
---
module:
  flat_kinds: true

categories:
  - name: Cat1
    errors:
      - ERR1: first error

  - name: Cat2
    errors:
      - ERR1: another first error
";

    assert_eq!(YamlParser::parse_str(s), NON_UNIQUE_NAME.into());

    let s = "
---
module:
  flat_kinds: true

categories:
  - name: Cat1
    errors:
      - ERR1: first error

  - name: Cat2
    errors:
      - ERR2: another first error
";

    assert!(YamlParser::parse_str(s).is_ok());
}

#[test]
fn test_module_categories_forbidden_in_single() {
    log_init();

    let s = r#"
---
module:
  name: my_errors
  categories:
    - name: BadCategory
      errors:
        - BAD_ERROR

errors:
  - GOOD_ERROR
"#;

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        BAD_OBJECT_ATTRIBUTE
    );

    let s = r#"
---
module:
  name: my_errors

errors:
  - GOOD_ERROR
"#;

    assert!(YamlParser::parse_str(s).is_ok());
}

#[test]
fn test_no_std() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!("---\nmain:\n  no_std: {}\n\nerrors:\n  - DUMMY_ERR", good.0);
        let main = MainSpec {
            no_std: Some(good.1),
            ..Default::default()
        };
        let spec = spec_from_main(main);
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!("---\nmain:\n  no_std: {}\n\nerrors:\n  - DUMMY_ERR", bad);
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
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
        let res = YamlParser::parse_str(&format!(
            "\n---\nmodule:\n  err_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        let s = format!(
            "\n---\nmodule:\n  err_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            bad
        );
        assert_eq!(YamlParser::parse_str(&s).unwrap_err().kind(), kind);
    }

    for bad in [idents::ERROR_CATEGORY, idents::ERROR_KIND] {
        let s = format!(
            "\n---\nmodule:\n  err_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            bad
        );
        assert_eq!(YamlParser::parse_str(&s), BAD_MODULE_IDENTIFIER.into());
    }

    let s = "---\nerrors:\n  - {}";

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );
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
        let res = YamlParser::parse_str(&format!(
            "\n---\nmodule:\n  err_kind_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        let s = format!(
            "\n---\nmodule:\n  err_kind_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            bad
        );
        assert_eq!(YamlParser::parse_str(&s).unwrap_err().kind(), kind);
    }

    for bad in [idents::ERROR, idents::ERROR_CATEGORY] {
        let s = format!(
            "\n---\nmodule:\n  err_kind_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            bad
        );
        assert_eq!(YamlParser::parse_str(&s), BAD_MODULE_IDENTIFIER.into());
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
        let res = YamlParser::parse_str(&format!(
            "\n---\nmodule:\n  err_cat_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            good
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        let s = format!(
            "\n---\nmodule:\n  err_cat_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            bad
        );
        assert_eq!(YamlParser::parse_str(&s).unwrap_err().kind(), kind);
    }

    for bad in [idents::ERROR, idents::ERROR_KIND] {
        let s = format!(
            "\n---\nmodule:\n  err_cat_name: {}\n\nerrors:\n  - DUMMY_ERR\n",
            bad
        );
        assert_eq!(YamlParser::parse_str(&s), BAD_MODULE_IDENTIFIER.into());
    }
}

#[test]
fn test_error_list_unique_names() {
    log_init();

    let s = "---\nerrors:\n  - FIRST_ERROR\n  - FIRST_ERROR\n  - SECOND_ERROR\n";
    assert_eq!(YamlParser::parse_str(s), NON_UNIQUE_NAME.into());

    let s = "---\nerrors:\n  - FIRST_ERROR\n  - SECOND_ERROR\n";
    assert!(YamlParser::parse_str(s).is_ok());
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
        let res = YamlParser::parse_str(&format!(
            "---\ncategory:\n  name: {good}\nerrors:\n  - DUMMY_ERR\n"
        ))
        .unwrap();
        assert_eq!(spec, res);
    }

    for (bad, kind) in BAD_IDENTS {
        assert_eq!(
            YamlParser::parse_str(&format!(
                "---\ncategory:\n  name: {bad}\nerrors:\n  - DUMMY_ERR\n"
            ))
            .unwrap_err()
            .kind(),
            kind
        );
    }

    for bad in [kws::MAIN, kws::ERRORS] {
        assert_eq!(
            YamlParser::parse_str(&format!(
                "---\ncategory:\n  name: {bad}\nerrors:\n  - DUMMY_ERR\n"
            ))
            .unwrap_err()
            .kind(),
            BAD_IDENTIFIER_CASE
        );
    }
}

#[test]
fn test_category_doc() {
    let s = "
---
category:
  doc: |
    Category long doc string.

    Appears on multiple lines.

errors:
  - DUMMY_ERR
";
    let spec = spec_from_category(CategorySpec {
        name: IMPLICIT_CATEGORY_NAME.into(),
        doc: Some("Category long doc string.\n\nAppears on multiple lines.\n".into()),
        ..Default::default()
    });
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
category:
   doc: \"\"
errors:
  - DUMMY_ERR
";
    let spec = spec_from_category(CategorySpec {
        name: IMPLICIT_CATEGORY_NAME.into(),
        doc: Some("".into()),
        ..Default::default()
    });
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);

    for bad in ["1", "null"] {
        assert_eq!(
            YamlParser::parse_str(&format!(
                "---\ncategory:\n  doc: {bad}\nerrors:\n  - DUMMY_ERR"
            ))
            .unwrap_err()
            .kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_category_doc_from_display() {
    log_init();

    for good in GOOD_BOOLEANS {
        let s = format!(
            "---\ncategory:\n  doc_from_display: {}\n\nerrors:\n  - DUMMY_ERR",
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
        let res = YamlParser::parse_str(&s).unwrap();
        assert_eq!(spec, res);
    }

    for bad in BAD_BOOLEANS {
        let s = format!(
            "---\ncategory:\n  doc_from_display: {}\n\nerrors:\n  - DUMMY_ERR",
            bad
        );
        assert_eq!(
            YamlParser::parse_str(&s).unwrap_err().kind(),
            BAD_VALUE_TYPE
        );
    }
}

#[test]
fn test_category_errors_list_in_single_category() {
    log_init();

    let s = "
---
category:
  name: Custom
  doc: Custom category.
  doc_from_display: false
  errors:
    - DUMMY_ERR

errors:
  - DUMMY_ERR
";

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        BAD_OBJECT_ATTRIBUTE
    );
}

#[test]
fn test_category_errors_and_categories_missing() {
    log_init();

    let s = "
---
category:
  name: Custom
  doc: Custom category.
  doc_from_display: false
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );
}

#[test]
fn test_category_errors_and_categories_mutual_exclusion() {
    log_init();

    let s = "
---
categories:
  - name: Custom
    doc: Custom category.
    doc_from_display: false
    errors:
      - DUMMY_ERR
errors:
  - DUMMY_ERR2
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );
}

#[test]
fn test_categories_list_cannot_be_empty() {
    log_init();

    let s = "
---
categories: []
";

    assert_eq!(YamlParser::parse_str(s).unwrap_err().kind(), EMPTY_LIST);
}

#[test]
fn test_category_errors_list_mandatory_in_categories() {
    log_init();

    let s = "
---
categories:
  - name: Custom
    doc: Custom category.
    doc_from_display: false
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );

    let s = "
---
categories:
  - name: Custom
    doc: Custom category.
    doc_from_display: false
    errors: []
";
    assert_eq!(YamlParser::parse_str(s).unwrap_err().kind(), EMPTY_LIST);
}

#[test]
fn test_category_name_mandatory_in_categories_list() {
    log_init();

    let s = "
---
categories:
  - doc: Category without name.
    doc_from_display: true
    errors:
      - DUMMY_ERR
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MISSING_ATTRIBUTE
    );
}

#[test]
fn test_category_multiple_categories() {
    log_init();

    let s = "
---
categories:
  - name: Cat1
    doc: First category.
    doc_from_display: false
    errors:
      - DUMMY_ERR
  - name: Cat2
    doc_from_display: true
    errors:
      - DUMMY_ERR2
";

    let cat1 = CategorySpec {
        name: "Cat1".into(),
        doc: Some("First category.".into()),
        oes: OverridableErrorSpec {
            doc_from_display: Some(false),
        },
        errors: vec![ErrorSpec {
            name: "DUMMY_ERR".into(),
            ..Default::default()
        }],
    };

    let cat2 = CategorySpec {
        name: "Cat2".into(),
        oes: OverridableErrorSpec {
            doc_from_display: Some(true),
        },
        errors: vec![ErrorSpec {
            name: "DUMMY_ERR2".into(),
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

    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_category_name_uniqueness() {
    log_init();

    let s = "
---
categories:
  - name: Cat1
    doc: First category.
    doc_from_display: false
    errors:
      - DUMMY_ERR
  - name: Cat1
    doc_from_display: true
    errors:
      - DUMMY_ERR2
  - name: Cat2
    errors:
      - DUMMY_ERR
  - name: Cat2
    errors:
      - DUMMY_ERR
";
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        NON_UNIQUE_NAME
    );
}

#[test]
fn test_multiple_modules() {
    log_init();

    let s = "
---
modules:
  - name: mod_1
    categories:
      - name: M1C1
        errors:
          - M1C1E1
      - name: M1C2
        errors:
          - M1C2E1

  - name: mod_2
    categories:
      - name: M2C1
        errors:
          - M2C1E1
      - name: M2C2
        errors:
          - M2C2E1
          - M2C2E2
";

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
    let res = YamlParser::parse_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_errors_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR
errors:
  - MY_OTHER_ERROR
"#;

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR
"#;

    assert!(YamlParser::parse_str(s).is_ok());
}

#[test]
fn test_category_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR
category:
  name: OtherCategory
"#;

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR
"#;

    assert!(YamlParser::parse_str(s).is_ok());
}

#[test]
fn test_categories_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR

categories:
  - name: OtherCategory
    errors:
      - OTHER_ERROR
"#;

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
---
categories:
  - name: OtherCategory
    errors:
      - OTHER_ERROR
"#;

    assert!(YamlParser::parse_str(s).is_ok());
}

#[test]
fn test_module_and_modules_mutual_exclusion() {
    log_init();

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR
module:
  name: my_module
"#;

    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        MUTUALLY_EXCLUSIVE_KEYWORDS
    );

    let s = r#"
---
modules:
  - name: my_module
    categories:
      - name: General
        errors:
          - MY_ERROR
"#;

    assert!(YamlParser::parse_str(s).is_ok());
}

#[test]
fn test_module_name_uniqueness() {
    log_init();

    let s = r#"
---
modules:
  - name: my_errors
    categories:
      - name: Cat1
        doc: First category.
        doc_from_display: false
        errors:
          - DUMMY_ERR
  - name: my_errors
    categories:
      - name: Cat2
        doc: Second category.
        errors:
          - ANOTHER_ERR
"#;
    assert_eq!(
        YamlParser::parse_str(s).unwrap_err().kind(),
        NON_UNIQUE_NAME
    );

    let s = r#"
---
modules:
  - name: my_errors
    categories:
      - name: Cat1
        doc: First category.
        doc_from_display: false
        errors:
          - DUMMY_ERR
  - name: your_errors
    categories:
      - name: Cat2
        doc: Second category.
        errors:
          - ANOTHER_ERR
"#;
    assert!(YamlParser::parse_str(s).is_ok());
}
