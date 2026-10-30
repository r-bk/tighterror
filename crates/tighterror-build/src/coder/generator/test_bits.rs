use super::bits::Bits;
use crate::{
    errors::kind::REPR_TOO_NARROW,
    spec::{CategorySpec, ErrorSpec, ModuleSpec, ReprType, Spec},
};

// A module with a category per entry of `sizes`, each with that many errors,
// and with the given `repr`.
fn module(sizes: &[usize], repr: Option<ReprType>) -> ModuleSpec {
    let categories = sizes
        .iter()
        .enumerate()
        .map(|(c, &n)| CategorySpec {
            name: format!("C{c}"),
            errors: (0..n)
                .map(|i| ErrorSpec {
                    name: format!("C{c}_E{i}"),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        })
        .collect();
    ModuleSpec {
        name: "errors".into(),
        repr,
        categories,
        ..Default::default()
    }
}

fn bits(sizes: &[usize], repr: Option<ReprType>) -> Bits {
    Bits::calculate(&Spec::default(), &module(sizes, repr)).expect("bits are calculated")
}

fn repr_type(sizes: &[usize], repr: Option<ReprType>) -> ReprType {
    bits(sizes, repr).repr_type
}

// Checks the number of variant bits on both sides of every edge up to the
// one between a u16 and a u32. It simulates a single category of 2^k - 1
// errors, which fill k bits together with the unused variant 0, and one of
// 2^k errors.
#[test]
fn test_variant_bits() {
    for k in 1..=16 {
        // the errors and the unused variant 0 fill k bits
        assert_eq!(bits(&[(1 << k) - 1], None).variant, k);

        // one error more needs one bit more
        assert_eq!(bits(&[1 << k], None).variant, k + 1);
    }
}

// Checks the underlying type of a module: without `repr` the smallest type
// that holds the kinds, with `repr` the set type whenever the kinds fit.
#[test]
fn test_repr_type() {
    // 255 kinds fit a u8, and 256 need a u16 because the value 0 is not used
    assert_eq!(repr_type(&[255], None), ReprType::U8);
    assert_eq!(repr_type(&[256], None), ReprType::U16);

    // 65535 kinds fit a u16, and 65536 need a u32
    assert_eq!(repr_type(&[65535], None), ReprType::U16);
    assert_eq!(repr_type(&[65536], None), ReprType::U32);

    // a set type wider than the kinds need is kept
    assert_eq!(repr_type(&[2], Some(ReprType::U64)), ReprType::U64);

    // a set type whose every bit the kinds need is kept
    assert_eq!(repr_type(&[255], Some(ReprType::U8)), ReprType::U8);
}

// Checks that every category takes one variant more, for the unused
// variant 0. It simulates modules whose first or last category is the
// largest one.
#[test]
fn test_full_category() {
    // a full first category needs one variant bit more
    assert_eq!(repr_type(&[128, 1], None), ReprType::U16);

    // a full last category needs one variant bit more
    assert_eq!(repr_type(&[1, 128], None), ReprType::U16);

    // categories one short of full keep the variant bits
    assert_eq!(repr_type(&[127, 127], None), ReprType::U8);
}

// Checks that a `repr` with too few bits for the kinds fails the
// generation. It simulates a module that outgrew its `repr u8`.
#[test]
fn test_repr_too_narrow() {
    let module = module(&[256], Some(ReprType::U8));
    let err = Bits::calculate(&Spec::default(), &module).unwrap_err();
    assert_eq!(err.kind(), REPR_TOO_NARROW);
}
