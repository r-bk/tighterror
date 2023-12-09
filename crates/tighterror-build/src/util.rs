use crate::{
    errors::{TebError, FAILED_TO_OPEN_SPEC_FILE},
    spec::Spec,
};
use std::{collections::HashSet, fs::File, path::PathBuf};

pub fn open_spec_file(path: &PathBuf) -> Result<File, TebError> {
    match File::options().read(true).open(path) {
        Ok(f) => Ok(f),
        Err(e) => {
            log::error!("Failed to open the spec file {:?}: {e}", path);
            FAILED_TO_OPEN_SPEC_FILE.into()
        }
    }
}

pub fn get_non_unique_error_names(spec: &Spec) -> Vec<String> {
    let mut ans = HashSet::new();
    let mut hs = HashSet::new();

    for c in &spec.categories {
        for e in &c.errors {
            let name = e.name.to_lowercase();
            if hs.contains(&name) {
                ans.insert(e.name.clone());
            } else {
                hs.insert(name);
            }
        }
    }

    Vec::from_iter(ans)
}
