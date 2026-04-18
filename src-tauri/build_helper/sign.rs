use std::{path::PathBuf, process::Command};

#[allow(dead_code)]
pub struct SignBinaryResult {
    pub is_fatal: bool,
    pub errors: Vec<String>,
}

pub fn sign_binary(path: &PathBuf) -> SignBinaryResult {
    use std::env::VarError;

    let should_sign = match std::env::var("SIGN_BINARIES") {
        Ok(value) => {
            let value = value.to_lowercase();
            value == "1" || value == "true"
        }
        Err(VarError::NotPresent) => {
            return SignBinaryResult {
                is_fatal: true,
                errors: vec!["[ENV var]: SIGN_BINARIES not set".into()],
            };
        }
        Err(VarError::NotUnicode(_)) => {
            return SignBinaryResult {
                is_fatal: true,
                errors: vec!["[ENV var]: Invalid value for SIGN_BINARIES".into()],
            };
        }
    };

    if !should_sign {
        return SignBinaryResult {
            is_fatal: false,
            errors: vec!["Skipping signing (SIGN_BINARIES is set to false)".into()],
        };
    }

    sign_with_signtool(path)
}

pub fn sign_with_signtool(path: &PathBuf) -> SignBinaryResult {
    let pfx_path = std::env::var("SIGN_CERT_PATH").expect("[ENV var]: SIGN_CERT_PATH not set");
    let pfx_pass =
        std::env::var("SIGN_CERT_PASSWORD").expect("[ENV var]: SIGN_CERT_PASSWORD not set");

    let status = Command::new("signtool")
        .status()
        .expect("signtool failed to run");

    assert!(status.success(), "signtool signing failed for {:?}", path);
    println!("cargo:warning=Signed {:?}", path);

    let output_info = status
        .success()
        .then(|| format!("Signed {:?}", path))
        .unwrap_or_else(|| format!("signtool signing failed for {:?}", path));

    return SignBinaryResult {
        is_fatal: !status.success(),
        errors: vec![output_info],
    };
}
