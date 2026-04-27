use std::path::PathBuf;
use std::process::Command;

const HELPER_FOLDER: &str = "helper-win32";

fn main() {
    #[cfg(windows)]
    {
        match dotenv::dotenv() {
            Ok(_) => {}
            Err(e) => assert!(false, "[.env] - {}", e),
        }

        let helper_folder = HELPER_FOLDER;
        let helper_file = format!("./{}/target/release/helper.exe", helper_folder);
        let helper_path = PathBuf::from(&helper_file);

        let product_name = build_tauri_config()
            .product_name
            .unwrap_or(env!("CARGO_PKG_NAME").to_string());

        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                &format!("./{}/Cargo.toml", helper_folder),
            ])
            .env("APPLICATION_NAME", &product_name)
            .output()
            .expect(&format!("Failed to build {}", helper_folder));

        // Some reason clippy only outputs to stderr.
        for line in String::from_utf8_lossy(&status.stderr).lines() {
            println!("cargo:warning=[helper] {}", format_child_output(line));
        }

        assert!(status.status.success(), "helper-win32 build failed");

        std::fs::copy(
            &helper_path,
            &get_target_dir().join(helper_path.file_name().unwrap()),
        )
        .expect("Failed to copy file");

        println!("cargo:rerun-if-changed=./{}/Cargo.toml", helper_folder);
        println!("cargo:rerun-if-changed=./{}/build.rs", helper_folder);
        println!("cargo:rerun-if-changed=./{}/src/main.rs", helper_folder);
    }

    tauri_build::build()
}

/// Building the Tauri config myself.
///
/// Because Tauri doesn't expose the config parsing to build scripts.
fn build_tauri_config() -> tauri::utils::config::Config {
    use tauri::utils;

    let target_triple = std::env::var("TARGET").expect("TARTGET TO EXIST");
    let target = utils::platform::Target::from_triple(&target_triple);
    let mut current_dir = std::env::current_dir().expect("NO CURRENT DIR");
    current_dir.push("tauri.conf.json");

    let (config, _path) = utils::config::parse(target, current_dir).unwrap();
    config
}

fn get_target_dir() -> std::path::PathBuf {
    // Copy how Tauri does it. To get the target directory.
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    target_dir.to_path_buf()
}

fn format_child_output(input: &str) -> String {
    fn get_version(input: &str) -> Option<String> {
        let first_part = input.split_once("@")?.1;
        let version = first_part.split_once(":")?.0;
        Some(version.to_string())
    }

    let version = match get_version(input) {
        Some(version) => version,
        // Can't find a version, return the input as is
        None => return input.to_string(),
    };

    let format_string = format!("warning: {}@{}: ", HELPER_FOLDER, version);
    input.replace(&format_string, "")
}
