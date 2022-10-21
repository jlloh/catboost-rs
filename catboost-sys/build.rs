use anyhow::{Context, Result};
use std::env;
use std::fs::create_dir_all;

use std::path::PathBuf;

fn main() -> Result<()> {
    let target = env::var("TARGET").unwrap();

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").context("Couldn't get OUT_DIR from env variable")?);

    let build_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .context("Couldn't get CARGO_MANIFEST_DIR from env variable")?,
    );

    let out_model_interface_dir = out_dir.join("catboost/libs/model_interface");

    create_dir_all(&out_model_interface_dir)
        .context("Couldn't create model interface directory")?;

    let c_bindings_model_interface_path = build_dir.join("c_bindings/model_interface");

    let c_bindings_model_interface =
        c_bindings_model_interface_path
            .canonicalize()
            .context(format!(
                "Failed to find model_interface directory with c bindings: {:?}",
                c_bindings_model_interface_path
            ))?;

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", c_bindings_model_interface.display()))
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings.");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .context("Couldn't write bindings.")?;

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
    println!("cargo:rustc-link-lib=dylib=catboostmodel");

    return Ok(());
}
