use crate::errors::{
    kind::coder::{FAILED_TO_PARSE_TOKENS, RUSTFMT_FAILED, RUSTFMT_NOT_FOUND},
    TbError,
};
use log::{error, info, warn};
use proc_macro2::TokenStream;
use regex::RegexSet;
use std::{ffi::OsStr, io::ErrorKind, process::Command};

pub fn pretty(tokens: TokenStream) -> Result<String, TbError> {
    let tokens_str = tokens.to_string();
    let file = match syn::parse2(tokens) {
        Ok(f) => f,
        Err(e) => {
            error!("syn failed to parse generated code: {e}");
            error!("bad tokens: {tokens_str}");
            return FAILED_TO_PARSE_TOKENS.into();
        }
    };
    Ok(add_newlines(prettyplease::unparse(&file)))
}

fn add_newlines(file: String) -> String {
    let rg_pre = RegexSet::new([
        r"^impl",
        r"^[[:space:]]*fn",
        r"^[[:space:]]*const fn",
        r"^[[:space:]]*pub fn",
        r"^[[:space:]]*#\[.*\]$",
    ])
    .unwrap();
    let rg_post = RegexSet::new([
        r"^[[:space:]]*}$",
        r"^[[:space:]]*];$",
        r"^[[:space:]]*impl.*\{\}$",
    ])
    .unwrap();
    let rg_comment = RegexSet::new([r"^[[:space:]]*///", r"^[[:space:]]*/\*\*"]).unwrap();
    let rg_inner_comment = RegexSet::new([r"^[[:space:]]*//!"]).unwrap();
    let mut ans = String::with_capacity(file.capacity());
    let mut last_line_is_comment = false;
    let mut last_line_is_inner_comment = false;
    let mut last_line_prefix = false;
    for line in file.lines() {
        let is_comment = rg_comment.is_match(line);
        let is_inner_comment = rg_inner_comment.is_match(line);
        let prefix = rg_pre.is_match(line)
            || (is_comment && !last_line_is_comment)
            || (!is_inner_comment && last_line_is_inner_comment);
        let postfix = rg_post.is_match(line);

        if prefix && !last_line_prefix {
            ans.push('\n');
        }
        ans.push_str(line);
        ans.push('\n');
        if postfix {
            ans.push('\n');
        }

        last_line_is_comment = is_comment;
        last_line_is_inner_comment = is_inner_comment;
        last_line_prefix = prefix;
    }
    ans
}

pub fn rustfmt(path: impl AsRef<OsStr>) -> Result<(), TbError> {
    let result = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .arg(path)
        .status();
    match result {
        Ok(exit_status) => {
            if !exit_status.success() {
                warn!("rustfmt failed");
                RUSTFMT_FAILED.into()
            } else {
                Ok(())
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            info!("rustfmt not found, skipping");
            RUSTFMT_NOT_FOUND.into()
        }
        Err(e) => {
            warn!("failed to spawn rustfmt: {e}");
            RUSTFMT_FAILED.into()
        }
    }
}
