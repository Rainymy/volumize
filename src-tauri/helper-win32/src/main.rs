mod formatter;
mod my_exit_code;
mod win32;

use my_exit_code::CustomExitCode;

pub const APPLICATION_NAME: &str = env!("APPLICATION_NAME");
pub const APPLICATION_EXE: &str = env!("APPLICATION_EXE");

#[cfg(unix)]
fn main() -> std::process::ExitCode {
    return CustomExitCode::SUCCESS.to_exit_code();
}

#[cfg(windows)]
fn main() -> std::process::ExitCode {
    let os_args = formatter::get_formatted_args();
    let command = formatter::get_command_at_index(0, &os_args);

    let mut writer = formatter::create_writer().ok();

    // formatter::writeln(&mut writer, &format!("NAME={}", APPLICATION_NAME));

    let divider = "-".repeat(40);
    formatter::writeln(&mut writer, &divider);
    let exit_code = execute(&command, &mut writer);
    formatter::writeln(&mut writer, &divider);

    let _ = std::fs::write("./exit-code.txt", exit_code.as_u8().to_string());

    exit_code.to_exit_code()
}

// #[cfg(windows)]
fn execute(command: &str, writer: &mut Option<impl std::io::Write>) -> CustomExitCode {
    match command {
        "--add" => win32::firewall_rule_add_or_update(writer),
        "--remove" => win32::firewall_rule_remove(writer),
        option => {
            formatter::writeln(writer, &format!("Unknown option: {}", option));
            CustomExitCode::SUCCESS
        }
    }
}
