use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn download_file(url: String, folder: PathBuf, file_name: &str) -> Result<()> {
    let body = ureq::get(&url).call()?;
    let mut buffer = Vec::new();
    let _ = body.into_reader().read_to_end(&mut buffer)?;
    let mut file = File::create(folder.join(file_name))
        .context("Failed to create destination file for writing")?;
    let _ = file.write(&buffer).context("Failed to write file")?;
    return Ok(());
}

fn main() -> Result<()> {
    let target = env::var("TARGET").unwrap();

    let version = "1.0.6";

    let source_file = if target.contains("apple") && target.contains("aarch64") {
        let base_file = "libcatboostmodel.dylib".to_string();
        base_file
    } else if target.contains("linux") {
        let base_file = "libcatboostmodel.so".to_string();
        base_file
    } else {
        return Err(anyhow!("Unknown target triple").context(target));
    };

    let dest_file = format!("{}.1", source_file);

    let download_url = format!(
        "https://github.com/catboost/catboost/releases/download/v{}/{}",
        version, source_file
    );

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").context("Couldn't get OUT_DIR from env variable")?);

    let build_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .context("Couldn't get CARGO_MANIFEST_DIR from env variable")?,
    );

    let out_model_interface_dir = out_dir.join("catboost/libs/model_interface");

    create_dir_all(&out_model_interface_dir)
        .context("Couldn't create model interface directory")?;

    download_file(
        download_url,
        Path::new(&out_model_interface_dir).to_path_buf(),
        &dest_file,
    )?;

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

    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("catboost/libs/model_interface").display()
    );

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
    println!("cargo:rustc-link-lib=dylib=catboostmodel");

    return Ok(());
}
