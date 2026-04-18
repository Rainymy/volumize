use std::path::PathBuf;

pub mod sign;

pub fn sha256_file(path: &PathBuf) -> Result<String, std::io::Error> {
    use sha2::{Digest, Sha256};

    Ok(Sha256::digest(std::fs::read(path)?)
        .into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(""))
}
