use crate::{
    errors::BAD_YAML,
    parser::yaml::*,
    spec::{CategorySpec, ErrorSpec, OverrideableErrorSpec, Spec},
};

const GENERAL_CAT: &str = "General";

fn log_init() {
    env_logger::builder().is_test(true).try_init().ok();
}

fn spec_from_err(err: ErrorSpec) -> Spec {
    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: vec![err],
        ..Default::default()
    };

    Spec {
        categories: vec![cat],
        ..Default::default()
    }
}

fn spec_from_err_iter(iter: impl IntoIterator<Item = ErrorSpec>) -> Spec {
    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: Vec::from_iter(iter),
        ..Default::default()
    };

    Spec {
        categories: vec![cat],
        ..Default::default()
    }
}

fn spec_from_main(main: MainSpec) -> Spec {
    let err = ErrorSpec {
        name: "DummyErr".into(),
        ..Default::default()
    };

    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: vec![err],
        ..Default::default()
    };

    Spec {
        main,
        categories: vec![cat],
    }
}

#[test]
fn test_multiple_documents_fails() {
    log_init();
    let s = "
---
meta:

---
errors:
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_YAML.into());
}

#[test]
fn test_minimal() {
    log_init();
    let s = "
---
errors:
    - SingleError
";

    let err = ErrorSpec {
        name: "SingleError".into(),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_minimal_with_display() {
    log_init();
    let s = "
---
errors:
    - SingleError: The single error in the test.
";

    let err = ErrorSpec {
        name: "SingleError".into(),
        display: Some("The single error in the test.".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_minimal_explicit() {
    log_init();
    let s = "
---
errors:
    - name: SingleError
";
    let err = ErrorSpec {
        name: "SingleError".into(),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_err_doc_from_display() {
    log_init();
    let s = "
---
errors:
    - name: TestError
      doc_from_display: false
";
    let err = ErrorSpec {
        name: "TestError".into(),
        oes: OverrideableErrorSpec {
            doc_from_display: Some(false),
        },
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      doc_from_display: true
";
    let err = ErrorSpec {
        name: "TestError".into(),
        oes: OverrideableErrorSpec {
            doc_from_display: Some(true),
        },
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      doc_from_display: yes
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());
}

#[test]
fn test_err_display() {
    let s = "
---
errors:
    - name: TestError
      display: An error occurred.
";
    let err = ErrorSpec {
        name: "TestError".into(),
        display: Some("An error occurred.".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      display: \"\"
";
    let err = ErrorSpec {
        name: "TestError".into(),
        display: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      display: An error occurred.
";
    let err = ErrorSpec {
        name: "TestError".into(),
        display: Some("An error occurred.".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      display: null
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());
}

#[test]
fn test_err_doc() {
    let s = "
---
errors:
    - name: TestError
      doc: An error doc.
";
    let err = ErrorSpec {
        name: "TestError".into(),
        doc: Some("An error doc.".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      doc: |
        A multiline doc.

        Second line.
";
    let err = ErrorSpec {
        name: "TestError".into(),
        doc: Some("A multiline doc.\n\nSecond line.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);

    let s = "
---
errors:
    - name: TestError
      doc: null
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());

    let s = "
---
errors:
    - name: TestError
      doc: 1
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());
}

#[test]
fn test_err_name() {
    // name cannot be an empty string
    let s = "
---
errors:
    - name: \"\"
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());

    // name must be in UpperCamel case
    let s = "
---
errors:
    - notUpperCamel
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());

    // name must be in UpperCamel case
    let s = "
---
errors:
    - name: notUpperCamel
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());

    // name cannot be null
    let s = "
---
errors:
    - name: null
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());

    // name cannot be non String
    let s = "
---
errors:
    - 1
";
    let res = YamlParser::from_str(s);
    assert_eq!(res, BAD_SPEC.into());
}

#[test]
fn test_err() {
    let s = "
---
errors:
    - name: TestError
      doc: An error doc.
      display: An error description.
      doc_from_display: false
";
    let err = ErrorSpec {
        name: "TestError".into(),
        doc: Some("An error doc.".into()),
        display: Some("An error description.".into()),
        oes: OverrideableErrorSpec {
            doc_from_display: Some(false),
        },
    };
    let spec = spec_from_err(err);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_errs() {
    log_init();
    let s = "
---
errors:
    - FirstErr
    - SecondErr: With display.
    - name: TestError
      doc: An error doc.
      display: An error description.
      doc_from_display: false
    - name: Err2
      doc: Another error.
    - name: Err3
      display: A third one.
      doc_from_display: true
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
        oes: OverrideableErrorSpec {
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
        oes: OverrideableErrorSpec {
            doc_from_display: Some(true),
        },
        ..Default::default()
    };
    let spec = spec_from_err_iter([err1, err2, err3, err4, err5]);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(res, spec);
}

#[test]
fn test_top_kws() {
    log_init();
    let s = "
---
my_errors:
    - BadError
";
    assert_eq!(YamlParser::from_str(s).unwrap_err(), BAD_SPEC);

    let s = "
---
tighterror:
  doc_from_display: true

";
    assert_eq!(YamlParser::from_str(s).unwrap_err(), BAD_SPEC);
}

#[test]
fn test_main_doc_from_display() {
    log_init();
    let s = "
---
tighterror:
  doc_from_display: true

errors:
  - DummyErr
";

    let main = MainSpec {
        oes: OverrideableErrorSpec {
            doc_from_display: Some(true),
        },
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  doc_from_display: false

errors:
  - DummyErr
";

    let main = MainSpec {
        oes: OverrideableErrorSpec {
            doc_from_display: Some(false),
        },
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_main_mod_doc() {
    log_init();
    let s = "
---
tighterror:
  mod_doc: |
    Module documentation.

    Multiline.

errors:
  - DummyErr
";
    let main = MainSpec {
        mod_doc: Some("Module documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
    mod_doc: \"\"

errors:
    - DummyErr
";
    let main = MainSpec {
        mod_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_main_cat_doc() {
    log_init();
    let s = "
---
tighterror:
  cat_doc: |
    Category documentation.

    Multiline.

errors:
  - DummyErr
";
    let main = MainSpec {
        cat_doc: Some("Category documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
    cat_doc: \"\"

errors:
    - DummyErr
";
    let main = MainSpec {
        cat_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_main_err_doc() {
    log_init();
    let s = "
---
tighterror:
  err_doc: |
    Error documentation.

    Multiline.

errors:
  - DummyErr
";
    let main = MainSpec {
        err_doc: Some("Error documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
    err_doc: \"\"

errors:
    - DummyErr
";
    let main = MainSpec {
        err_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_main_err_code_doc() {
    log_init();
    let s = "
---
tighterror:
  err_code_doc: |
    ErrorCode documentation.

    Multiline.

errors:
  - DummyErr
";
    let main = MainSpec {
        err_code_doc: Some("ErrorCode documentation.\n\nMultiline.\n".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
    err_code_doc: \"\"

errors:
    - DummyErr
";
    let main = MainSpec {
        err_code_doc: Some("".into()),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);
}

#[test]
fn test_main_err_into_result() {
    log_init();
    let s = "
---
tighterror:
  err_into_result: true

errors:
  - DummyErr
";

    let main = MainSpec {
        err_into_result: Some(true),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  err_into_result: false

errors:
  - DummyErr
";

    let main = MainSpec {
        err_into_result: Some(false),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  err_into_result: yes

errors:
  - DummyErr
";

    assert_eq!(YamlParser::from_str(s), BAD_SPEC.into());
}

#[test]
fn test_main_err_code_into_result() {
    log_init();
    let s = "
---
tighterror:
  err_code_into_result: true

errors:
  - DummyErr
";

    let main = MainSpec {
        err_code_into_result: Some(true),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  err_code_into_result: false

errors:
  - DummyErr
";

    let main = MainSpec {
        err_code_into_result: Some(false),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  err_code_into_result: yes

errors:
  - DummyErr
";

    assert_eq!(YamlParser::from_str(s), BAD_SPEC.into());
}

#[test]
fn test_error_trait() {
    log_init();
    let s = "
---
tighterror:
  error_trait: true

errors:
  - DummyErr
";

    let main = MainSpec {
        error_trait: Some(true),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  error_trait: false

errors:
  - DummyErr
";

    let main = MainSpec {
        error_trait: Some(false),
        ..Default::default()
    };
    let spec = spec_from_main(main);
    let res = YamlParser::from_str(s).unwrap();
    assert_eq!(spec, res);

    let s = "
---
tighterror:
  error_trait: yes

errors:
  - DummyErr
";

    assert_eq!(YamlParser::from_str(s), BAD_SPEC.into());
}
