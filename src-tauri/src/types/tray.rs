use std::{num::ParseFloatError, str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, Copy)]
pub enum Discovery {
    TurnOff,
    OnDuration(Duration),
    #[default]
    AlwaysOn,
}

impl Discovery {
    pub fn display(&self) -> String {
        match self {
            Discovery::TurnOff => String::from("Turn off"),
            Discovery::OnDuration(mins) => format!("On for {}s", mins.as_secs()),
            Discovery::AlwaysOn => String::from("Always on"),
        }
    }
}

impl std::fmt::Display for Discovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Discovery::TurnOff => write!(f, "turn_off"),
            Discovery::OnDuration(mins) => write!(f, "{}", mins.as_secs()),
            Discovery::AlwaysOn => write!(f, "always_on"),
        }
    }
}

impl FromStr for Discovery {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == Discovery::AlwaysOn.to_string() {
            return Ok(Discovery::AlwaysOn);
        }
        if s == Discovery::TurnOff.to_string() {
            return Ok(Discovery::TurnOff);
        }

        match s.parse::<f32>() {
            Ok(mins) => Ok(Discovery::OnDuration(Duration::from_secs_f32(mins))),
            Err(e) => Err(e),
        }
    }
}
