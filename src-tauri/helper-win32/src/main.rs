use std::env::args_os;
use std::io;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let os_args = args_os()
        .skip(1)
        .map(|v| v.to_string_lossy().trim().to_string())
        .collect::<Vec<String>>();

    let exit_code = match os_args[0].clone().as_str() {
        "--add" => add_firewall_rule(),
        "--remove" => remove_firewall_rule(),
        _ => ExitCode::FAILURE,
    };

    let mut index = 1;
    for arg in os_args.clone() {
        println!("{index}: {}", arg);
        index += 1;
    }
    let _ = Command::new("echo").args(args_os()).output();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input == "1" {
        return ExitCode::from(3);
    }
    exit_code
}

fn add_firewall_rule() -> ExitCode {
    println!("Adding Firewall");
    ExitCode::SUCCESS
}
fn remove_firewall_rule() -> ExitCode {
    println!("Removing Firewall");
    ExitCode::SUCCESS
}
