#![deny(warnings)]

// Checks that a module generated with `result_from_err`,
// `result_from_err_kind` and `error_trait` disabled leaves out the
// conversions and the trait impl those settings control. Every case under
// `compile-fail` must fail to compile with the errors in its `.stderr` file.
#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
