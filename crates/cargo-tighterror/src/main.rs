#![deny(warnings)]
#![forbid(unsafe_code)]

use log::error;
use std::process::exit;
use tighterror_build::CodegenOptions;

mod args;
use args::*;

fn codegen_main(args: Args) {
    let test = args.test();
    let update = args.update();
    let separate_files = args.separate_files();
    if let Err(e) = CodegenOptions::new()
        .spec_option(args.spec)
        .output_option(args.output)
        .test(test)
        .update(update)
        .separate_files(separate_files)
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
