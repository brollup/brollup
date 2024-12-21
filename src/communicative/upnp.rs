use crate::baked;
use easy_upnp::{add_ports, PortMappingProtocol, UpnpConfig};

pub async fn open_port() -> bool {
    let upnp_config = UpnpConfig {
        address: None,
        port: baked::PORT,
        protocol: PortMappingProtocol::TCP,
        duration: 100_000_000,
        comment: baked::PROJECT_TAG.to_string() + " " + "P2P Protocol",
    };

    for result in add_ports([upnp_config]) {
        if let Ok(_) = result {
            return true;
        }
    }

    false
}
