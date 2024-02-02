#![deny(warnings)]
#![forbid(unsafe_code)]

use log::error;
use std::process::exit;
use tighterror_build::CodegenOptions;

mod args;
use args::*;

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
    codegen_main(Args::parse_args())
}
