use std::path::Path;

const C_SDK_OWNER: &str = "cute-lights";
const C_SDK_REPO: &str = "c-sdk";
const C_SDK_VERSION: Option<&str> = None; // None for latest

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=NULL");
    println!("cargo:rerun-if-changed=build.rs");
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_file = Path::new(&project_dir).join("CuteLight.Sdk.csproj");

    if std::env::var("BUMP_VERSION").unwrap_or("0".to_string()) == "1" {
        let tag = std::env::var("CARGO_PKG_VERSION").unwrap();
        let content = std::fs::read_to_string(&project_file).unwrap();
        let re = regex::Regex::new(r#"<Version>(.*)<\/Version> <!-- ci-tag-anchor -->"#).unwrap();

        // get the first group
        let new_content = re.replace(
            &content,
            format!("<Version>{}</Version> <!-- ci-tag-anchor -->", tag).as_str(),
        );

        std::fs::write(&project_file, new_content.as_bytes()).unwrap();
    }

    let obj_dir = Path::new(&project_dir).join("obj");
    let natives_dir = obj_dir.join("natives");

    let header_path = natives_dir.join("dist/include").join("cutelight.h");

    let lib_path = natives_dir.join("dist/lib64").join("libcutelight.so");

    if !header_path.exists() || !lib_path.exists() {
        if natives_dir.exists() {
            std::fs::remove_dir_all(&natives_dir).unwrap();
        }
        std::fs::create_dir_all(&natives_dir).unwrap();
        let arc_path = natives_dir.join("cutelight.tar.gz");

        let releases = octocrab::instance()
            .repos(C_SDK_OWNER, C_SDK_REPO)
            .releases()
            .list()
            .send()
            .await
            .unwrap();

        let release = releases
            .items
            .iter()
            .find(|r| {
                if let Some(version) = C_SDK_VERSION {
                    r.tag_name == version
                } else {
                    true
                }
            })
            .expect("No release found")
            .assets
            .iter()
            .find(|a| a.name == "cutelight.tar.gz")
            .expect("No asset found");

        let data = reqwest::get(&release.browser_download_url.to_string())
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        std::fs::write(&arc_path, data).unwrap();

        let tar = flate2::read::GzDecoder::new(std::fs::File::open(&arc_path).unwrap());
        let mut archive = tar::Archive::new(tar);
        archive.unpack(&natives_dir).unwrap();
    }
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
        .csharp_namespace("CuteLights.Sdk")
        .generate_to_file(
            rs_intermediary_path,
            Path::new(&project_dir)
                .join("src")
                .join("NativeMethods.g.cs"),
        )
        .unwrap();
}
