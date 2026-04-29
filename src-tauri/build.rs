const HELPER_FOLDER: &str = "helper-win32";

fn main() {
    #[cfg(windows)]
    {
        dotenv::dotenv().expect("[env.] dotenv to not fail");

        let helper_folder = HELPER_FOLDER;
        let cargo_pkg_name = env!("CARGO_PKG_NAME");
        let product_name = build_tauri_config()
            .product_name
            .unwrap_or(cargo_pkg_name.into());

        use std::process::Command;
        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                &format!("./{}/Cargo.toml", helper_folder),
            ])
            .env("APPLICATION_NAME", &product_name)
            .env("APPLICATION_EXE", &format!("{}.exe", cargo_pkg_name))
            .output()
            .expect(&format!("Failed to build {}", helper_folder));

        // Some reason clippy only outputs to stderr.
        for line in String::from_utf8_lossy(&status.stderr).lines() {
            println!("cargo:warning=[helper] {}", format_child_output(line));
        }
        assert!(status.status.success(), "helper-win32 build failed");

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
    use std::env::{current_dir, var};
    use tauri::utils::{
        config::{parse, Config},
        platform,
    };

    let target_triple = var("TARGET").expect("TARTGET TO EXIST");
    let target = platform::Target::from_triple(&target_triple);
    let current_dir = current_dir().unwrap();

    let (read_value, _path) = parse::read_from(target, &current_dir).unwrap();
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
