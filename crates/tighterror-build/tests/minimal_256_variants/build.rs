use tighterror_build::CodegenOptions;

fn main() {
    println!("cargo:rerun-if-changed=tighterror.yaml");
    env_logger::builder().init();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dst_path = format!("{out_dir}/errors.rs");
    if let Err(e) = CodegenOptions::new().dst(dst_path).test(true).codegen() {
        panic!("codegen failed: out_dir: {out_dir}; {e}");
    }
}
