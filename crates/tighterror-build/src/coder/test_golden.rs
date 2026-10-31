use super::{FrozenOptions, generator::spec_to_rust};
use crate::parser;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

const BLESS_VAR: &str = "TIGHTERROR_BLESS";
const GOLDEN_DIR: &str = "tests/golden";
const TEST_CRATES_DIR: &str = "tests";
const SPEC_FILE_NAMES: [&str; 1] = ["tighterror.kdl"];
const RUST_EXTENSION: &str = "rs";

// A specification file under test, relative to the crate root, the options
// its crate generates the code with, and the golden file that holds the
// code. With `separate_files` the golden is a directory with a file per
// module. Without `module_doc` the module doc is left out, as BuildOptions
// leaves it out.
struct Case {
    spec: &'static str,
    test: bool,
    separate_files: bool,
    module_doc: bool,
    golden: &'static str,
}

const fn case(spec: &'static str, golden: &'static str) -> Case {
    Case {
        spec,
        test: true,
        separate_files: false,
        module_doc: true,
        golden,
    }
}

// A case of a crate that generates its code with BuildOptions.
const fn build_case(spec: &'static str, golden: &'static str) -> Case {
    Case {
        module_doc: false,
        ..case(spec, golden)
    }
}

const CASES: &[Case] = &[
    Case {
        test: false,
        ..case("tighterror.kdl", "tighterror_build.rs")
    },
    build_case("tests/compilation/tighterror.kdl", "compilation.rs"),
    Case {
        separate_files: true,
        ..case("tests/multiple_modules/tighterror.kdl", "multiple_modules")
    },
    build_case("tests/extra/flat_kinds/tighterror.kdl", "flat_kinds.rs"),
    build_case(
        "tests/extra/full_width_categories/tighterror.kdl",
        "full_width_categories.rs",
    ),
    Case {
        separate_files: true,
        ..build_case("tests/extra/custom_repr/tighterror.kdl", "custom_repr")
    },
    case(
        "tests/extra/custom_type_names/tighterror.kdl",
        "custom_type_names.rs",
    ),
    Case {
        separate_files: true,
        ..build_case("tests/extra/edition_2024/tighterror.kdl", "edition_2024")
    },
    build_case("tests/extra/minimal/tighterror.kdl", "minimal.rs"),
    build_case(
        "tests/extra/minimal_255_variants/tighterror.kdl",
        "minimal_255_variants.rs",
    ),
    build_case(
        "tests/extra/minimal_256_variants/tighterror.kdl",
        "minimal_256_variants.rs",
    ),
    build_case(
        "tests/extra/minimal_no_std/tighterror.kdl",
        "minimal_no_std.rs",
    ),
    build_case(
        "tests/extra/minimal_with_display/tighterror.kdl",
        "minimal_with_display.rs",
    ),
    build_case(
        "tests/extra/multiple_categories/tighterror.kdl",
        "multiple_categories.rs",
    ),
    case(
        "tests/extra/multiple_modules_one_file/tighterror.kdl",
        "multiple_modules_one_file.rs",
    ),
    Case {
        separate_files: true,
        ..build_case("tests/extra/wrap_app/tighterror.kdl", "wrap_app")
    },
    build_case("tests/extra/wrap_render/tighterror.kdl", "wrap_render.rs"),
    build_case("tests/extra/wrap_text/tighterror.kdl", "wrap_text.rs"),
];

fn crate_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn in_repository() -> bool {
    crate_root().join(TEST_CRATES_DIR).join("extra").is_dir()
}

fn bless() -> bool {
    std::env::var(BLESS_VAR).is_ok_and(|v| v == "1")
}

fn generate(c: &Case) -> Vec<(String, String)> {
    let spec = parser::parse(Some(&crate_root().join(c.spec))).expect("spec parses");
    let opts = FrozenOptions {
        output: PathBuf::from("-"),
        test: c.test,
        update: false,
        separate_files: c.separate_files,
        module_doc: c.module_doc,
        rustfmt: false,
    };
    let modules = spec_to_rust(&opts, &spec).expect("code is generated");
    if c.separate_files {
        modules
            .into_iter()
            .map(|m| (format!("{}/{}.{RUST_EXTENSION}", c.golden, m.name), m.code))
            .collect()
    } else {
        assert_eq!(modules.len(), 1);
        modules
            .into_iter()
            .map(|m| (c.golden.to_owned(), m.code))
            .collect()
    }
}

fn first_difference(expected: &str, actual: &str) -> String {
    let mut expected_lines = expected.lines();
    let mut actual_lines = actual.lines();
    let mut n = 1;
    loop {
        match (expected_lines.next(), actual_lines.next()) {
            (Some(e), Some(a)) if e == a => n += 1,
            (e, a) => return format!("line {n}:\n  expected: {e:?}\n  actual:   {a:?}"),
        }
    }
}

fn files_under(dir: &Path, root: &Path, out: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("directory is readable") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            files_under(&path, root, out);
        } else {
            let rel = path.strip_prefix(root).expect("path is under root");
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn spec_files_under(dir: &Path, out: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("directory is readable") {
        let path = entry.expect("directory entry").path();
        let name = path.file_name().expect("file name").to_string_lossy();
        if path.is_dir() && name != "target" {
            spec_files_under(&path, out);
        } else if SPEC_FILE_NAMES.contains(&name.as_ref()) {
            let rel = path.strip_prefix(crate_root()).expect("path is under root");
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

// Checks that the code generated from every specification under test equals
// its committed golden file, so that a change of the generator shows up as a
// diff of those files. It simulates a build without rustfmt: the code is
// taken before any formatting by an external tool. The test is skipped
// outside the repository, where the test crates and their specs are missing.
#[test]
fn test_generated_code_matches_golden_files() {
    if !in_repository() {
        return;
    }
    let bless = bless();
    let mut written: BTreeMap<String, (&str, String)> = BTreeMap::new();
    let mut failures = Vec::new();

    for c in CASES {
        // generate the code of the spec with the options its crate uses
        for (golden, actual) in generate(c) {
            let path = crate_root().join(GOLDEN_DIR).join(&golden);

            // in bless mode write the golden file, unless a twin spec wrote
            // different code to it before
            if bless {
                match written.get(&golden) {
                    Some((spec, code)) if *code != actual => failures.push(format!(
                        "{golden}: {} and {spec} generate different code",
                        c.spec
                    )),
                    Some(_) => {}
                    None => {
                        fs::create_dir_all(path.parent().expect("golden parent"))
                            .expect("golden directory is created");
                        fs::write(&path, &actual).expect("golden file is written");
                        written.insert(golden, (c.spec, actual));
                    }
                }
                continue;
            }

            // compare with the golden file, ignoring CRLF line endings that a
            // Windows checkout may introduce
            match fs::read_to_string(&path) {
                Ok(expected) if expected.replace("\r\n", "\n") == actual => {}
                Ok(expected) => failures.push(format!(
                    "{golden} (from {}) differs at {}",
                    c.spec,
                    first_difference(&expected.replace("\r\n", "\n"), &actual)
                )),
                Err(e) => failures.push(format!("{golden} (from {}) can't be read: {e}", c.spec)),
            }
        }
    }

    assert!(
        failures.is_empty(),
        "generated code differs from the golden files; rerun with {BLESS_VAR}=1 \
         to update them and review the diff:\n{}",
        failures.join("\n")
    );
}

// Checks that every file in the golden directory belongs to a case, so that
// a removed or renamed case doesn't leave a stale golden file behind. It
// generates the code of every case to learn the per-module file names.
#[test]
fn test_golden_files_have_cases() {
    if !in_repository() {
        return;
    }

    // collect the golden file names the cases produce
    let mut known = BTreeSet::new();
    for c in CASES {
        if c.separate_files {
            known.extend(generate(c).into_iter().map(|(golden, _)| golden));
        } else {
            known.insert(c.golden.to_owned());
        }
    }

    // list the files under the golden directory and keep those no case names
    let golden_dir = crate_root().join(GOLDEN_DIR);
    let mut files = Vec::new();
    files_under(&golden_dir, &golden_dir, &mut files);
    let stale: Vec<String> = files.into_iter().filter(|f| !known.contains(f)).collect();

    assert!(stale.is_empty(), "golden files without a case: {stale:?}");
}

// Checks that every specification file of the test crates has a case, so
// that a new test crate doesn't go without a golden file unnoticed.
#[test]
fn test_spec_files_have_cases() {
    if !in_repository() {
        return;
    }

    // collect the spec files the cases name
    let known: BTreeSet<&str> = CASES.iter().map(|c| c.spec).collect();

    // find the spec files of the crate and its test crates
    let mut specs = Vec::new();
    spec_files_under(&crate_root().join(TEST_CRATES_DIR), &mut specs);
    specs.extend(
        SPEC_FILE_NAMES
            .iter()
            .filter(|name| crate_root().join(name).is_file())
            .map(|name| name.to_string()),
    );

    // keep those no case names
    let missing: Vec<String> = specs
        .into_iter()
        .filter(|s| !known.contains(s.as_str()))
        .collect();

    assert!(missing.is_empty(), "spec files without a case: {missing:?}");
}
