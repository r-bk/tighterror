// Checks that `result_from_err_kind: false` leaves out the conversion of an
// `ErrorKind` into `Result<T, Error>`.

use test_compilation::errors::{kind::general::BAD_FILE, Error};

fn main() {
    // Convert an `ErrorKind` into a `Result` through `Into`.
    let _res: Result<(), Error> = BAD_FILE.into();
}
