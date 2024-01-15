use clap::Parser;
use std::path::Path;

pub mod acpi;
pub mod backlight;
pub mod config;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Installs acpi events and reloads acpi daemon
    #[clap(long)]
    configure: bool,

    /// Action from acpi event
    #[clap(short, long)]
    action: Option<acpi::AcpiEventAction>,

    /// Absolute path to config file
    #[arg(short, long, default_value = "config.toml")]
    config_file: String,

    /// Log config details
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let config = config::Config::new(&args.config_file);
    let mut backlight_config = backlight::BacklightConfig::from_config(&config);
    if args.debug {
        println!("{:?}", &args);
        println!("{:?}", &config);
        println!("{:?}", &backlight_config);
        println!("{}%", &backlight_config.percentage());
    }
    if args.action.is_some() {
        config::sudo_check();
        match args.action {
            Some(acpi::AcpiEventAction::BrightnessUp) => {
                backlight_config
                    .change_brightness(&config.xrandr_display, 1 * config.brightness_increment);
                backlight_config
                    .save(&config.acpi_device)
                    .expect("could not increase brightness");
            }
            Some(acpi::AcpiEventAction::BrightnessDown) => {
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
