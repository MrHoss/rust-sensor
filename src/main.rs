use sysinfo::{System};
use ocl::{Platform, Device, flags};
use std::{thread, time::Duration};

fn main() {
    let mut sys = System::new_all();
    
    loop {
        sys.refresh_all();
        println!("========== Monitor de Desempenho ==========");
        
        // CPU
        let global_cpu_usage = sys.global_cpu_usage();
        println!("Uso da CPU: {:.2}%", global_cpu_usage);
        
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("Core {}: {:.2}% - {} MHz", i, cpu.cpu_usage(), cpu.frequency());
        }

        // RAM
        let total_memory = sys.total_memory() as f64 / 1_048_576.0; // MB
        let used_memory = sys.used_memory() as f64 / 1_048_576.0;   // MB
        println!("Uso de RAM: {:.2} MB / {:.2} MB", used_memory, total_memory);

        // GPU (OpenCL - qualquer fabricante)
        println!("Tentando detectar GPU via OpenCL...");
        
        match Platform::first() {
            Ok(platform) => {
                // Obtendo dispositivos da plataforma
                let devices = Device::list(&platform, Some(flags::DEVICE_TYPE_GPU)).unwrap_or_else(|_| vec![]);
                
                if !devices.is_empty() {
                    for device in devices {                        
                         match device.info(ocl::enums::DeviceInfo::GlobalMemSize) {
                            Ok(mem_size) => println!("Memória global da GPU: {} bytes", mem_size),
                            Err(e) => println!("Erro ao obter memória global: {}", e),
                        }

                        match device.info(ocl::enums::DeviceInfo::Name) {
                            Ok(name) => println!("Nome do dispositivo: {}", name),
                            Err(e) => println!("Erro ao obter nome do dispositivo: {}", e),
                        }
                        
                        // Aqui você pode tentar obter outras informações sobre a GPU, como temperatura, se possível
                    }
                } else {
                    println!("Nenhuma GPU detectada.");
                }
            }
            Err(e) => println!("Erro ao inicializar OpenCL: {:?}", e),
        }

        thread::sleep(Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H"); // Limpar terminal
    }
}
