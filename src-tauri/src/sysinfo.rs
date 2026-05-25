use sysinfo::System;

pub fn get_system_info() -> String {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let os_name = sysinfo::System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let cpu_name = system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    let memory_mb = system.total_memory() / 1024 / 1024;

    format!("OS: {}\nCPU: {}\nMemory: {} MB", os_name, cpu_name, memory_mb)
}


pub fn get_app_list() -> String {
    let mut system = System::new_all();
    system.refresh_all();

    let mut names = Vec::new();

    for process in system.processes().values() {
        names.push(
            process.name().to_string_lossy().to_string()
        );
    }

    names.sort();
    names.dedup();

    format!(
        "Open apps:\n{}",
        names.join("\n")
    )
}