use std::path::PathBuf;
use std::process::Command;

mod build_helper;

const HELPER_FOLDER: &str = "helper-win32";

fn main() {
    #[cfg(windows)]
    {
        use dotenv;
        match dotenv::dotenv() {
            Ok(_) => {}
            Err(e) => assert!(false, "[.env] - {}", e),
        }

        let target_dir = get_target_dir();

        let helper_folder = HELPER_FOLDER;
        let helper_file = format!("./{}/target/release/helper.exe", helper_folder);
        let helper_path = PathBuf::from(&helper_file);

        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                &format!("./{}/Cargo.toml", helper_folder),
            ])
            .env("APPLICATION_FULL_PATH", &target_dir)
            .env("APPLICATION_NAME", &config.product_name.unwrap())
            .output()
            .expect("Failed to build helper-win32");

        // Some reason clippy only outputs to stderr.
        for line in String::from_utf8_lossy(&status.stderr).lines() {
            if line.starts_with("warning") {
                println!("cargo:warning=[helper] {}", format_child_output(line));
            }
        }

        assert!(status.status.success(), "helper-win32 build failed");


        std::fs::copy(
            &helper_path,
            &target_dir.join(helper_path.file_name().unwrap()),
        )
        .expect("Failed to copy file");

        println!("cargo:rerun-if-changed=./{}/Cargo.toml", helper_folder);
        println!("cargo:rerun-if-changed=./{}/build.rs", helper_folder);
        println!("cargo:rerun-if-changed=./{}/src/main.rs", helper_folder);
    }

    tauri_build::build()
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
