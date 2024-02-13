extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cb_model_interface_root = Path::new("../../libs/model_interface/")
        .canonicalize()
        .unwrap();

    #[allow(unused_mut)]
    let mut build_native_args = vec![
        "--targets",
        "catboostmodel_static",
        "--build-root-dir",
        out_dir.to_str().unwrap(),
        "--build-type=Release",
    ];

    #[cfg(feature = "gpu")]
    build_native_args.push("--have-cuda");

    let build_cmd_status = Command::new("../../../build/build_native.py")
        .args(&build_native_args)
        .status()
        .unwrap_or_else(|e| {
            panic!("Failed to run build_native.py : {}", e);
        });

    if !build_cmd_status.success() {
        panic!("Building with build_native.py failed");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", cb_model_interface_root.display()))
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings.");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings.");

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("catboost/libs/model_interface/static").display()
    );
    
    println!("cargo:rustc-link-lib=static=catboostmodel_static");
    println!("cargo:rustc-link-lib=static=catboostmodel_static.global");
}
