use windows_firewall::FirewallRule;

pub struct NetworkFirewallRules {
    all: FirewallRules,
}

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

impl NetworkFirewallRules {
    pub fn add_or_update(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let inbound = self.all.add_or_update()?;
        Ok(inbound)
    }

    pub fn remove(self) -> Result<(), Box<dyn std::error::Error>> {
        self.all.remove()?;
        Ok(())
    }

    pub fn exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let inbound = self.all.exists()?;
        Ok(inbound)
    }
}

/// Only reason for returning `Option` is to handle `std::env::current_exe` error.
#[cfg(target_family = "windows")]
pub fn firewall_rules() -> Option<NetworkFirewallRules> {
    use std::{collections::HashSet, env::current_exe};
    use windows_firewall::{Direction, InterfaceType, Profile};

    let application_path = current_exe().ok()?;

    let base = |direction: Direction| {
        use windows_firewall::{Action, Protocol};

        let application_path = application_path.display().to_string();
        let description = "\
            Allows Volumize to communicate with devices on the local network (LAN).";

        let name = match direction {
            Direction::In => format!("{} (Inbound)", super::super::APPLICATION_NAME),
            Direction::Out => format!("{} (Outbound)", super::super::APPLICATION_NAME),
            Direction::Max => format!("{} (Max)", super::super::APPLICATION_NAME),
        };

        FirewallRule::builder()
            .name(name)
            .application_name(application_path)
            .grouping(super::super::APPLICATION_NAME)
            .description(description)
            .enabled(true)
            .action(Action::Allow)
            .protocol(Protocol::Tcp)
            .interface_types(HashSet::from([InterfaceType::Lan, InterfaceType::Wireless]))
            .direction(direction)
    };

    Some(NetworkFirewallRules {
        all: FirewallRules {
            inbound: base(Direction::In)
                .profiles(Profile::All)
                .local_ports([9002])
                .build(),
            outbound: base(Direction::Out).profiles(Profile::All).build(),
        },
    })
}
