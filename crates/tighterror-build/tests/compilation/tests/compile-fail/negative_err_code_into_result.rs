extern crate test_compilation;

use test_compilation::errors::{codes::BAD_FILE, Error};

fn main() {
    let _res: Result<(), Error> = BAD_FILE.into();
    //~^ ERROR the trait bound `Result<(), test_compilation::errors::Error>: From<ErrorCode>` is not satisfied
}
