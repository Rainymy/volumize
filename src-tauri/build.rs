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
        let helper_file = format!("{}/target/release/helper-win32.exe", helper_folder);
        let helper_path = PathBuf::from(&helper_file);

        let status = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                &format!("{}/Cargo.toml", helper_folder),
            ])
            .status()
            .expect("Failed to build helper-win32");

        assert!(status.success(), "helper-win32 build failed");

        let result = build_helper::sign::sign_binary(&helper_path);
        for error in result.errors {
            println!("cargo:warning=[sign binary]: {}", error);
        }
        if result.is_fatal {
            assert!(result.is_fatal, "helper-win32 signing failed");
        }

        println!("cargo:rerun-if-changed={}/Cargo.toml", helper_folder);
        println!("cargo:rerun-if-changed={}/src/main.rs", helper_folder);
    }

    tauri_build::build()
}
