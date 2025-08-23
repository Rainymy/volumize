use std::{fs::OpenOptions, io::Write};

struct ThreadLogger {
    file: std::fs::File,
}

impl ThreadLogger {
    fn new(filename: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;

        Ok(ThreadLogger { file })
    }

    fn log(&mut self, message: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let log_entry = format!(
            "[{}] {}: {}\n",
            timestamp,
            std::thread::current().name().unwrap_or("unnamed"),
            message
        );

        let _ = self.file.write_all(log_entry.as_bytes());
        let _ = self.file.flush();

        // Also print to console
        print!("{}", log_entry);
        let _ = std::io::stdout().flush();
    }
}
