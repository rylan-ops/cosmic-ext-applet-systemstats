use std::path::Path;
use std::time::Instant;
use sysinfo::Networks;

pub struct NetworkStats {
    networks: Networks,
    last_update: Instant,
    rx_bytes_per_sec: u64,
    tx_bytes_per_sec: u64,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            last_update: Instant::now(),
            rx_bytes_per_sec: 0,
            tx_bytes_per_sec: 0,
        }
    }

    pub fn update(&mut self) {
        self.networks.refresh(true);

        let elapsed = self.last_update.elapsed().as_secs_f64();
        self.last_update = Instant::now();

        let (mut rx, mut tx) = (0u64, 0u64);
        for (interface_name, data) in &self.networks {
            if is_physical_interface(interface_name) {
                rx += data.received();
                tx += data.transmitted();
            }
        }

        if elapsed > 0.0 {
            self.rx_bytes_per_sec = (rx as f64 / elapsed) as u64;
            self.tx_bytes_per_sec = (tx as f64 / elapsed) as u64;
        }
    }

    pub fn download_bps(&self) -> u64 {
        self.rx_bytes_per_sec
    }

    pub fn upload_bps(&self) -> u64 {
        self.tx_bytes_per_sec
    }
}

/// Virtual interfaces (VPN, docker) route through a physical one, so counting them would double traffic
fn is_physical_interface(name: &str) -> bool {
    Path::new("/sys/class/net").join(name).join("device").exists()
}
