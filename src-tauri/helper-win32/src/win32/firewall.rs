#[cfg(target_family = "windows")]
use super::CustomExitCode;
// #[allow(unused_imports)]
use super::writeln;
// #[allow(unused_imports)]
use std::io::Write;

#[cfg(target_family = "windows")]
use windows_firewall::{Action, Direction, FirewallRule, Profile, Protocol};

/// Only reason for returning `Option` is to handle `std::env::current_exe` error.
#[cfg(target_family = "windows")]
fn firewall_rule() -> FirewallRule {
    FirewallRule::builder()
        .description("Volumize firewall rule")
        .enabled(true)
        .action(Action::Allow)
        .profiles(Profile::Private)
        .protocol(Protocol::Tcp)
        .direction(Direction::In)
        .local_ports([9002])
        .build()
            .name(super::super::APPLICATION_NAME)
            .application_name(application_path)
}

#[cfg(target_family = "windows")]
pub fn firewall_rule_add_or_update(writer: &mut Option<impl Write>) -> CustomExitCode {
    let rule = match firewall_rule() {
        Some(rule) => rule,
        None => return CustomExitCode::FAILED_TO_FIND_EXECUTABLE,
    };

    match rule.add_or_update() {
        Ok(true) => writeln(writer, "Firewall rule added successfully!"),
        Ok(false) => writeln(writer, "Firewall rule updated successfully!"),
        Err(e) => {
            writeln(
                writer,
                &format!("Failed to add/update firewall rule: {}", e),
            );
            return CustomExitCode::FAILED_TO_ADD_FIREWALL_RULE;
        }
    }
    CustomExitCode::SUCCESS
}

#[cfg(target_family = "windows")]
pub fn firewall_rule_remove(writer: &mut Option<impl Write>) -> CustomExitCode {
    let rule = match firewall_rule() {
        Some(rule) => rule,
        None => return CustomExitCode::FAILED_TO_FIND_EXECUTABLE,
    };
    writeln(writer, "Remove Firewall Rule");

    match rule.remove() {
        Ok(_) => writeln(writer, "Removed firewall rule successfully!"),
        Err(e) => {
            writeln(writer, &format!("Failed to remove firewall rule: {}", e));
            return CustomExitCode::FAILED_TO_REMOVE_FIREWALL_RULE;
        }
    }

    CustomExitCode::SUCCESS
}

#[cfg(target_family = "windows")]
pub fn firewall_rule_exists(writer: &mut Option<impl Write>) -> Result<bool, CustomExitCode> {
    let rule = match firewall_rule() {
        Some(rule) => rule,
        None => return Err(CustomExitCode::FAILED_TO_FIND_EXECUTABLE),
    };

    match rule.exists() {
        Ok(true) => {
            writeln(writer, "Firewall rule already exist");
            Ok(true)
        }
        Ok(false) => {
            writeln(writer, "Firewall rule does not exist");
            Ok(false)
        }
        Err(e) => {
            writeln(writer, &format!("Failed to check firewall rule: {}", e));
            Err(CustomExitCode::FAILED_TO_CHECK_FIREWALL_RULE)
        }
    }
}
