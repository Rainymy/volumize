use std::{io::Write, path::Path};

/// Returns the formatted arguments passed to the program.
/// - Skips the first argument (the program name).
pub fn get_formatted_args() -> Vec<String> {
    std::env::args_os()
        .skip(1)
        .map(|v| v.to_string_lossy().trim().to_string())
        .collect::<Vec<String>>()
}

pub fn get_command_at_index(index: usize, os_args: &Vec<String>) -> String {
    os_args.get(index).cloned().unwrap_or_default()
}

fn format_current_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();

    #[cfg(target_os = "windows")]
    let offset = super::win32::local_utc_offset_secs();
    #[cfg(not(target_os = "windows"))]
    let offset = 0i64;
    let t_secs = since_epoch.as_secs() as i64 + offset;

    let ms = since_epoch.subsec_millis() % 1000;
    let secs = t_secs % 60;
    let mins = (t_secs / 60) % 60;
    let hours = (t_secs / 3600) % 24;

    format!("{:02}:{:02}:{:02}:{:03}", hours, mins, secs, ms)
}

pub fn writeln(w: &mut Option<impl Write>, msg: &str) {
    let curent_time = format_current_time();

    let _ = match w {
        Some(w) => {
            let formatted = format!("[{curent_time}]: {}\n", msg);
            #[cfg(debug_assertions)]
            print!("{}", formatted);
            let _ = write!(w, "{}", formatted);
        }
        None => (),
    };
}

fn get_log_path() -> std::path::PathBuf {
    #[cfg(debug_assertions)]
    let mut folder = {
        use std::env::current_dir;
        current_dir().unwrap_or_default()
    };
    #[cfg(not(debug_assertions))]
    let mut folder = {
        use std::env::home_dir;
        let mut root = home_dir().unwrap_or_default();
        root.push(".volumize");
        root
    };
    folder.push("log.txt");
    folder
}

pub fn create_writer() -> Result<impl Write, std::io::Error> {
    let folder = get_log_path();

    const FILE_SIZE: u64 = 4 * 1024; // 256KB
    match remove_file_over(&folder, FILE_SIZE) {
        Ok(_) => (),
        Err(err) => eprintln!("Remove file error: {}", err),
    };

    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(folder)
}

fn remove_file_over(path: &Path, size: u64) -> Result<(), std::io::Error> {
    let file_size = std::fs::metadata(path)
        .and_then(|op| Ok(op.len()))
        .unwrap_or(0);

    if file_size > size {
        use std::fs::File;
        File::create(path).map(|_| ())
    } else {
        Ok(())
    }
}

pub fn write_arguments(writer: &mut Option<impl Write>, args: Vec<String>) {
    writeln(writer, "");
    writeln(writer, "Arguments:");
    for (index, arg) in args.iter().enumerate() {
        writeln(writer, &format!("   {}: {}", index + 1, arg));
    }
    writeln(writer, "");
}
