use std::{env, fs::File, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let const_path = Path::new(&out_dir).join("const.rs");
    let mut const_file = File::create(&const_path).expect("Create const.rs file failed");
}
