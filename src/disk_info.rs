use sysinfo::Disks;
use crate::utils::format_memory;

pub fn disk_info(disks: &Disks) {
    for disk in disks.list() {
        let used: u64 = disk.total_space() - disk.available_space();
        println!(
            "[{:?}] Type: {} Total: {} Used:{} Free:{} \n Usage: [
              Total Read Bytes: {}
              Read Bytes: {}
              Total Written Bytes: {}
              Written Bytes: {}

            ]",
            disk.name(),
            disk.kind(),
            format_memory(disk.total_space()),
            format_memory(used),
            format_memory(disk.available_space()),
            disk.usage().total_read_bytes,
            disk.usage().read_bytes,
            disk.usage().total_written_bytes,
            disk.usage().written_bytes
        );
    }

}
