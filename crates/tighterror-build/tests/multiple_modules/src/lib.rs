//! This is a test crate to check `tighterror.kdl`
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

    // Checks that user code can handle a whole category with one match arm
    // on a category constant, through the error's own `category` method. It
    // simulates a caller that reacts to the group of an error, in a module
    // with the nested layout and in one with the flat layout.
    #[test]
    fn test_match_on_category() {
        // map an error of the nested module to its group
        let nested = |e: errors::McError| match e.category() {
            errors::category::PARSING => "parsing",
            errors::category::PROCESSING => "processing",
            _ => "other",
        };
        assert_eq!(nested(errors::kind::parsing::QUEUE_FULL.into()), "parsing");
        assert_eq!(
            nested(errors::kind::processing::QUEUE_FULL.into()),
            "processing"
        );
        assert_eq!(nested(errors::kind::general::TIMEOUT.into()), "other");

        // map an error of the flat module to its group
        let flat = |e: flat_kinds_mod::Error| match e.category() {
            flat_kinds_mod::category::CAT_ONE => 1,
            flat_kinds_mod::category::CAT_TWO => 2,
            _ => 0,
        };
        assert_eq!(flat(flat_kinds_mod::kind::CAT_ONE_ERR_TWO.into()), 1);
        assert_eq!(flat(flat_kinds_mod::kind::CAT_TWO_ERR_ONE.into()), 2);
    }

    // Checks that user code can handle single errors, alone or several in
    // one arm, with match arms on kind constants. It simulates a caller that
    // tells apart errors whose names repeat across categories of a nested
    // module, and errors of a flat module.
    #[test]
    fn test_match_on_kind() {
        // tell apart the two QUEUE_FULL kinds of the nested module
        let nested = |e: errors::McError| match e.kind() {
            errors::kind::parsing::QUEUE_FULL => "parsing queue",
            errors::kind::processing::QUEUE_FULL => "processing queue",
            errors::kind::parsing::BAD_TOKEN | errors::kind::processing::OUT_OF_MEMORY => "fatal",
            _ => "other",
        };
        assert_eq!(
            nested(errors::kind::parsing::QUEUE_FULL.into()),
            "parsing queue"
        );
        assert_eq!(
            nested(errors::kind::processing::QUEUE_FULL.into()),
            "processing queue"
        );
        assert_eq!(nested(errors::kind::parsing::BAD_TOKEN.into()), "fatal");
        assert_eq!(
            nested(errors::kind::processing::OUT_OF_MEMORY.into()),
            "fatal"
        );
        assert_eq!(nested(errors::kind::general::TIMEOUT.into()), "other");

        // group kinds of the flat module across categories
        let flat = |e: flat_kinds_mod::Error| match e.kind() {
            flat_kinds_mod::kind::CAT_ONE_ERR_ONE | flat_kinds_mod::kind::CAT_TWO_ERR_ONE => {
                "first"
            }
            flat_kinds_mod::kind::CAT_ONE_ERR_TWO => "second",
            _ => "other",
        };
        assert_eq!(flat(flat_kinds_mod::kind::CAT_ONE_ERR_ONE.into()), "first");
        assert_eq!(flat(flat_kinds_mod::kind::CAT_TWO_ERR_ONE.into()), "first");
        assert_eq!(flat(flat_kinds_mod::kind::CAT_ONE_ERR_TWO.into()), "second");
        assert_eq!(flat(flat_kinds_mod::kind::CAT_TWO_ERR_TWO.into()), "other");
    }

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
