use std::path::Path;

use crate::acpi;
use crate::config::Config;

#[derive(Debug)]
pub struct BacklightConfig {
    max: i16,
    brightness: i16,
}

impl BacklightConfig {
    // ? determine current backlight config frmo system files
    pub fn from_config(config: &Config) -> BacklightConfig {
        let device_config_dir = Path::new("/sys/class/backlight").join(&config.acpi_device);
        let max_brightness = acpi::read_acpi_config_value(device_config_dir.join("max_brightness"));
        let current_brightness = acpi::read_acpi_config_value(device_config_dir.join("brightness"));
        BacklightConfig {
            max: max_brightness,
            brightness: current_brightness,
        }
    }

    // ? calulate current brightness as ratio
    pub fn percentage(&self) -> String {
        let ratio = f32::from(self.brightness) / f32::from(self.max);
        format!("{:.3}", ratio)
    }

    // ? modify brigtness value and call xrandr on system
    pub fn change_brightness(&mut self, display_target: &str, increment: i16) {
        let value = self.brightness + increment;
        if value > self.max {
            self.brightness = self.max
        } else if value <= 0 {
            self.brightness = 1
        } else {
            self.brightness = value
        }
        std::process::Command::new("xrandr")
            .args(&[
                "--output",
                display_target,
                "--brightness",
                &self.percentage(),
            ])
            .output()
            .expect("could not change brightness");
    }

    // ? store brightness values in acpi config
    pub fn save(&self, acpi_device: &str) -> Result<(), std::io::Error> {
        acpi::set_acpi_config_value(acpi_device, "brightness", &self.brightness.to_string())
    }
}
