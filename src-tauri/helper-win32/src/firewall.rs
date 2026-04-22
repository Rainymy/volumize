use std::path::PathBuf;

use super::embedded;
use super::sign;

pub fn setup_firewall() -> Result<(), String> {
    if !is_private_network() {
        return Ok(());
    }

    if exists_firewall_rule() {
        return Ok(());
    }

    add_firewall_rule()
}

fn is_private_network() -> bool {
    use windows::Win32::Networking::NetworkListManager::{
        INetwork, INetworkListManager, NetworkListManager, NLM_ENUM_NETWORK_CONNECTED,
        NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED, NLM_NETWORK_CATEGORY_PRIVATE,
        NLM_NETWORK_CATEGORY_PUBLIC,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

    let nlm: INetworkListManager = unsafe {
        match CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL) {
            Ok(v) => v,
            Err(_) => return false,
        }
    };

    // Early exit if not connected
    let is_connected = unsafe { nlm.IsConnected().map(|v| v.as_bool()).unwrap_or(false) };
    if !is_connected {
        return false;
    }

    let network_enumator = unsafe {
        match nlm.GetNetworks(NLM_ENUM_NETWORK_CONNECTED) {
            Ok(connections) => connections,
            Err(_) => return false,
        }
    };

    let network = unsafe {
        let mut networks = [None::<INetwork>; 1];
        let mut pceltfetched = 0u32;

        match network_enumator.Next(&mut networks, Some(&mut pceltfetched)) {
            Ok(_) if pceltfetched > 0 => {}
            _ => return false,
        }
        match networks[0].take() {
            Some(net) => net,
            None => return false,
        }
    };

    match unsafe { network.GetCategory() } {
        Ok(NLM_NETWORK_CATEGORY_PRIVATE) => true,
        // I'm not sure what the domain authenticated category means,
        // so treat it as private.
        Ok(NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED) => true,
        Ok(NLM_NETWORK_CATEGORY_PUBLIC) | _ => false,
    }
}

#[allow(dead_code)]
pub enum FireWallEnum {
    ADD,
    REMOVE,
    EXISTS,
}

impl FireWallEnum {
    fn as_str(&self) -> &'static str {
        match self {
            FireWallEnum::ADD => "--add-rule",
            FireWallEnum::REMOVE => "--remove-rule",
            FireWallEnum::EXISTS => "--exist",
        }
    }
}

#[allow(dead_code)]
struct FireWallWindows {}

impl FireWallWindows {
    #[allow(dead_code)]
    fn execute_helper(&self, operation: FireWallEnum) -> Result<(), String> {
        let path = embedded::extract_helper().unwrap();
        embedded::elevate_helper(&path, operation.as_str())
    }
}

fn embed_validator(path: &PathBuf) -> Result<(), String> {
    if !sign::verify_hash(path) {
        return Err("Hash mismatch".to_string());
    }
    sign::verify_signature(path)?;
    Ok(())
}

pub fn add_firewall_rule() -> Result<(), String> {
    let path = embedded::extract_helper()?;
    embed_validator(&path)?;
    embedded::elevate_helper(&path, "--add-rule")?;
    Ok(())
}

#[allow(dead_code)]
pub fn remove_firewall_rule() -> Result<(), String> {
    let path = embedded::extract_helper()?;
    embed_validator(&path)?;
    embedded::elevate_helper(&path, "--remove-rule")?;
    Ok(())
}

pub fn exists_firewall_rule() -> bool {
    if let Ok(path) = embedded::extract_helper() {
        if embed_validator(&path).is_ok() {
            return embedded::elevate_helper(&path, "--exist").is_ok();
        }
    }
    false
}
