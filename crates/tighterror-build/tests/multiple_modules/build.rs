use tighterror_build::CodegenOptions;

fn main() {
    println!("cargo:rerun-if-changed=tighterror.yaml");
    env_logger::builder().init();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    if let Err(e) = CodegenOptions::new()
        .output(out_dir.clone())
        .test(true)
        .separate_files(true)
        .codegen()
    {
        panic!("codegen failed: out_dir: {out_dir}; {e}");
    }
}
