//! This is a test crate to check `tighterror.yaml`
//! with multiple modules.

#![deny(missing_docs)]
#![deny(warnings)]

/// Public errors.
pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

/// Internal errors.
pub mod internal_errors {
    include!(concat!(env!("OUT_DIR"), "/internal_errors.rs"));
}

/// FlatKinds module.
pub mod flat_kinds_mod {
    include!(concat!(env!("OUT_DIR"), "/flat_kinds_mod.rs"));
}

/// FlatKinds module without display attributes.
pub mod flat_kinds_mod_without_display {
    include!(concat!(
        env!("OUT_DIR"),
        "/flat_kinds_mod_without_display.rs"
    ));
}

#[cfg(test)]
mod tests {
    use crate::{errors, flat_kinds_mod, internal_errors};

    #[test]
    fn test_kind_constants_are_placed_in_different_modules() {
        assert_ne!(
            errors::kind::parsing::QUEUE_FULL,
            errors::kind::processing::QUEUE_FULL
        );
        assert_ne!(
            internal_errors::kind::parser::BAD_FILE,
            internal_errors::kind::processor::BAD_FILE,
        );
    }

    #[test]
    fn test_variant_types() {
        assert_eq!(
            format!("{}", flat_kinds_mod::variant::types::CatOneErrOne),
            "CatOne error #1"
        );
        assert_eq!(
            format!("{}", flat_kinds_mod::variant::types::CatOneWithoutDisplay),
            "CatOneWithoutDisplay"
        );
        assert_eq!(
            format!("{}", flat_kinds_mod::variant::types::CatTwoSpecial),
            "CatTwoSpecial"
        );
    }
}
