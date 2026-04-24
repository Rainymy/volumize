#[allow(dead_code)]
pub fn is_private_network() -> bool {
    use windows::Win32::Networking::NetworkListManager::{
        INetwork, INetworkListManager, NLM_ENUM_NETWORK_CONNECTED,
        NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED, NLM_NETWORK_CATEGORY_PRIVATE,
        NLM_NETWORK_CATEGORY_PUBLIC, NetworkListManager,
    };
    use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};

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
