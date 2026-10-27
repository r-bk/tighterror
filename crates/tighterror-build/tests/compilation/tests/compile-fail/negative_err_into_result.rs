// Checks that `result_from_err: false` leaves out the conversion of an
// `Error` into `Result<T, Error>`.

use test_compilation::errors::{kind::general::BAD_FILE, Error};

fn main() {
    // Convert an `Error` into a `Result` through `Into`.
    let _res: Result<(), Error> = Error::from(BAD_FILE).into();
}
