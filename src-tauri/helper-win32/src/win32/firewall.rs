use windows_firewall::FirewallRule;

pub struct NetworkFirewallRules {
    public: FirewallRules,
    private: FirewallRules,
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
        let inbound = self.private.add_or_update()?;
        let outbound = self.public.add_or_update()?;
        Ok(inbound && outbound)
    }

    pub fn remove(self) -> Result<(), Box<dyn std::error::Error>> {
        self.private.remove()?;
        self.public.remove()?;
        Ok(())
    }

    pub fn exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let inbound = self.private.exists()?;
        let outbound = self.public.exists()?;
        Ok(inbound && outbound)
    }
}

/// Only reason for returning `Option` is to handle `std::env::current_exe` error.
#[cfg(target_family = "windows")]
pub fn firewall_rules() -> Option<NetworkFirewallRules> {
    use std::env::current_dir;
    use windows_firewall::{Direction, Profile};

    let mut application_path = current_dir().ok()?;
    application_path.push(super::super::APPLICATION_EXE);

    let base = |direction: Direction| {
        use windows_firewall::{Action, Protocol};

        let application_path = application_path.display().to_string();
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
            .application_name(application_path)
            .grouping(super::super::APPLICATION_NAME)
            .description(description)
            .enabled(true)
            .action(Action::Allow)
            .protocol(Protocol::Tcp)
            .direction(direction)
    };

    Some(NetworkFirewallRules {
        private: FirewallRules {
            inbound: base(Direction::In)
                .profiles(Profile::Private)
                .local_ports([9002])
                .build(),
            outbound: base(Direction::Out).profiles(Profile::Private).build(),
        },
        public: FirewallRules {
            inbound: base(Direction::In)
                .profiles(Profile::Public)
                .local_ports([9002])
                .build(),
            outbound: base(Direction::Out).profiles(Profile::Public).build(),
        },
    })
}
