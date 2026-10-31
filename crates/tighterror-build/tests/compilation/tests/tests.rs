#![deny(warnings)]

// Checks that a module generated with `result_from_err`,
// `result_from_err_kind` and `error_trait` disabled leaves out the
// conversions and the trait impl those settings control, and that error
// kinds of more than 64 bits name the limit. Every case under
// `compile-fail` must fail to compile with the errors in its `.stderr` file.
#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
