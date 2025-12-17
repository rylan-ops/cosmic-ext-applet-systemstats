use sysinfo::System;

pub struct CpuStats {
    system: System,
}

impl CpuStats {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        // Initialize CPU usage tracking
        system.global_cpu_usage();

        Self { system }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu_usage();
    }

    pub fn usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }
}
