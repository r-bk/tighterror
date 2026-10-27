// Checks that `error_trait: false` leaves out the `std::error::Error` impl
// of `Error`.

use test_compilation::errors::{kind::general::BAD_FILE, Error};

fn foo<T: std::error::Error>(e: T) -> String {
    format!("{e}")
}

fn main() {
    // Pass an `Error` where `std::error::Error` is required.
    let e = Error::from(BAD_FILE);
    foo(e);
}
