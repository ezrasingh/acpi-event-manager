use std::{
    fs::File,
    io::Write,
    num::ParseIntError,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{backlight, config};

#[derive(Debug, Deserialize, Clone)]
pub struct AcpiEventsConfig {
    pub brightness_up: String,
    pub brightness_down: String,
}

#[derive(Debug, Clone)]
pub enum AcpiEventAction {
    BrightnessUp,
    BrightnessDown,
}

impl std::str::FromStr for AcpiEventAction {
    type Err = String;
    // ? parse string into event action
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(AcpiEventAction::BrightnessUp),
            "down" => Ok(AcpiEventAction::BrightnessDown),
            _ => Err("Invalid operation".to_string()),
        }
    }
}

// ? configures value of acpi event handler
pub fn set_acpi_event_script(
    script_path: &str,
    script_args: &str,
    event_name: &str,
    event_code: &str,
    acpi_events_path: Option<&Path>,
) -> Result<(), std::io::Error> {
    let acpi_events_dir = acpi_events_path.unwrap_or(Path::new("/etc/acpi/events"));
    let mut event_script = File::create(acpi_events_dir.join(event_name))?;

    let content = format!(
        "event={}\naction={} {}\n",
        event_code, script_path, script_args
    );
    event_script.write_all(content.as_bytes())
}

// ? reloads acpi daemon
pub fn reload_acpi() {
    let output = std::process::Command::new("/etc/init.d/acpid")
        .arg("reload")
        .output()
        .expect("Failed to reload acpi daemon");
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

// ? read value from acpi config file
pub fn read_acpi_config_value(acpi_config_file: PathBuf) -> Result<i16, ParseIntError> {
    let mut acpi_config = match File::open(acpi_config_file) {
        Ok(file) => file,
        Err(error) => panic!("Failed to open acpi config file: {:?}", error),
    };
    let mut acpi_config_content = String::new();
    match std::io::Read::read_to_string(&mut acpi_config, &mut acpi_config_content) {
        Ok(_) => acpi_config_content.trim().parse(),
        Err(error) => panic!("Could not read ACPI config ({:?})", error),
    }
}

// ? set value in acpi config file
fn set_acpi_config_value(
    acpi_device: &str,
    acpi_field: &str,
    acpi_value: &str,
) -> Result<(), std::io::Error> {
    let device_config_dir = Path::new("/sys/class/backlight")
        .join(&acpi_device)
        .join(&acpi_field);
    let mut acpi_config = File::create(device_config_dir)?;
    acpi_config.write_all(&acpi_value.as_bytes())
}

// ? store acpi event handlers
pub fn save(config: config::Config, backlight: backlight::BacklightConfig) {
    set_acpi_config_value(
        &config.acpi_device,
        "brightness",
        &backlight.brightness.to_string(),
    )
    .expect("could not increase brightness");

    std::process::Command::new("xrandr")
        .args(&[
            "--output",
            &config.xrandr_display,
            "--brightness",
            &backlight.percentage(),
        ])
        .output()
        .expect("could not change brightness");
}

mod tests {

    #[test]
    fn set_acpi_event() {
        let cwd = std::env::current_dir().unwrap();
        let events_path = cwd.join("fixtures").join("events");
        let result = crate::acpi::set_acpi_event_script(
            "script_path",
            &format!("script_arg"),
            "event_name",
            "event_code",
            Some(&events_path),
        );

        assert!(result.is_ok());
        assert!(events_path.join("event_name").exists());

        let event_config = std::fs::read_to_string("fixtures/events/event_name");
        assert_eq!(
            event_config.unwrap(),
            "event=event_code\naction=script_path script_arg\n"
        );
    }
}
