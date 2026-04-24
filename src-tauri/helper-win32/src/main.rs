use std::{io::Write, process::ExitCode};

mod embedded;
mod firewall;
mod formatter;
mod my_exit_code;
mod network;

use my_exit_code::CustomExitCode;

fn main() -> ExitCode {
    let os_args = formatter::get_formatted_args();
    let command = formatter::get_command_at_index(0, &os_args);

    let mut writer = formatter::create_writer()
        .inspect_err(|err| eprintln!("Failed to create writer: {}", err))
        .ok();

    formatter::write_arguments(os_args, &mut writer);

    let divider = "-".repeat(40);
    formatter::writeln(&mut writer, &divider);
    let exit_code = execute(&command, &mut writer).to_exit_code();
    formatter::writeln(&mut writer, &divider);

    exit_code
}

fn execute(command: &str, writer: &mut Option<impl Write>) -> CustomExitCode {
    match command {
        "--add" => {
            if !network::is_private_network() {
                return CustomExitCode::SUCCESS;
            }
            if firewall::firewall_rule_exists(writer).is_err() {
                return CustomExitCode::FAILED_TO_CHECK_FIREWALL_RULE;
            }
            // Guard rest of the operations. They need admin elevation.
            if !ensure_elevation(writer) {
                return CustomExitCode::USER_DENIED_TO_ELEVATE;
            }
            firewall::firewall_rule_add_or_update(writer)
        }
        "--remove" => {
            if !ensure_elevation(writer) {
                return CustomExitCode::USER_DENIED_TO_ELEVATE;
            }
            firewall::firewall_rule_remove(writer)
        }
        _ => CustomExitCode::SUCCESS,
    }
}

fn ensure_elevation(writer: &mut Option<impl Write>) -> bool {
    if embedded::is_elevated() {
        formatter::writeln(writer, "Running as elevated");
        return true;
    }

    formatter::writeln(writer, "I need admin privilage");
    match embedded::elevate_current_exe() {
        Ok(true) => true,
        Ok(false) => {
            formatter::writeln(writer, "User denied to elevate");
            false
        }
        Err(err) => {
            formatter::writeln(writer, &err);
            return false;
        }
    }
}
