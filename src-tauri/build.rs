use std::path::PathBuf;
use std::process::Command;

mod build_helper;

fn main() {
    #[cfg(windows)]
    {
        use dotenv;
        match dotenv::dotenv() {
            Ok(_) => {}
            Err(e) => assert!(false, "[.env] - {}", e),
        }

        let helper_folder = "./helper-win32";
        let helper_file = format!("{}/target/release/helper.exe", helper_folder);
        let helper_path = PathBuf::from(&helper_file);

        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                &format!("{}/Cargo.toml", helper_folder),
            ])
            // .env("EMBEDDED_WIN32_SHA256", "<Just setting here>")
            .status()
            .expect("Failed to build helper-win32");

        assert!(status.success(), "helper-win32 build failed");

        let result = build_helper::sign::sign_binary();
        for error in result.errors {
            println!("cargo:warning=[sign binary]: {}", error);
        }
        assert!(!result.is_fatal, "helper-win32 signing failed");

        let hash = build_helper::sha256_file(&helper_path).expect("Expected file to not fail");
        println!("cargo:rustc-env=EMBEDDED_WIN32_SHA256={}", hash);

        println!("cargo:rerun-if-changed={}/Cargo.toml", helper_folder);
        println!("cargo:rerun-if-changed={}/src/main.rs", helper_folder);
    }

    tauri_build::build()
}
