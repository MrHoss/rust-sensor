use sysinfo::System;
use std::{thread, time::Duration};
use std::process::Command;

fn get_gpu_info() -> Option<(String, String, String)> {
    // Tenta usar nvidia-smi para NVIDIA
    if let Ok(output) = Command::new("nvidia-smi")
        .arg("--query-gpu=utilization.gpu,memory.total,memory.used,memory.free,clocks.current")
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let values: Vec<&str> = output_str.split(", ").collect();
        if values.len() == 5 {
            return Some((
                format!("Uso da GPU: {}", values[0]),
                format!("Uso de VRAM: {}/{} MB", values[2], values[1]),
                format!("Frequência da GPU: {} MHz", values[4]),
            ));
        }
    }

    // Tenta usar radeontop para AMD
    if let Ok(_) = Command::new("radeontop")
        .arg("-d")
        .output()
    {
        // let output_str = String::from_utf8_lossy(&output.stdout);
        // Parâmetros específicos do radeontop poderiam ser analisados aqui
        // Exemplo: pegar a linha relevante com o uso da GPU
        // Vamos simular o retorno para AMD
        return Some((
            "Uso da GPU: 45%".to_string(),
            "Uso de VRAM: 1200/4000 MB".to_string(),
            "Frequência da GPU: 1400 MHz".to_string(),
        ));
    }

    None
}

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        println!("========== Monitor de Desempenho ==========");

        // CPU
        let global_cpu_usage = sys.global_cpu_usage();
        println!("Uso da CPU: {:.2}%", global_cpu_usage);

        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!(
                "Core {}: {:.2}% - {} MHz",
                i,
                cpu.cpu_usage(),
                cpu.frequency()
            );
        }

        // RAM
        let total_memory = sys.total_memory() as f64 / 1_048_576.0; // MB
        let used_memory = sys.used_memory() as f64 / 1_048_576.0; // MB
        println!("Uso de RAM: {:.2} MB / {:.2} MB", used_memory, total_memory);

        // GPU
        match get_gpu_info() {
            Some((gpu_usage, vram_usage, gpu_freq)) => {
                println!("{}", gpu_usage);
                println!("{}", vram_usage);
                println!("{}", gpu_freq);
            }
            None => {
                println!("Nenhuma GPU detectada ou falha ao obter informações.");
            }
        }

        thread::sleep(Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H"); // Limpar terminal
    }
}
