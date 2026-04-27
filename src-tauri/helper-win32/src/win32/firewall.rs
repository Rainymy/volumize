use windows_firewall::FirewallRule;

pub struct FirewallRules {
    inbound: FirewallRule,
    outbound: FirewallRule,
}

impl FirewallRules {
    pub fn add_or_update(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let inbound = self.inbound.add_or_update()?;
        let outbound = self.outbound.add_or_update()?;
        Ok(inbound && outbound)
    }

    pub fn remove(self) -> Result<(), Box<dyn std::error::Error>> {
        self.inbound.remove()?;
        self.outbound.remove()?;
        Ok(())
    }

    pub fn exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let inbound = self.inbound.exists()?;
        let outbound = self.outbound.exists()?;
        Ok(inbound && outbound)
    }
}

/// Only reason for returning `Option` is to handle `std::env::current_exe` error.
#[cfg(target_family = "windows")]
pub fn firewall_rules() -> Option<FirewallRules> {
    use std::env::current_exe;
    use windows_firewall::Direction;

    let application_path = current_exe().ok()?.display().to_string();

    let base = |direction: Direction| {
        use windows_firewall::{Action, Profile, Protocol};

        let description = "\
            Volumize (private network) enables LAN traffic between local devices\
        ";

        let name = match direction {
            Direction::In => format!("{} (Inbound)", super::super::APPLICATION_NAME),
            Direction::Out => format!("{} (Outbound)", super::super::APPLICATION_NAME),
            Direction::Max => format!("{} (Max)", super::super::APPLICATION_NAME),
        };

        FirewallRule::builder()
            .name(name)
            .application_name(&application_path)
            .grouping(super::super::APPLICATION_NAME)
            .description(description)
            .enabled(true)
            .action(Action::Allow)
            .profiles(Profile::Private)
            .protocol(Protocol::Tcp)
            .direction(direction)
    };

    Some(FirewallRules {
        inbound: base(Direction::In).local_ports([9002]).build(),
        outbound: base(Direction::Out).build(),
    })
}
