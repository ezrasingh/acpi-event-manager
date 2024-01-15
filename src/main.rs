use std::{
    env,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct AcpiEventsConfig {
    brightness_up: String,
    brightness_down: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    acpi_device: String,
    xrandr_display: String,
    brightness_increment: i16,
    acpi_events: AcpiEventsConfig,
}

fn set_acpi_event_script(
    script_path: &str,
    script_args: &str,
    event_name: &str,
    event_code: &str,
) -> Result<(), std::io::Error> {
    let acpi_events_dir = Path::new("/etc/acpi/events");
    let mut event_script = File::create(acpi_events_dir.join(event_name))?;

    let content = format!(
        "event={}\naction={} {}\n",
        event_code, script_path, script_args
    );
    event_script.write_all(content.as_bytes())
}

fn reload_acpi() {
    let output = std::process::Command::new("/etc/init.d/acpid")
        .arg("reload")
        .output()
        .expect("failed to reload acpi daemon");
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

fn sudo_check() {
    env::var("SUDO_USER").expect("This program must be run with sudo.");
}

impl Config {
    fn new(config_file: &String) -> Config {
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
    fn apply_config(&self, config_path: &str) {
        sudo_check();
        let _ = set_acpi_event_script(
            "acpi-keyboard-backlight",
            &format!("--action down --config-file {}", &config_path),
            "keyboard-backlight-down",
            &self.acpi_events.brightness_down,
        );
        let _ = set_acpi_event_script(
            "acpi-keyboard-backlight",
            &format!("--action up --config-file {}", &config_path),
            "keyboard-backlight-up",
            &self.acpi_events.brightness_up,
        );
        reload_acpi();
    }
}

#[derive(Debug)]
struct BacklightConfig {
    max: i16,
    brightness: i16,
}

fn read_acpi_config_value(acpi_config_file: PathBuf) -> i16 {
    let mut acpi_config = match File::open(acpi_config_file) {
        Ok(file) => file,
        Err(error) => panic!("Failed to open acpi config file: {:?}", error),
    };
    let mut acpi_config_content = String::new();
    let _ = io::Read::read_to_string(&mut acpi_config, &mut acpi_config_content);
    acpi_config_content.trim().parse::<i16>().unwrap()
}

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

impl BacklightConfig {
    fn from_config(config: &Config) -> BacklightConfig {
        let device_config_dir = Path::new("/sys/class/backlight").join(&config.acpi_device);
        let max_brightness = read_acpi_config_value(device_config_dir.join("max_brightness"));
        let current_brightness = read_acpi_config_value(device_config_dir.join("brightness"));
        BacklightConfig {
            max: max_brightness,
            brightness: current_brightness,
        }
    }
    fn percentage(&self) -> String {
        let ratio = f32::from(self.brightness) / f32::from(self.max);
        format!("{:.3}", ratio)
    }
    fn change_brightness(&mut self, display_target: &str, increment: i16) {
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
    fn save(&self, acpi_device: &str) -> Result<(), std::io::Error> {
        set_acpi_config_value(acpi_device, "brightness", &self.brightness.to_string())
    }
}

#[derive(Debug, Clone)]
enum AcpiEventAction {
    BrightnessUp,
    BrightnessDown,
}

impl std::str::FromStr for AcpiEventAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(AcpiEventAction::BrightnessUp),
            "down" => Ok(AcpiEventAction::BrightnessDown),
            _ => Err("Invalid operation".to_string()),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Installs acpi events and reloads acpi daemon
    #[clap(long)]
    configure: bool,

    /// Action from acpi event
    #[clap(short, long)]
    action: Option<AcpiEventAction>,

    /// Absolute path to config file
    #[arg(short, long, default_value = "config.toml")]
    config_file: String,

    /// Log config details
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let config = Config::new(&args.config_file);
    let mut backlight_config = BacklightConfig::from_config(&config);
    if args.debug {
        println!("{:?}", &args);
        println!("{:?}", &config);
        println!("{:?}", &backlight_config);
        println!("{}%", &backlight_config.percentage());
    }
    if args.action.is_some() {
        sudo_check();
        match args.action {
            Some(AcpiEventAction::BrightnessUp) => {
                backlight_config
                    .change_brightness(&config.xrandr_display, 1 * config.brightness_increment);
                backlight_config
                    .save(&config.acpi_device)
                    .expect("could not increase brightness");
            }
            Some(AcpiEventAction::BrightnessDown) => {
                backlight_config
                    .change_brightness(&config.xrandr_display, -1 * config.brightness_increment);
                backlight_config
                    .save(&config.acpi_device)
                    .expect("could not decrease brightness");
            }
            None => {}
        }
    }
    if args.configure {
        let config_path = Path::new(&args.config_file);
        config.apply_config(config_path.to_str().unwrap());
    }
}
