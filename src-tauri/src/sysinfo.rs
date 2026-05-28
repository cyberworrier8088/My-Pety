use sysinfo::System; // import sysinfo module

pub fn get_system_info() -> String { // get system info func
    let mut system = sysinfo::System::new_all(); // create new system
    system.refresh_all(); // refresh all

    let os_name = sysinfo::System::name().unwrap_or_else(|| "Unknown OS".to_string()); // get os name
    let cpu_name = system.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string()); // get cpu name
    let memory_mb = system.total_memory() / 1024 / 1024; // get memory in mb

    format!("OS: {}\nCPU: {}\nMemory: {} MB", os_name, cpu_name, memory_mb) // format output
}


pub fn get_app_list() -> String { // get app list func
    let mut system = System::new_all(); // create new system
    system.refresh_all(); // refresh all

    let mut names = Vec::new(); // create new vector

    for process in system.processes().values() { // loop through processes
        names.push(
            process.name().to_string_lossy().to_string() // get process name
        );
    }

    names.sort(); // sort names
    names.dedup(); // remove duplicates

    format!("Open apps:\n{}", names.join("\n")) // format output
}
