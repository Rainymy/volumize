const HELPER_FOLDER: &str = "helper-win32";

fn main() {
    let target_os = std::env::var_os("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        let tauri_config = build_tauri_config();
        let current_dir = std::env::current_dir().unwrap();

        let cargo_pkg_name = env!("CARGO_PKG_NAME");
        let product_name = tauri_config.product_name.unwrap_or(cargo_pkg_name.into());
        let installer_icon = current_dir.join(
            tauri_config
                .bundle
                .windows
                .nsis
                .unwrap_or_default()
                .installer_icon
                .unwrap_or_default(),
        );

        use std::process::Command;
        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                &format!("./{}/Cargo.toml", HELPER_FOLDER),
            ])
            .env("APPLICATION_NAME", product_name)
            .env("APPLICATION_ICON", installer_icon.display().to_string())
            .output()
            .expect(&format!("Failed to build {}", HELPER_FOLDER));

        // Some reason clippy only outputs to stderr.
        for line in String::from_utf8_lossy(&status.stderr).lines() {
            println!("cargo:warning=[helper] {}", format_child_output(line));
        }
        assert!(status.status.success(), "helper-win32 build failed");

        println!("cargo:rerun-if-changed=./{}/Cargo.toml", HELPER_FOLDER);
        println!("cargo:rerun-if-changed=./{}/build.rs", HELPER_FOLDER);
        println!("cargo:rerun-if-changed=./{}/src/main.rs", HELPER_FOLDER);
        println!("cargo:rerun-if-changed=./{}/manifest.xml", HELPER_FOLDER);
    }

    tauri_build::build()
}

/// Building the Tauri config myself.
///
/// Because Tauri doesn't expose the config parsing to build scripts.
fn build_tauri_config() -> tauri::utils::config::Config {
    use std::env::{current_dir, var};
    use tauri::utils::{
        config::{parse, Config},
        platform,
    };

    let target_triple = var("TARGET").expect("TARTGET TO EXIST");
    let target = platform::Target::from_triple(&target_triple);
    let current_dir = current_dir().expect("Failed to get current_dir");

    let (mut read_value, _path) = parse::read_from(target, &current_dir).unwrap();
    if let Ok(env) = var("TAURI_CONFIG") {
        let merge_config: serde_json::Value = serde_json::from_str(&env).unwrap();
        json_patch::merge(&mut read_value, &merge_config);
    }

    let config: Config = serde_json::from_value(read_value).unwrap();
    config
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
