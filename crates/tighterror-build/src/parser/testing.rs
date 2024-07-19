use crate::spec::{CategorySpec, ErrorSpec, MainSpec, ModuleSpec, Spec};

pub const GENERAL_CAT: &str = "General";

pub fn log_init() {
    env_logger::builder().is_test(true).try_init().ok();
}

pub fn spec_from_err(err: ErrorSpec) -> Spec {
    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: vec![err],
        ..Default::default()
    };

    let module = ModuleSpec {
        categories: vec![cat],
        ..Default::default()
    };

    Spec {
        modules: vec![module],
        ..Default::default()
    }
}

pub fn spec_from_err_iter(iter: impl IntoIterator<Item = ErrorSpec>) -> Spec {
    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: Vec::from_iter(iter),
        ..Default::default()
    };

    let module = ModuleSpec {
        categories: vec![cat],
        ..Default::default()
    };

    Spec {
        modules: vec![module],
        ..Default::default()
    }
}

pub fn spec_from_module(mut module: ModuleSpec) -> Spec {
    let err = ErrorSpec {
        name: "DUMMY_ERR".into(),
        ..Default::default()
    };

    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: vec![err],
        ..Default::default()
    };

    module.categories = vec![cat];

    Spec {
        modules: vec![module],
        ..Default::default()
    }
}

pub fn spec_from_main(main: MainSpec) -> Spec {
    let err = ErrorSpec {
        name: "DUMMY_ERR".into(),
        ..Default::default()
    };

    let cat = CategorySpec {
        name: GENERAL_CAT.into(),
        errors: vec![err],
        ..Default::default()
    };

    let module = ModuleSpec {
        categories: vec![cat],
        ..Default::default()
    };

    Spec {
        main,
        modules: vec![module],
        ..Default::default()
    }
}

pub fn spec_from_category(mut cat: CategorySpec) -> Spec {
    let err = ErrorSpec {
        name: "DUMMY_ERR".into(),
        ..Default::default()
    };
    cat.errors = vec![err];

    let module = ModuleSpec {
        categories: vec![cat],
        ..Default::default()
    };

    Spec {
        modules: vec![module],
        ..Default::default()
    }
}
