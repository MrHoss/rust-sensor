use sysinfo::{Component, Components, System};

pub fn cpu_info(sys: &System, components: &Components) {
    let global_cpu_usage: f32 = sys.global_cpu_usage();
    println!(
        "CPU usage: {:.2}% - {}",
        global_cpu_usage,
        sys.cpus()[0].brand()
    );

    // Filtrando somente os componentes que começam com "Core"
    let mut coretemps: Vec<_> = components
        .iter()
        .filter(|component: &&Component| component.label().starts_with("Core"))
        .collect();
    coretemps.sort_by_key(|component| component.label().to_string());

    // Exibindo o uso e temperatura dos núcleos
    for (i, cpu) in sys.cpus().iter().enumerate() {
        // Encontra o componente de temperatura correspondente ao núcleo
        if let Some(core) = coretemps.get(i) {
            let temperature: f32 = core.temperature().unwrap_or(f32::NAN); // Caso a temperatura não seja encontrada, use NaN
            let overheat = if temperature > core.critical().unwrap_or(100.0) {
                "\x1b[31m !!! OVERHEAT !!!\x1b[0m"
            } else if temperature > 70.0 {
                "\x1b[33m !!! HEAT !!!\x1b[0m"
            } else {
                ""
            };
            println!(
                " - {}: {:.2}% - {} MHz, Temp: {:.1}ºC{}",
                core.label(),
                cpu.cpu_usage(),
                cpu.frequency(),
                temperature,
                overheat
            );
        }
    }
}
