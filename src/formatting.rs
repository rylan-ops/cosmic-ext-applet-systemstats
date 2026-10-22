//! Formatting utilities for system stats display

pub fn format_percentage(value: f32) -> String {
    format!("{:.0}%", value.clamp(0.0, 100.0))
}

pub fn format_celsius(value: f32) -> String {
    format!("{:.0}°C", value)
}

pub fn format_memory_gb(value: f32) -> String {
    format!("{:.1} GB", value)
}

/// Shows Kbps below 1 Mbps and auto-switches to Gbps when >= 1000 Mbps
pub fn format_network_speed(bytes_per_sec: u64) -> String {
    let kbps = bytes_per_sec as f64 / 125.0;
    let mbps = kbps / 1000.0;

    // Thresholds account for rounding so we never show "1000 Kbps" or "1000.0 Mbps"
    if kbps < 999.5 {
        format!("{:.0} Kbps", kbps)
    } else if mbps < 999.95 {
        format!("{:.1} Mbps", mbps)
    } else {
        format!("{:.2} Gbps", mbps / 1000.0)
    }
}
