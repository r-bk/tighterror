extern crate test_compilation;

use test_compilation::errors::{kinds::general::BAD_FILE, Error};

fn foo<T: std::error::Error>(e: T) -> String {
    format!("{e}")
}

fn main() {
    let e = Error::from(BAD_FILE);
    foo(e);
    //~^ ERROR the trait bound `test_compilation::errors::Error: std::error::Error` is not satisfied
}
