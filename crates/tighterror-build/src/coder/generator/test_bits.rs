use super::bits::Bits;
use crate::{
    errors::kind::REPR_TOO_NARROW,
    spec::{CategorySpec, ErrorSpec, ModuleSpec, ReprType, Spec},
};

// A module with one category of `n` errors, so that its kinds need
// log2(n) bits, rounded up, and with the given `repr`.
fn module(n: usize, repr: Option<ReprType>) -> ModuleSpec {
    let errors = (0..n)
        .map(|i| ErrorSpec {
            name: format!("E{i}"),
            ..Default::default()
        })
        .collect();
    ModuleSpec {
        name: "errors".into(),
        repr,
        categories: vec![CategorySpec {
            name: "General".into(),
            errors,
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn repr_type(n: usize, repr: Option<ReprType>) -> ReprType {
    Bits::calculate(&Spec::default(), &module(n, repr))
        .expect("bits are calculated")
        .repr_type
}

// Checks the underlying type of a module: without `repr` the smallest type
// that holds the kinds, with `repr` the set type whenever the kinds fit.
#[test]
fn test_repr_type() {
    // 256 kinds fill a u8, and 257 need a u16
    assert_eq!(repr_type(256, None), ReprType::U8);
    assert_eq!(repr_type(257, None), ReprType::U16);

    // a set type wider than the kinds need is kept
    assert_eq!(repr_type(2, Some(ReprType::U64)), ReprType::U64);

    // a set type that the kinds fill exactly is kept
    assert_eq!(repr_type(256, Some(ReprType::U8)), ReprType::U8);
}

// Checks that a `repr` with too few bits for the kinds fails the
// generation. It simulates a module that outgrew its `repr u8`.
#[test]
fn test_repr_too_narrow() {
    let module = module(257, Some(ReprType::U8));
    let err = Bits::calculate(&Spec::default(), &module).unwrap_err();
    assert_eq!(err.kind(), REPR_TOO_NARROW);
}
