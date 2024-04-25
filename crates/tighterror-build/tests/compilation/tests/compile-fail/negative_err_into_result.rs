extern crate test_compilation;

use test_compilation::errors::{kinds::general::BAD_FILE, Error};

fn main() {
    let _res: Result<(), Error> = Error::from(BAD_FILE).into();
    //~^ ERROR the trait bound `Result<(), test_compilation::errors::Error>: From<test_compilation::errors::Error>` is not satisfied
}
