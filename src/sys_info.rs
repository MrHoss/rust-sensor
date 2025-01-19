use sysinfo::System;

pub fn system_info() {

    // Obter o nome do sistema operacional
    let os_name: String = System::name().unwrap_or_else(|| "Desconhecido".to_string());
    println!("Sistema Operacional: {}", os_name);

    // Obter a versão do sistema operacional
    let os_version: String = System::os_version().unwrap_or_else(|| "Desconhecido".to_string());
    println!("Versão do SO: {}", os_version);

    // Obter a versão do kernel
    let kernel_version: String = System::kernel_version().unwrap_or_else(|| "Desconhecido".to_string());
    println!("Kernel: {}", kernel_version);

    // Obter informações adicionais, se necessário
    let hostname: String = System::host_name().unwrap_or_else(|| "Desconhecido".to_string());
    println!("Hostname: {}", hostname);
}