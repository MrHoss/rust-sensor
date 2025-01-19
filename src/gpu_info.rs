use sysinfo::{Component, Components, System};
use std::process::Command;

pub fn gpu_info(sys: &System, components: &Components) {
    // Filtrando componentes relacionados à GPU
    let gpus: Vec<_> = components
        .iter()
        .filter(|component: &&Component| component.label().contains("edge"))
        .collect();

    // Iterando sobre os componentes de GPU
    for (index, component) in gpus.iter().enumerate() {
        // Obtendo a temperatura
        let temperature: f32 = component.temperature().unwrap_or(f32::NAN);

        // Verificando se há superaquecimento
        let overheat = if temperature > component.critical().unwrap_or(100.0) {
            "\x1b[31m !!! OVERHEAT !!!\x1b[0m"
        } else if temperature > 70.0 {
            "\x1b[33m !!! HEAT !!!\x1b[0m"
        } else {
            ""
        };

        // Obtendo informações sobre o uso de GPU e VRAM
        match get_gpu_info() {
            Some((gpu_usage, vram_usage, gpu_freq)) => {
                println!(
                    "{} {}: Usage: {} | VRAM: {} | Freq: {} MHz | Temp: {:.1}ºC{}",
                    component.label(),
                    index,
                    gpu_usage,
                    vram_usage,
                    gpu_freq,
                    temperature,
                    overheat
                );
            }
            None => {
                println!(
                    "{} {}: Temp: {:.1}ºC{}",
                    component.label(),
                    index,
                    temperature,
                    overheat
                );
            }
        }
    }
}

// Função auxiliar para pegar informações sobre a GPU (NVIDIA ou AMD)
fn get_gpu_info() -> Option<(String, String, String)> {
    // Tentando usar nvidia-smi para NVIDIA
    if let Ok(output) = Command::new("nvidia-smi")
        .arg("--query-gpu=utilization.gpu,memory.total,memory.used,memory.free,clocks.current")
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let values: Vec<&str> = output_str.split(", ").collect();
        if values.len() == 5 {
            return Some((
                format!("GPU Usage: {}", values[0]),
                format!("VRAM Usage: {}/{} MB", values[2], values[1]),
                format!("GPU Frequency: {} MHz", values[4]),
            ));
        }
    }

    // Tentando usar radeontop para AMD
    if let Ok(output) = Command::new("radeontop")
        .arg("-d")
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Aqui vamos processar a saída do radeontop
        let lines: Vec<&str> = output_str.lines().collect();

        // Inicializando variáveis para armazenar as informações
        let mut gpu_usage = String::new();
        let mut vram_usage = String::new();
        let mut gpu_freq = String::new();

        for line in lines {
            // Extrai o uso da GPU da linha que contém "Graphics pipe"
            if line.contains("Graphics pipe") {
                if let Some(pos) = line.find('%') {
                    gpu_usage = line[..pos].trim().to_string();
                }
            }

            // Extrai o uso de VRAM da linha que contém "VRAM"
            if line.contains("VRAM") {
                if let Some(pos) = line.find("VRAM") {
                    vram_usage = line[pos..].trim().to_string();
                }
            }

            // Extrai a frequência de memória da linha que contém "Memory Clock"
            if line.contains("Memory Clock") {
                if let Some(pos) = line.find("G") {
                    gpu_freq = line[..pos].trim().to_string();
                }
            }
        }

        // Retorna as informações obtidas
        if !gpu_usage.is_empty() && !vram_usage.is_empty() && !gpu_freq.is_empty() {
            return Some((
                format!("GPU Usage: {}", gpu_usage),
                format!("VRAM Usage: {}", vram_usage),
                format!("GPU Frequency: {}", gpu_freq),
            ));
        }
    }

    None
}
