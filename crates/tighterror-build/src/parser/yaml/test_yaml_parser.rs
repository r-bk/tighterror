use crate::{errors::BAD_YAML, parser::yaml::*};

fn log_init() {
    env_logger::builder().is_test(true).try_init().ok();
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
    assert!(matches!(res, Err(e) if e == BAD_YAML));
}
