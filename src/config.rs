use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io;
use toml;

use crate::acpi;

// ? making changes to acpi requires root
pub fn sudo_check() {
    env::var("SUDO_USER").expect("This program must be run with sudo.");
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub acpi_device: String,
    pub xrandr_display: String,
    pub brightness_increment: i16,
    pub acpi_events: acpi::AcpiEventsConfig,
}

impl Config {
    // ? parse config file
    pub fn new(config_file: &String) -> Config {
        let mut file = match File::open(config_file) {
            Ok(file) => file,
            Err(error) => panic!("failed to open config file: {:?}", error),
        };
        let mut config_contents = String::new();
        let _ = io::Read::read_to_string(&mut file, &mut config_contents);

        match toml::from_str(&config_contents) {
            Ok(config) => config,
            Err(error) => panic!("failed to parse config file: {:?}", error),
        }
    }

    // ? save generated acpi event handlers to system
    pub fn apply_config(&self, config_path: &str) {
        sudo_check();
        let _ = acpi::set_acpi_event_script(
            "acpi-keyboard-backlight",
            &format!("--action down --config-file {}", &config_path),
            "keyboard-backlight-down",
            &self.acpi_events.brightness_down,
        );
        let _ = acpi::set_acpi_event_script(
            "acpi-keyboard-backlight",
            &format!("--action up --config-file {}", &config_path),
            "keyboard-backlight-up",
            &self.acpi_events.brightness_up,
        );
        acpi::reload_acpi();
    }
}
