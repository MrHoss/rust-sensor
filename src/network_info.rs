use crate::utils::format_memory;
use sysinfo::{Networks, NetworkData};

pub fn network_info(networks: &Networks) {
    for (interface_name, network) in networks {
        for ip in network.ip_networks(){
          
        }
        println!(
          "[{:?}] IP: {:?}{}",
          interface_name,
          network.ip_networks(),
          network.total_received(),
      );
    }
}
