#![forbid(unsafe_code)]

use log::error;
use std::{path::PathBuf, process::exit};
use tighterror_build::{lint, CodegenOptions, LintLevel};

mod args;
use args::*;

fn lint_level_to_log_level(level: LintLevel) -> log::Level {
    match level {
        LintLevel::Error => log::Level::Error,
        LintLevel::Warning => log::Level::Warn,
        LintLevel::Notice => log::Level::Info,
    }
}

fn lint_main(args: Args) {
    let report = match lint(args.spec.map(PathBuf::from)) {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            exit(1);
        }
    };

    for lint in report.messages() {
        let log_level = lint_level_to_log_level(lint.level);
        log::log!(log_level, "{}", lint.msg);
    }
}

fn codegen_main(args: Args) {
    let test = args.test();
    if let Err(e) = CodegenOptions::new()
        .spec(args.spec)
        .dst(args.dst)
        .test(test)
        .codegen()
    {
        error!("{e}");
        exit(1);
    }
}

fn main() {
    pretty_env_logger::init();

    let args = Args::parse_args();
    if args.lint {
        lint_main(args)
    } else {
        codegen_main(args)
    }
}
