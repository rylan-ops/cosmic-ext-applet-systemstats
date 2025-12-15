// TODO: Implement temperature monitoring
// CPU: /sys/class/thermal/thermal_zone*/temp
// GPU: /sys/class/hwmon/hwmon*/temp*_input or nvml-wrapper

// pub struct TemperatureStats {
//     cpu_temp: Option<f32>,
//     gpu_temp: Option<f32>,
// }
//
// impl TemperatureStats {
//     pub fn new() -> Self {
//         todo!()
//     }
//
//     pub fn update(&mut self) {
//         todo!()
//     }
//
//     pub fn cpu_celsius(&self) -> Option<f32> {
//         self.cpu_temp
//     }
//
//     pub fn gpu_celsius(&self) -> Option<f32> {
//         self.gpu_temp
//     }
// }
