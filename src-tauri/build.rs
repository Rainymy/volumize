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
            .status()
            .expect("Failed to build helper-win32");

        assert!(status.success(), "helper-win32 build failed");

        // Copy how Tauri does it. To get the target directory.
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let target_dir = out_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        println!(
            "cargo:warning=Target directory: {}",
            target_dir.to_string_lossy()
        );

        fn copy_file(src: &PathBuf, dest: &PathBuf) {
            std::fs::copy(src, dest).expect("Failed to copy file");
        }

        copy_file(
            &helper_path,
            &target_dir.join(helper_path.file_name().unwrap()),
        );

        println!("cargo:rerun-if-changed={}/Cargo.toml", helper_folder);
        println!("cargo:rerun-if-changed={}/src/main.rs", helper_folder);
    }

    tauri_build::build()
}
