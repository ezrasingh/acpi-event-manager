use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use toml;

use crate::acpi;

// ? making changes to acpi requires root
pub fn sudo_check() {
    env::var("SUDO_USER").expect("This program must be run with sudo.");
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub acpi_device: String,
    pub xrandr_display: String,
    pub brightness_increment: i16,
    pub acpi_events: acpi::AcpiEventsConfig,
}

impl Config {
    // ? parse config file
    pub fn new(config_file: &str) -> Config {
        let mut file = match File::open(config_file) {
            Ok(file) => file,
            Err(error) => panic!("could not open config file: {:?}", error),
        };
        let mut config_contents = String::new();
        let _ = io::Read::read_to_string(&mut file, &mut config_contents);

        match toml::from_str(&config_contents) {
            Ok(config) => config,
            Err(error) => panic!("failed to parse config file: {:?}", error),
        }
    }

    // ? save generated acpi event handlers to system
    pub fn apply_config(&self, config_path: &Path) -> Result<(), std::io::Error> {
        let config_dir = config_path.to_str().unwrap();
        sudo_check();
        acpi::set_acpi_event_script(
            "acpi-keyboard-backlight",
            &format!("--action down --config-file {}", &config_dir),
            "keyboard-backlight-down",
            &self.acpi_events.brightness_down,
            None,
        )?;
        acpi::set_acpi_event_script(
            "acpi-keyboard-backlight",
            &format!("--action up --config-file {}", &config_dir),
            "keyboard-backlight-up",
            &self.acpi_events.brightness_up,
            None,
        )?;
        acpi::reload_acpi();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn parse_config() {
        let cwd = std::env::current_dir().unwrap();
        let fixtures_path = cwd.join("fixtures");
        let config_file = fixtures_path.join("config.toml");
        let config = Config::new(config_file.to_str().unwrap());

        assert_eq!(config.acpi_device, "acpi_device");
        assert_eq!(config.xrandr_display, "xrandr_display");
        assert_eq!(config.brightness_increment, 11);
        assert_eq!(config.acpi_events.brightness_up, "brightness_up");
        assert_eq!(config.acpi_events.brightness_down, "brightness_down");
    }
}
