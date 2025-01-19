use sysinfo::System;
use crate::utils::format_memory;

pub fn mem_info(sys: &System) {
    let total_memory = format_memory(sys.total_memory());
    let used_memory = format_memory(sys.used_memory());
    let free_memory = format_memory(sys.free_memory());
    let available_memory = format_memory(sys.available_memory());

    let total_swap = format_memory(sys.total_swap());
    let used_swap = format_memory(sys.used_swap());
    let free_swap = format_memory(sys.free_swap());
    println!("RAM usage: {} MB / {}  - Free: {} - Avaliable: {}", used_memory, total_memory, free_memory, available_memory);
    println!("SWAP usage: {} MB / {} - Free: {}", used_swap, total_swap, free_swap);
}
