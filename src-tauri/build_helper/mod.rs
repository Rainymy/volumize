use std::path::PathBuf;

#[allow(dead_code)]
pub fn sha256_file(path: &PathBuf) -> Result<String, std::io::Error> {
    use sha2::{Digest, Sha256};

    Ok(Sha256::digest(std::fs::read(path)?)
        .into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(""))
}

#[allow(dead_code)]
pub fn sha256_file_stream(path: &std::path::Path) -> Result<String, std::io::Error> {
    use sha2::{Digest, Sha256};
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher
        .finalize()
        .iter()
        .map(|v| format!("{:x}", v))
        .collect::<String>())
}
