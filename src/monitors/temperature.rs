use sysinfo::Components;

pub struct TemperatureStats {
    components: Components,
}

impl TemperatureStats {
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();
        Self { components }
    }

    pub fn update(&mut self) {
        self.components.refresh(false);
    }

    pub fn cpu_celsius(&self) -> Option<f32> {
        // Search for CPU temperature sensor
        // Not cached to support hot-plug sensors
        self.components.iter().find_map(|component| {
            let label = component.label().to_lowercase();
            // Match common CPU temperature sensor names
            if label.contains("cpu")
                || label.contains("tdie")
                || label.contains("tctl")
                || label.starts_with("core")
            {
                component.temperature()
            } else {
                None
            }
        })
    }
}
