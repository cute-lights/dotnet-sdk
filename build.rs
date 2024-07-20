use std::path::{Path, PathBuf};

const VERSION: &str = "v0.2.3";

fn main() {
    println!("cargo:rerun-if-changed=NULL");
    println!("cargo:rerun-if-changed=build.rs");
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let obj_dir = Path::new(&project_dir).join("obj");
    let natives_dir = obj_dir.join("natives");

    let header_path =
        download_gh_asset("cute-lights", "c-sdk", "cutelight.h", VERSION, &natives_dir);

    let _lib_path = download_gh_asset(
        "cute-lights",
        "c-sdk",
        "libcutelight.so",
        VERSION,
        &natives_dir,
    );

    println!("cargo:rerun-if-changed={}", header_path.display());
    let rs_intermediary_path = obj_dir.join("intermediary.rs");

    // Remove the unended header includes
    let header_contents = std::fs::read_to_string(header_path)
        .unwrap()
        .replace("#include <stdarg.h>", "")
        .replace("#include <stdlib.h>", "");
    bindgen::Builder::default()
        .header_contents("cute_lights.h", &header_contents)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&rs_intermediary_path)
        .expect("Couldn't write bindings!");

    csbindgen::Builder::default()
        .input_bindgen_file(&rs_intermediary_path) // read from bindgen generated code
        .csharp_dll_name("libcutelight.so")
        .csharp_namespace("CuteLight.Sdk")
        .generate_to_file(
            rs_intermediary_path,
            Path::new(&project_dir)
                .join("src")
                .join("NativeMethods.g.cs"),
        )
        .unwrap();
}

fn download_gh_asset(
    owner: &str,
    repo: &str,
    asset_name: &str,
    version: &str,
    target_dir: &Path,
) -> PathBuf {
    let out_file = target_dir.join(asset_name);

    if out_file.exists() {
        return out_file;
    }

    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir).unwrap();
    }

    let dl_url = format!(
        "https://github.com/{}/{}/releases/download/{}/{}",
        owner, repo, version, asset_name
    );

    let command = format!("curl -L {} -o {}", dl_url, out_file.to_str().unwrap());

    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to download asset");

    return out_file;
}
