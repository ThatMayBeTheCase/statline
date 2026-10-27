use sysinfo::{Components, Disks, Networks, System};

pub struct SystemStats {
    pub cpu_usage: f32,
    pub used_ram: u64,
    pub total_ram: u64,
}

pub fn get_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        used_ram: sys.used_memory(),
        total_ram: sys.total_memory(),
    }
}
