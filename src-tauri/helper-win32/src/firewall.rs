#![allow(dead_code)]
use super::formatter::writeln;
use super::my_exit_code::CustomExitCode;
use std::io::Write;

use windows_firewall::{Action, Direction, FirewallRule, Profile, Protocol};

// Try to get name/port/application from build.rs env, fallback to a hardcoded value
//
// const RULE_NAME: &str = env!("RULE_NAME");
// const RULE_NAME: &str = "Volumize";

pub fn firewall_rule() -> FirewallRule {
    FirewallRule::builder()
        .name("Volumize")
        .application_name("Volumize")
        .description("Volumize firewall rule")
        .enabled(true)
        .action(Action::Allow)
        .profiles(Profile::Private)
        .protocol(Protocol::Tcp)
        .direction(Direction::In)
        .local_ports([9002])
        .build()
}

pub fn firewall_rule_add_or_update(writer: &mut Option<impl Write>) -> CustomExitCode {
    let rule = firewall_rule();

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

pub fn firewall_rule_remove(writer: &mut Option<impl Write>) -> CustomExitCode {
    let rule = firewall_rule();
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

pub fn firewall_rule_exists(writer: &mut Option<impl Write>) -> Result<bool, ()> {
    let rule = firewall_rule();

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
            Err(())
        }
    }
}
