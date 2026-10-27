use super::reset_dir;
use std::fs;

// Checks that the output directory of a build script keeps no file from an
// earlier run. It simulates a module removed from the spec, whose file the
// earlier run wrote.
#[test]
fn test_reset_dir_removes_files_of_earlier_runs() {
    // an output directory with a file that an earlier run wrote
    let tmp = tempfile::tempdir().expect("temporary directory is created");
    let dir = tmp.path().join("tighterror");
    fs::create_dir(&dir).expect("output directory is created");
    fs::write(dir.join("removed.rs"), "pub const X: u8 = 0;").expect("file is written");

    // reset it, as a run does before writing the modules
    reset_dir(&dir).expect("output directory is reset");

    // the directory is still there, and empty
    assert!(dir.is_dir());
    assert_eq!(
        fs::read_dir(&dir).expect("directory is readable").count(),
        0
    );
}

// Checks that the first run of a build script creates the output directory.
#[test]
fn test_reset_dir_creates_a_missing_directory() {
    // a path that doesn't exist yet
    let tmp = tempfile::tempdir().expect("temporary directory is created");
    let dir = tmp.path().join("tighterror");

    // reset it, as the first run does
    reset_dir(&dir).expect("output directory is created");

    // the directory is created
    assert!(dir.is_dir());
}
