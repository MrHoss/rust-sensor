mod cpu_info;
mod disk_info;
mod gpu_info;
mod mem_info;
mod network_info;
mod utils;
mod sys_info;
use async_std::net;
use cpu_info::cpu_info;
use disk_info::disk_info;
use gpu_info::gpu_info;
use mem_info::mem_info;
use network_info::network_info;
use sys_info::system_info;
use utils::clear_terminal;

use std::{thread, time::Duration};
use sysinfo::{
    Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System,
};

#[async_std::main]
async fn main() {
    clear_terminal();
    let mut sys: System = System::new_all();
    let mut components: Components = Components::new_with_refreshed_list();
    let mut disks: Disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();
    loop {
        println!("SYSTEM  ===================================");
        system_info();
        println!("CPU  ======================================");
        cpu_info(&sys, &components);
        println!("MEMORY  ===================================");
        mem_info(&sys);
        println!("GPU  ======================================");
        gpu_info(&sys, &components);
        println!("DISK  =====================================");
        disk_info(&disks);
        println!("NETWORK  ==================================");
        network_info(&networks);

        sys.refresh_all();
        components.refresh(true);
        disks.refresh(true);
        networks.refresh(true);
        thread::sleep(Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H"); // Limpar terminal
    }
}
